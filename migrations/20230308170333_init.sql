CREATE TABLE users(
  id      UUID         NOT NULL PRIMARY KEY,
  address VARCHAR(128) NOT NULL UNIQUE,
  name    VARCHAR(128) NOT NULL,
  pass    VARCHAR(256) NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  verified_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE clients(
  id           UUID         NOT NULL PRIMARY KEY,
  name         VARCHAR(128) NOT NULL,
  description  TEXT,
  redirect_uri VARCHAR(256) NOT NULL,
  secret       VARCHAR(256)
);

CREATE TYPE access AS ENUM ('confidential', 'public');

CREATE TABLE scopes(
  id          UUID NOT NULL PRIMARY KEY,
  client      UUID NOT NULL,
  access      access      NOT NULL,
  method      VARCHAR(64) NOT NULL,
  description TEXT,
  FOREIGN KEY (client) REFERENCES clients(id) ON DELETE CASCADE
);

CREATE TABLE token(
  id      UUID NOT NULL PRIMARY KEY,
  account UUID NOT NULL,
  client  UUID NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  issuer     VARCHAR(128) NOT NULL,
  audience   VARCHAR(128) NOT NULL,
  expired_in TIMESTAMPTZ  NOT NULL,
  issued_at  TIMESTAMPTZ  NOT NULL DEFAULT clock_timestamp(),
  not_before TIMESTAMPTZ  NOT NULL DEFAULT clock_timestamp(),
  subject    VARCHAR(128) NOT NULL,

  FOREIGN KEY (account) REFERENCES users(id)   ON DELETE CASCADE,
  FOREIGN KEY (client)  REFERENCES clients(id) ON DELETE CASCADE
);

CREATE TABLE token_scope(
  id       UUID          NOT NULL PRIMARY KEY,
  token_id UUID          NOT NULL,
  scoped   VARCHAR(32)[] NOT NULL,

  FOREIGN KEY (token_id) REFERENCES token(id) ON DELETE CASCADE
);