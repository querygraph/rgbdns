# Wishfully

Wishfully is the Vercel control plane and CLI for domains hosted by rgbdns on
`a.ns.cron.sh` and `b.ns.cron.sh`. It verifies ownership, stores structured
zone manifests, compiles deterministic consolidated `rgbdns.data` and
`rgbdns.zones` files, and opens reviewed deployment pull requests.

## Local verification

```sh
npm ci
npm audit --audit-level=low
npm test
npm run typecheck
npm run build
```

Copy `.env.example` to `.env.local` and configure PostgreSQL plus the GitHub
App before using mutation routes. Initialize the database and bootstrap the
first API token with:

```sh
npm run db:migrate
WISHFULLY_BOOTSTRAP_ACCOUNT=Operations \
WISHFULLY_BOOTSTRAP_TIER=studio \
npm run db:bootstrap
```

The token is printed once. Use it with the CLI:

```sh
export WISHFULLY_API_URL=https://wishful.ly
export WISHFULLY_TOKEN=wishfully_...
npm link
wishfully domains list
```

See [`../WISHFULLY.md`](../WISHFULLY.md) for architecture, tiers, DNS
onboarding, GitHub/Vercel configuration, and the Certbot DNS-01 lifecycle.
