# Wishfully: hosted rgbdns control plane

Wishfully is a Vercel-hosted control plane for authoritative domains served by
`a.ns.cron.sh` and `b.ns.cron.sh`. The application lives under `wishfully/`
and is intended for the production hostname `wishful.ly`.

The product does not replace the deployment path already operating rgbdns. It
turns user intent into reviewed, deterministic inputs for that path:

```text
web app / CLI
      │
      ▼
tenant domain manifests ── ownership and policy checks
      │
      ▼
deterministic compiler
      ├── rgbdns.data    one primary source for every hosted zone
      └── rgbdns.zones   one name per line for secondary synchronization
      │
      ▼
GitHub pull request and existing deploy-dns.yml
      │
      ├── atomic data drop to a.ns.cron.sh
      └── atomic zone-list drop to b.ns.cron.sh
```

## Product tiers

| Tier | Price | Intended user | Limits and features |
|---|---:|---|---|
| Seed | Free | One personal project | 1 zone, 25 records, standard record types, both authorities, community support |
| Maker | $9/month | Independent sites and publications | 10 zones, unlimited records, ANAME, scoped Certbot credentials, history, rollback, email support |
| Studio | $39/month | Small teams and portfolios | 100 zones, roles, audit log, bulk import, API access, delegated ACME validation zones, priority support |
| Infrastructure | Custom | SaaS and registrars | Higher limits, SSO, contractual SLA, dedicated authorities, custom AXFR peers and migration support |

Do not sell “unlimited zones” on shared authority. DNS abuse, packet volume,
record count, and support cost all scale differently; explicit limits make
capacity planning and suspension policy intelligible.

## Domain onboarding

1. The user enters a lower-case ASCII domain in the web app or runs:

   ```sh
   wishfully domains add example.com --aname project.vercel.app
   ```

2. Wishfully creates a random ownership token and asks the user to publish it
   at `_wishfully.example.com` through the current DNS provider. Verification
   follows CNAMEs, requires an exact TXT value, and records the observation.
3. The user chooses records through structured forms or imports an AXFR/BIND
   zone for review. Unsupported provider-specific flattening is never silently
   converted. It becomes an explicit rgbdns ANAME decision.
4. A manifest compiler validates ownership, record coexistence, quotas,
   in-bailiwick names, serial monotonicity, and delegation. It produces a pull
   request containing the consolidated `rgbdns.data` and `rgbdns.zones` diff.
5. Existing CI compiles the CDB and verifies that every secondary zone contains
   the SOA plus both rgbdns nameservers. Approval and merge invoke the existing
   atomic deployment workflow.
6. Wishfully queries UDP and TCP on both authorities until the expected SOA
   serial and record digest agree. Only then does it instruct the user to set:

   ```text
   a.ns.cron.sh
   b.ns.cron.sh
   ```

7. After public delegation converges, the old provider can be removed.

The service must not infer ownership merely because the registrant delegates a
domain to the shared nameservers: an attacker could briefly delegate another
party's expired or misconfigured name. The pre-delegation TXT challenge is the
durable authorization event.

## Consolidated source model

The durable application database stores one structured manifest per domain.
The generated files remain deployment artifacts and are never edited by two
concurrent requests. A single serialized reconciler:

1. reads all active manifests at a database snapshot;
2. sorts zones and records canonically;
3. advances only the SOA serials whose semantic record digest changed;
4. emits complete temporary files;
5. runs `tinydns-data` and policy validation;
6. opens one GitHub pull request for the resulting pair.

Every manifest revision records actor, account, previous digest, requested
change, verification evidence and deployment commit. Idempotency keys prevent
CLI retries from creating duplicate changes. A unique constraint gives each
domain one active owner.

The Vercel application should use managed PostgreSQL for accounts, memberships,
API-token hashes, domains, manifests, challenges, deployments and certificate
credential metadata. Git is the deployment ledger, not the authentication or
billing database.

## Authentication and authorization

- Web users authenticate by passkey or verified email; team roles are owner,
  administrator, editor and viewer.
- CLI tokens are shown once, stored only as Argon2id hashes, scoped to an
  account and optionally a set of zones, and revocable independently.
- The Vercel application uses a GitHub App installation token, not a personal
  access token. The App can create branches and pull requests but cannot
  approve its own protected deployment environment.
- Domain verification tokens expire. Successful proof remains attached to the
  account, while ownership is rechecked on sensitive actions such as account
  transfer.
- Billing state can prevent new mutations but must never abruptly remove
  authoritative service. Suspension is a reviewed state transition with a
  notification and export window.

## Certbot and Let's Encrypt

Wishfully uses DNS-01 through rgbdns's authenticated RFC 2136 update profile.
Certificate private keys remain on the user's machine or workload.

### Credential provisioning

After the domain is active, Maker and higher users run:

```sh
wishfully certbot credentials example.com \
  --output ~/.config/letsencrypt/wishfully-example.ini
```

Wishfully creates a distinct HMAC-SHA256 TSIG key authorized only for
`_acme-challenge` owners within that zone. The secret is returned once. The
database retains an encrypted copy only long enough to publish a signed ACME
policy bundle to the primary; thereafter it retains the key name, scope,
creation time and revocation state. The clear secret is never committed to
Git, logged, placed in `rgbdns.data`, or sent to the secondary.

The primary policy corresponds to:

```text
wishfully-example. hmac-sha256. SECRET example.com. _acme-challenge. 60
```

The deployment action obtains the encrypted policy bundle from an
authenticated, single-purpose Wishfully endpoint and atomically installs the
preserved `/etc/rgbdns/acme-update.conf`. This is separate from the public
zone-data diff because Git must never contain TSIG material.

### Running Certbot

The CLI can print the exact command or execute it after confirmation:

```sh
wishfully certbot run example.com
```

Equivalent direct Certbot invocation:

```sh
certbot certonly \
  --dns-rfc2136 \
  --dns-rfc2136-credentials ~/.config/letsencrypt/wishfully-example.ini \
  --dns-rfc2136-propagation-seconds 90 \
  -d example.com -d '*.example.com'
```

The generated INI contains the primary endpoint, key name, secret and
HMAC-SHA256 algorithm and must be mode 0600. Certbot adds a TXT value, waits
for propagation, completes validation and removes only its own value. rgbdns
advances the logical SOA serial, publishes the overlay immediately and includes
it in ordinary AXFR so `b.ns.cron.sh` converges through the existing transfer.

For customers unwilling to grant UPDATE authority on the application zone,
Wishfully can provision a delegated validation zone and instruct them to add a
CNAME from `_acme-challenge.example.com`. This is the preferred Studio pattern.

### Renewal and revocation

`certbot renew` remains owned by the workload that owns the private key. The
Wishfully CLI installs no root daemon by default; it can generate a systemd
timer or launchd plist after explicit confirmation. Revoking a Wishfully API
token does not revoke TSIG automatically because that could break renewals.
Certificate credentials have their own rotate and revoke commands.

## API and CLI surface

Versioned API routes:

```text
POST   /api/v1/domains
GET    /api/v1/domains
GET    /api/v1/domains/:name
POST   /api/v1/domains/:name/verify
PUT    /api/v1/domains/:name/records
POST   /api/v1/domains/:name/deployments
GET    /api/v1/deployments/:id
POST   /api/v1/domains/:name/acme-credentials
POST   /api/v1/acme-credentials/:id/rotate
DELETE /api/v1/acme-credentials/:id
```

CLI commands mirror those resources:

```text
wishfully login
wishfully domains add/list/show/verify/remove
wishfully records list/set/delete/import
wishfully deployments list/watch/rollback
wishfully certbot credentials/run/rotate/revoke
```

The repository currently implements the public planner endpoint and
`wishfully domains plan`. Mutating endpoints must not be enabled until the
database, authentication, GitHub App, protected environment and encrypted ACME
bundle path are configured.

## Deployment to Vercel

Create a Vercel project whose root directory is `wishfully`, attach
`wishful.ly`, and configure the environment shown in `wishfully/.env.example`.
The public planner and health endpoint require no secrets. Production mutation
routes require PostgreSQL, authentication, a GitHub App installation and an
ACME bundle-signing key.

Before enabling mutations:

- require pull-request checks and the `production-dns` environment approval;
- configure a Content Security Policy and rate limits at Vercel's edge;
- make webhook handlers idempotent and verify their signatures;
- test domain races, serial collisions and failed partial deployments;
- exercise Let's Encrypt staging, concurrent wildcard/apex challenges, cleanup,
  renewal and TSIG rotation;
- monitor both authoritative endpoints from outside AWS;
- document zone export and account deletion before accepting payment.
