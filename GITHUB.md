# GitHub DNS change and deployment workflow

Changes to `rgbdns.data` and `rgbdns.zones` are deployed through
`.github/workflows/deploy-dns.yml`. Use a branch and pull request so GitHub
validates DNS data before a change reaches production.

## What triggers the workflow

The workflow runs when any of these files change:

- `rgbdns.data`
- `rgbdns.zones`
- `.github/workflows/deploy-dns.yml`

A pull request runs only the `validate` job. A push or merge to `master` runs
validation and, if it passes, the production deployment. A push to any other
branch does not deploy.

The workflow can also be started with `workflow_dispatch`. A manually selected
ref is eligible for deployment, so do not manually run this workflow from an
unreviewed branch.

## Prepare a DNS change

Start from an up-to-date `master` and create a branch:

```sh
git switch master
git pull --ff-only origin master
git switch -c codex/dns-change-description
```

Edit both files as appropriate:

- Add the complete authoritative zone block to `rgbdns.data`.
- Add the zone name to `rgbdns.zones` when `b.ns.cron.sh` must transfer and
  serve it.
- Increment an existing zone's SOA serial for every published change.
- Keep zone names lowercase and list each secondary zone exactly once.
- Do not leave trailing whitespace or extra blank lines at EOF.

The helper script creates a new zone with the production nameservers and either
an A or ANAME record:

```sh
./scripts/add-domain.sh example.com A 216.150.1.1
./scripts/add-domain.sh example.com ANAME target.example.net
```

The script appends to `rgbdns.data`; add the domain separately to
`rgbdns.zones` when it belongs on the secondary.

ANAME zones rely on rgbdns's explicitly negotiated experimental AXFR
extension. Delegate them only to upgraded rgbdns authorities such as
`a.ns.cron.sh` and `b.ns.cron.sh`; an ordinary secondary cannot reproduce the
ANAME metadata.

## Check and submit the change

At minimum, inspect the patch and run the whitespace check:

```sh
git diff -- rgbdns.data rgbdns.zones
git diff --check
```

Commit only the intended files, push the branch, and open a pull request:

```sh
git add rgbdns.data rgbdns.zones
git commit -m "Add DNS zones"
git push -u origin HEAD
gh pr create --base master
```

The pull-request workflow verifies that:

1. `rgbdns.data` compiles into a nonempty CDB with `tinydns-data`.
2. `rgbdns.zones` is nonempty and has no duplicate or malformed zone names.
3. Every secondary zone has an SOA and exact `a.ns.cron.sh` and
   `b.ns.cron.sh` records in `rgbdns.data`.
4. `git diff --check` passes.

Wait for the `Deploy DNS data / validate` check to pass. Do not merge locally
and push to bypass a pending or failed pull-request check.

## Merge and deploy

Merge the pull request into `master` on GitHub. The resulting push to `master`
starts the production workflow automatically. The deployment job may wait for
approval if the `production-dns` environment has protection rules.

The deployment proceeds in this order:

1. Copy `rgbdns.data` to the primary and rename it atomically into place.
2. Wait for every zone in `rgbdns.zones` to publish the expected SOA serial on
   the primary.
3. Copy `rgbdns.zones` to the secondary and rename it atomically into place.
4. Wait for every listed zone to publish the expected SOA serial on the
   secondary.

Production deployment requires the `RGBDNS_DEPLOY_KEY` and
`RGBDNS_SSH_KNOWN_HOSTS` secrets. Repository variables can override the
primary and secondary hosts, users, and secondary drop path; otherwise the
workflow uses the deployment defaults recorded in the workflow file.

Monitor the run with GitHub or the CLI:

```sh
gh run list --workflow deploy-dns.yml --branch master --limit 5
gh run watch
```

After a successful merge, update the local checkout:

```sh
git switch master
git pull --ff-only origin master
git branch -d codex/dns-change-description
```

Delete the remote topic branch through the pull-request UI or with:

```sh
git push origin --delete codex/dns-change-description
```

## Failure and rollback

A failed validation job does not deploy anything. Fix the branch and push a
new commit to update the pull request.

A deployment failure can occur after primary data has already been published;
the workflow does not automatically roll it back. Inspect the failed step and
the authoritative SOA responses before taking further action. Roll back with a
new reviewed commit that restores the last-known-good records and uses a newer
SOA serial. Reusing an older serial can prevent secondaries and caches from
recognizing the correction.
