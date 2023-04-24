CREATE TABLE users(
  id      UUID         NOT NULL,
  address VARCHAR(128) NOT NULL,
  name    VARCHAR(128) NOT NULL,
  pass    VARCHAR(256) NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  verified_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  PRIMARY KEY (id, address)
);

CREATE TYPE TEP_AM
  AS ENUM (
    'client_secret_post',
    'client_secret_basic',
    'private_key_jwt',
    'none'
  );

CREATE TYPE GRANT_TYPE
  AS ENUM (
    'authorization_code',
    'implicit',
    'password',
    'client_credentials',
    'refresh_token',
    'urn:ietf:params:oauth:grant-type:jwt-bearer',
    'urn:ietf:params:oauth:grant-type:saml2-bearer'
  );

CREATE TABLE __client(
  id             UUID           NOT NULL PRIMARY KEY,
  redirect_uris  VARCHAR(512)[] NOT NULL UNIQUE,
  auth_method    TEP_AM         NOT NULL,
  grant_type     GRANT_TYPE[]   NOT NULL,
  response_types VARCHAR(512)[] NOT NULL,
  client_name    VARCHAR(256)   NOT NULL,
  client_uri     VARCHAR(512),
  logo_uri       VARCHAR(512),
  scopes         VARCHAR(128)[] NOT NULL,
  req_scopes     VARCHAR(128)[],
  contacts       VARCHAR(128)[] NOT NULL,
  tos_uri        VARCHAR(512)   NOT NULL,
  policy_uri     VARCHAR(512)   NOT NULL,
  jwks           JSONB          NOT NULL,
  software_id    VARCHAR(256),
  software_ver   VARCHAR(128)
);

CREATE TABLE ___clients(
  id          UUID         NOT NULL,
  client_name VARCHAR(256) NOT NULL UNIQUE,
  PRIMARY KEY(id, client_name)
);

CREATE TABLE client_metadata(
  id             UUID           NOT NULL PRIMARY KEY,
  client_id      UUID           NOT NULL,
  client_uri     VARCHAR(512),
  logo_uri       VARCHAR(512),
  contact        VARCHAR(128)[] NOT NULL,
  tos_uri        VARCHAR(512)   NOT NULL,
  policy_uri     VARCHAR(512)   NOT NULL,
  FOREIGN KEY (client_id) REFERENCES ___clients(id) ON DELETE CASCADE
);

CREATE TABLE client_cert(
  -- TODO: Impl client authorization registry.
);

CREATE TABLE redirect_uris(
  id        UUID NOT NULL,
  uri       VARCHAR(512) NOT NULL,
  client_id UUID NOT NULL,
  PRIMARY KEY (id, uri),
  FOREIGN KEY (client_id) REFERENCES ___clients(id) ON DELETE CASCADE
);

CREATE TABLE scopes(
  id          UUID         NOT NULL PRIMARY KEY,
  client_id   UUID         NOT NULL,
  method      VARCHAR(64)  NOT NULL,
  description VARCHAR(256) NOT NULL,

  UNIQUE (method, client_id),
  FOREIGN KEY (client_id) REFERENCES ___clients(id) ON DELETE CASCADE
);

-- Will deprecate.
CREATE TABLE clients(
  id           UUID         NOT NULL PRIMARY KEY,
  name         VARCHAR(128) NOT NULL,
  description  TEXT,
  redirect_uri VARCHAR(256) NOT NULL,
  secret       VARCHAR(256)
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