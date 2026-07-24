---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Operating an authoritative service

## Build, stage, verify, replace

A safe publication cycle separates source editing from serving:

```sh
cd /etc/rgbdns
tinydns-data
tinydns-get example.com A www.example.com
```

In production, compile in a staging directory, run representative exact,
wildcard, delegation, negative, IPv4, IPv6, and large-response queries, then
atomically replace `data.cdb`. Retain the previous known-good database for
rollback. Query the bound service over both UDP and TCP after deployment.

Do not expose the recursive service to arbitrary networks by accident. The
default `ALLOW_NETS` is loopback only because an open resolver can be abused
for amplification and can consume local capacity. Likewise, expand AXFR
allowlists only for intended secondaries.

## Observe the right signals

Useful signals include:

- query and error rate by transport;
- truncated UDP responses and TCP retries;
- SERVFAIL, REFUSED, NXDOMAIN, and validation-failure rates;
- resolver cache capacity and latency percentiles;
- process restarts and file-descriptor use;
- root-hint and trust-anchor freshness;
- time synchronization;
- CDB build identity and deployment time.

High NXDOMAIN volume is not automatically an incident; browsers, typo traffic,
and discovery protocols generate it. A change from baseline paired with
latency or SERVFAIL is more meaningful.

TAI64N log labels make events stable for storage. Convert them for human
display at the edge:

```sh
tail -f main/current | tai64nlocal
```

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-e5fff4b8cb2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-e5fff4b8cb2b: const HEADER_LEN", "sourcePath": "src/cdb.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4907bb687e44", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4907bb687e44: const MAX_DATABASE_SIZE", "sourcePath": "src/cdb.rs", "startLine": 10, "endLine": 11}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4a9794b88127", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4a9794b88127: fn compile", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 52}
```

```rgbdns-fragment
{"id": "rgbdns-frag-83b463908a3c", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-83b463908a3c: fn load", "sourcePath": "src/cdb.rs", "startLine": 53, "endLine": 69}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9e8e0d51389c", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-9e8e0d51389c: fn read_entries", "sourcePath": "src/cdb.rs", "startLine": 70, "endLine": 124}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4a71bdeba2ec", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4a71bdeba2ec: fn decode_record", "sourcePath": "src/cdb.rs", "startLine": 125, "endLine": 178}
```

```rgbdns-fragment
{"id": "rgbdns-frag-a98232a8cdb0", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-a98232a8cdb0: fn encode_rdata", "sourcePath": "src/cdb.rs", "startLine": 179, "endLine": 237}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1cfb12457767", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-1cfb12457767: fn decode_name", "sourcePath": "src/cdb.rs", "startLine": 238, "endLine": 264}
```

```rgbdns-fragment
{"id": "rgbdns-frag-908320134aee", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-908320134aee: fn le_u32", "sourcePath": "src/cdb.rs", "startLine": 265, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-489b39a43ae6", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-489b39a43ae6: mod tests", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 275}
```

```rgbdns-fragment
{"id": "rgbdns-frag-563d0d5def13", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-563d0d5def13: fn exact_cdb_roundtrip_preserves_lookup_semantics", "sourcePath": "src/cdb.rs", "startLine": 276, "endLine": 324}
```

```rgbdns-fragment
{"id": "rgbdns-frag-af1b07391a27", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-af1b07391a27: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 325, "endLine": 331}
```
