CREATE TABLE users(
  user_id UUID         NOT NULL PRIMARY KEY,
  address VARCHAR(128) NOT NULL UNIQUE,
  name    VARCHAR(128) NOT NULL,
  pass    VARCHAR(256) NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  verified_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
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

CREATE TYPE RESPONSE_TYPE
  AS ENUM (
    'code',
    'token'
  );

CREATE TABLE clients(
  client_id     UUID         NOT NULL PRIMARY KEY,
  client_id_iat TIMESTAMPTZ  NOT NULL,
  client_name   VARCHAR(256) NOT NULL UNIQUE,

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE client_metadata(
  owner          UUID           NOT NULL,
  client_id      UUID           NOT NULL,
  client_uri     VARCHAR(512),
  logo_uri       VARCHAR(512),
  contact        VARCHAR(128)[] NOT NULL,
  tos_uri        VARCHAR(512)   NOT NULL,
  policy_uri     VARCHAR(512)   NOT NULL,

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  PRIMARY KEY (owner, client_id),

  FOREIGN KEY (owner)     REFERENCES users(user_id)     ON DELETE CASCADE,
  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);

CREATE TABLE client_cert(
  client_id         UUID            NOT NULL PRIMARY KEY,
  client_secret     VARCHAR(64)     CHECK ( client_secret_exp IS NOT NULL ),
  client_secret_exp TIMESTAMPTZ     CHECK ( client_secret IS NOT NULL ),
  auth_method       TEP_AM          NOT NULL,
  grant_types       GRANT_TYPE[]    NOT NULL,
  response_types    RESPONSE_TYPE[] NOT NULL,
  jwks              JSONB           CHECK ( jwks_uri IS NULL ), -- ─┬─ MUST NOT both be present in the same request or response.
  jwks_uri          VARCHAR(512)    CHECK ( jwks IS NULL ),     -- ─┘

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);

CREATE TABLE client_redirect_uris(
  client_id UUID           NOT NULL,
  uri       VARCHAR(512)[] NOT NULL,

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  PRIMARY KEY (uri, client_id),
  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);

CREATE TABLE client_scopes(
  client_id   UUID  NOT NULL,
  scope       JSONB NOT NULL,

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  PRIMARY KEY (client_id, scope),
  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);

-- Referenced RFC7592 Dynamic Client Registration Management Protocol
CREATE TABLE client_configuration_policy(
  client_id UUID         NOT NULL PRIMARY KEY,
  endpoint  VARCHAR(32)  NOT NULL UNIQUE,
  token     VARCHAR(64)  NOT NULL UNIQUE,

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);