---
type: "code-fragment"
fragment_id: "rgbdns-frag-f9c2e38b98db"
source_path: "src/bin/dnscache.rs"
code_note: "DNS from First Principles/Code/src/bin/dnscache.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnscache"
symbol: "run"
kind: "fn"
start_line: 36
end_line: 116
---

# run

- Fragment ID: `rgbdns-frag-f9c2e38b98db`
- Source file: [[DNS from First Principles/Code/src/bin/dnscache.rs.source|src/bin/dnscache.rs]]
- Lines: 36-116
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnscache|dnscache]]

```rgbdns-fragment
{"id": "rgbdns-frag-f9c2e38b98db", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-f9c2e38b98db: fn run", "sourcePath": "src/bin/dnscache.rs", "startLine": 36, "endLine": 116}
```

## Excerpt

<span id="rgbdns-frag-f9c2e38b98db" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f9c2e38b98db: fn run

```rust
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
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
```
