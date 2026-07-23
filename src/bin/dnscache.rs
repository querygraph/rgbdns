use hickory_server::{
    Server,
    net::runtime::TokioRuntimeProvider,
    proto::rr::{LowerName, Name},
    resolver::{
        config::{NameServerConfig, ResolverOpts},
        recursor::{DnssecConfig, DnssecPolicyConfig, RecursiveConfig, RecursorOptions},
    },
    store::{
        forwarder::{ForwardConfig, ForwardZoneHandler},
        recursor::RecursiveZoneHandler,
    },
    zone_handler::{Catalog, ZoneHandler, ZoneType},
};
use ipnet::IpNet;
use rgbdns::dnscache_config::{PreparedRoots, forward_zones_from_environment};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::{TcpListener, UdpSocket};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .with_target(false)
        .compact()
        .init();
    if let Err(error) = run().await {
        eprintln!("dnscache: fatal: {error}");
        std::process::exit(111);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "53".into());
    let address: SocketAddr = format!("{ip}:{port}").parse()?;
    let roots = PreparedRoots::from_environment()?;

    let options = RecursorOptions {
        case_randomization: true,
        response_cache_size: bounded_env_usize(
            "CACHESIZE",
            16 * 1024 * 1024,
            1024,
            1024 * 1024 * 1024,
        )? as u64,
        ns_cache_size: bounded_env_usize("NSCACHESIZE", 4096, 16, 1_048_576)?,
        recursion_limit: bounded_env_u8("RECURSION_LIMIT", 64, 1, 128)?,
        ns_recursion_limit: bounded_env_u8("NS_RECURSION_LIMIT", 32, 1, 128)?,
        edns_payload_len: 1232,
        ..Default::default()
    };

    let config = RecursiveConfig {
        roots: roots.path().to_owned(),
        dnssec_policy: DnssecPolicyConfig::ValidateWithStaticKey {
            path: None,
            nsec3_soft_iteration_limit: DnssecConfig::default().nsec3_soft_iteration_limit,
            nsec3_hard_iteration_limit: DnssecConfig::default().nsec3_hard_iteration_limit,
            validation_cache_size: Some(16_384),
        },
        options,
    };
    let handler = RecursiveZoneHandler::try_from_config(
        Name::root(),
        ZoneType::External,
        config,
        None,
        TokioRuntimeProvider::default(),
    )
    .await?;

    let mut catalog = Catalog::new();
    catalog.upsert(
        LowerName::from(Name::root()),
        vec![Arc::new(handler) as Arc<dyn ZoneHandler>],
    );
    for zone in forward_zones_from_environment()? {
        // djbdns uses filenames without a trailing root label; Catalog keys
        // must be fully qualified to participate in suffix matching.
        let origin = Name::from_ascii(format!("{}.", zone.name))?;
        let mut options = ResolverOpts::default();
        // Many private authoritative servers canonicalize owner case. Strict
        // 0x20 checking would make otherwise valid legacy forwarding fail.
        options.case_randomization = false;
        options.try_tcp_on_error = true;
        options.cache_size = 1024;
        let config = ForwardConfig {
            name_servers: zone
                .servers
                .into_iter()
                .map(NameServerConfig::udp_and_tcp)
                .collect(),
            options: Some(options),
        };
        let handler = ForwardZoneHandler::builder_tokio(config)
            .with_origin(origin.clone())
            .build()
            .map_err(std::io::Error::other)?;
        catalog.upsert(
            LowerName::from(origin),
            vec![Arc::new(handler) as Arc<dyn ZoneHandler>],
        );
    }

    let denied = ["0.0.0.0/0", "::/0"]
        .into_iter()
        .map(str::parse::<IpNet>)
        .collect::<Result<Vec<_>, _>>()?;
    let allowed_values = env::var("ALLOW_NETS")
        .unwrap_or_else(|_| "127.0.0.0/8,::1/128".into())
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if allowed_values.len() > 4096 {
        return Err("ALLOW_NETS contains too many networks".into());
    }
    let allowed = allowed_values
        .iter()
        .map(|value| value.parse::<IpNet>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut server = Server::with_access(catalog, denied, allowed);
    server.register_socket(UdpSocket::bind(address).await?);
    server.register_listener(
        TcpListener::bind(address).await?,
        Duration::from_secs(10),
        64 * 1024,
    );
    eprintln!("dnscache: listening on {address}; DNSSEC validation enabled");
    tokio::select! {
        result = server.block_until_done() => result?,
        signal = tokio::signal::ctrl_c() => signal?,
    }
    Ok(())
}

fn bounded_env_usize(
    name: &str,
    default: usize,
    minimum: usize,
    maximum: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    bounded_env(name, default, minimum, maximum)
}

fn bounded_env_u8(
    name: &str,
    default: u8,
    minimum: u8,
    maximum: u8,
) -> Result<u8, Box<dyn std::error::Error>> {
    bounded_env(name, default, minimum, maximum)
}

fn bounded_env<T>(
    name: &str,
    default: T,
    minimum: T,
    maximum: T,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: Copy + Ord + std::str::FromStr,
    T::Err: std::error::Error + 'static,
{
    let value = match env::var(name) {
        Ok(value) => value.parse::<T>()?,
        Err(env::VarError::NotPresent) => default,
        Err(error) => return Err(error.into()),
    };
    if !(minimum..=maximum).contains(&value) {
        return Err(format!("{name} is outside the supported range").into());
    }
    Ok(value)
}
