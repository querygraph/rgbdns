---
type: "code-fragment"
fragment_id: "rgbdns-frag-41728d016c64"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 593
end_line: 673
---

# serve

- Fragment ID: `rgbdns-frag-41728d016c64`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 593-673
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-41728d016c64", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 593, "endLine": 673}
```

## Excerpt

<span id="rgbdns-frag-41728d016c64" class="rgbdns-fragment-target"></span>
### rgbdns-frag-41728d016c64: fn serve

```rust
pub fn serve(zone: Zone, addr: &str) -> Result<()> {
    let query_logger = QueryLogger::from_env()?;
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?
        .map(Arc::new);
    let live = crate::acme::LiveZone::new(zone.clone());
    let acme = std::env::var("ACME_UPDATE_CONFIG")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(|config| {
            let state_dir = std::env::var("ACME_STATE_DIR")
                .unwrap_or_else(|_| "/var/lib/rgbdns/tinydns".into());
            let publication = match (
                std::env::var("ACME_PUBLISH_COMMAND").ok(),
                std::env::var("ACME_PUBLISHED_DATA").ok(),
            ) {
                (Some(command), Some(data)) => {
                    Some(crate::acme::Publication::new(command.into(), data.into())?)
                }
                (None, None) => None,
                _ => {
                    return Err(Error::Format(
                        "ACME_PUBLISH_COMMAND and ACME_PUBLISHED_DATA must be set together",
                    ));
                }
            };
            crate::acme::AcmeUpdates::from_file_with_publication(
                config,
                state_dir,
                zone.clone(),
                live.clone(),
                publication,
            )
            .map(Arc::new)
        })
        .transpose()?;
    let allowed = std::env::var("ALLOW_NETS")
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::parse::<ipnet::IpNet>)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|_| Error::Format("invalid ALLOW_NETS"))
        })
        .transpose()?
        .map(Arc::new);
    let stream_handler = if allowed.is_some() || acme.is_some() {
        Some(authoritative_stream_handler(
            live.clone(),
            allowed,
            acme.clone(),
            query_logger,
        ))
    } else {
        None
    };
    query_logger.starting();
    crate::transport::serve(
        addr,
        Arc::new(move |wire, limit, client| {
            if crate::acme::AcmeUpdates::is_update(wire) {
                let response = error_response(wire, 5);
                query_logger.request(client, wire, response.as_deref().ok());
                return response;
            }
            let zone = live.snapshot();
            let response = respond_over_transport(
                &zone,
                resolver.as_deref(),
                wire,
                limit,
                limit <= 4096,
                Some(client.ip()),
            );
            query_logger.request(client, wire, response.as_deref().ok());
            response
```
