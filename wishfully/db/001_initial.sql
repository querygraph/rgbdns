CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE accounts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  tier text NOT NULL DEFAULT 'seed' CHECK (tier IN ('seed','maker','studio','infrastructure')),
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE api_tokens (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  label text NOT NULL,
  token_hash text NOT NULL UNIQUE,
  last_used_at timestamptz,
  expires_at timestamptz,
  revoked_at timestamptz,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE domains (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  name text NOT NULL UNIQUE,
  status text NOT NULL DEFAULT 'pending_verification' CHECK (status IN ('pending_verification','verified','pending_delegation','active','suspended')),
  verification_token text NOT NULL,
  destination_type text NOT NULL CHECK (destination_type IN ('A','ANAME')),
  destination_value text NOT NULL,
  include_www boolean NOT NULL DEFAULT true,
  verified_at timestamptz,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE records (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  domain_id uuid NOT NULL REFERENCES domains(id) ON DELETE CASCADE,
  owner text NOT NULL,
  type text NOT NULL CHECK (type IN ('A','AAAA','ANAME','CNAME','MX','TXT','CAA','SRV')),
  value text NOT NULL,
  ttl integer NOT NULL DEFAULT 300 CHECK (ttl BETWEEN 30 AND 86400),
  priority integer,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE deployments (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  status text NOT NULL DEFAULT 'planning' CHECK (status IN ('planning','pull_request','deploying','succeeded','failed')),
  branch text,
  pull_request_url text,
  source_digest text,
  error text,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX domains_account_idx ON domains(account_id);
CREATE INDEX records_domain_idx ON records(domain_id);
CREATE INDEX deployments_account_idx ON deployments(account_id, created_at DESC);
