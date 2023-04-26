CREATE TABLE users(
  user_id UUID         NOT NULL PRIMARY KEY,
  address VARCHAR(128) NOT NULL,
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
  client_id   UUID         NOT NULL PRIMARY KEY ,
  client_name VARCHAR(256) NOT NULL UNIQUE,

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
  client_id      UUID            NOT NULL PRIMARY KEY,
  auth_method    TEP_AM          NOT NULL,
  grant_types    GRANT_TYPE[]    NOT NULL,
  response_types RESPONSE_TYPE[] NOT NULL,
  jwks           JSONB           CHECK ( jwks_uri IS NULL ), -- ─┬─ MUST NOT both be present in the same request or response.
  jwks_uri       VARCHAR(512)    CHECK ( jwks IS NULL ),     -- ─┘

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);

CREATE TABLE redirect_uris(
  client_id UUID         NOT NULL,
  uri       VARCHAR(512) NOT NULL,

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  PRIMARY KEY (uri, client_id),
  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);

CREATE TABLE scopes(
  client_id   UUID         NOT NULL,
  method      VARCHAR(64)  NOT NULL,
  description VARCHAR(256) NOT NULL,

  created_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  PRIMARY KEY (method, client_id),
  FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
);