use rgbdns::{cdb, zone::Zone};
fn main() {
    let result = Zone::from_file("data").and_then(|zone| {
        cdb::compile(&zone, "data.cdb")?;
        if let Ok(config) = std::env::var("ACME_UPDATE_CONFIG")
            && !config.trim().is_empty()
        {
            let state_dir = std::env::var("ACME_STATE_DIR")
                .unwrap_or_else(|_| "/var/lib/rgbdns/tinydns".into());
            let live = rgbdns::acme::LiveZone::new(zone.clone());
            let _ = rgbdns::acme::AcmeUpdates::from_file(config, state_dir, zone, live)?;
        }
        Ok(())
    });
    if let Err(e) = result {
        eprintln!("tinydns-data: fatal: {e}");
        std::process::exit(111)
    }
}
