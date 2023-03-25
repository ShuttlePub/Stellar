CREATE TABLE users(
  id      UUID         NOT NULL PRIMARY KEY,
  address VARCHAR(128) NOT NULL UNIQUE,
  name    VARCHAR(128) NOT NULL,
  pass    VARCHAR(256) NOT NULL,
  verified_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE token(
  id UUID NOT NULL PRIMARY KEY,
  client UUID NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  issuer     VARCHAR(128) NOT NULL,
  audience   VARCHAR(128) NOT NULL,
  expired_in TIMESTAMPTZ  NOT NULL,
  issued_at  TIMESTAMPTZ  NOT NULL DEFAULT clock_timestamp(),
  not_before TIMESTAMPTZ  NOT NULL DEFAULT clock_timestamp(),
  subject    VARCHAR(128) NOT NULL,

  FOREIGN KEY (client) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE token_scope(
  id UUID NOT NULL PRIMARY KEY,
  token_id UUID NOT NULL,
  scoped VARCHAR(32)[] NOT NULL,

  FOREIGN KEY (token_id) REFERENCES token(id) ON DELETE CASCADE
);