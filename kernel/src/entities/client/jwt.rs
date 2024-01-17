use crate::KernelError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct ClientSignJwt(String);

#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct ClientDecodeJwt(JwtClaims);

#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct JwtClaims {
    iss: Uuid,
    sub: Uuid,
    aud: String,
    exp: i64,
    jti: String,
    iat: i64,
}

impl ClientSignJwt {
    pub fn claims() -> JwtClaims {
        JwtClaims {
            iss: Uuid::new_v4(),
            sub: Uuid::new_v4(),
            aud: "placeholder".to_string(),
            exp: 0,
            jti: "".to_string(),
            iat: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }

    pub fn decode(self, pubkey: impl AsRef<[u8]>) -> Result<ClientDecodeJwt, KernelError> {
        let key = DecodingKey::from_rsa_pem(pubkey.as_ref())?;
        let mut val = Validation::new(Algorithm::RS512);
        val.validate_aud = false; // FixMe: Should validate aud.
        let dec = jsonwebtoken::decode::<JwtClaims>(self.as_ref(), &key, &val)?;
        Ok(ClientDecodeJwt(dec.claims))
    }
}

impl ClientDecodeJwt {
    pub fn iss(&self) -> &Uuid {
        &self.0.iss
    }

    pub fn sub(&self) -> &Uuid {
        &self.0.sub
    }

    pub fn aud(&self) -> &str {
        &self.0.aud
    }

    pub fn exp(&self) -> Result<OffsetDateTime, KernelError> {
        OffsetDateTime::from_unix_timestamp(self.0.exp).map_err(|e| KernelError::InvalidValue {
            method: "try_ref_exp",
            value: e.to_string(),
        })
    }

    pub fn jti(&self) -> &str {
        &self.0.jti
    }
}

impl JwtClaims {
    pub fn iss(&mut self, iss: impl Into<Uuid>) -> &mut Self {
        self.iss = iss.into();
        self
    }

    pub fn sub(&mut self, sub: impl Into<Uuid>) -> &mut Self {
        self.sub = sub.into();
        self
    }

    pub fn aud(&mut self, aud: impl Into<String>) -> &mut Self {
        self.aud = aud.into();
        self
    }

    pub fn exp(&mut self, exp: impl Into<OffsetDateTime>) -> &mut Self {
        self.exp = exp.into().unix_timestamp();
        self
    }

    pub fn jti(&mut self, jti: impl Into<String>) -> &mut Self {
        self.jti = jti.into();
        self
    }

    pub fn encode(self, privkey: impl AsRef<[u8]>) -> Result<ClientSignJwt, KernelError> {
        let hed = Header::new(Algorithm::RS512);
        let key = EncodingKey::from_rsa_pem(privkey.as_ref())?;
        let enc = jsonwebtoken::encode::<JwtClaims>(&hed, &self, &key)?;
        Ok(self.jwt(enc))
    }

    fn jwt(self, jwt: impl Into<String>) -> ClientSignJwt {
        ClientSignJwt(jwt.into())
    }
}

impl From<ClientSignJwt> for String {
    fn from(value: ClientSignJwt) -> Self {
        value.0
    }
}

impl AsRef<String> for ClientSignJwt {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{ClientId, ClientSignJwt, TicketId, UserId};
    use time::{Duration, OffsetDateTime};
    use uuid::Uuid;

    #[test]
    fn jwt() -> anyhow::Result<()> {
        let client_id = ClientId::new_at_now(Uuid::new_v4());
        let user_id = UserId::new(Uuid::new_v4());
        let ticket = TicketId::default();

        let test_privkey = r#"-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQDbMeVgbOOI6uT/
Jj4JqP3FTP2ymBTG15jcOZekMxgeYr/D5Yn4G8OZyVqc5UfPnUe1nZYxIOz5G6kV
scToNVPfqyvsyp9wpV0jZiUpOwG7zv6sSuG4cIXjg1ZBycNNEaYLqki4xw/yfei+
46DHYn/MYU7SipmGSIgcBRXEixvnZU4WyUl1+hfzsTcWUe6s7UDR/SLm8PnmUALA
aKhyQro46vSTMUD8W01drehnxtk+MjLOPnWI21N7rzVp3fecKn+rYBZJWZhoaMIa
eKFgASVqt1u19Zjs1sBFhiYegkRh/UO0v8wDOEN5mv7RslzTkaEkNJ1DjUrcDbAc
QZeuAdwdAgMBAAECggEAJ2EZcm9dmXWJ2oUGVDn2DYI9ed50rRyCq9307lz3kk3v
YcRVOyvRwkk4bEOM2THKdkhveTgPGtnrUKh1Vbpu7RoVdB8a8ePjjb6GIpglQZZ4
jn+NedNNWoh38K3M1oW7LRFUq02oAcq5C8JdWjTJ/O/md1DLMCLLwSzwsjTDuJhX
SK//LMg4E1rYkdMxJMx7VD2KyGyfSM4akC/Fqgca3v707nv1nRB8W+BAxGfGgOGa
5381KlTxSX1f3Y7EJ1qziGtu1ROSUCkKXGVLOSD6wD5axO7Yi9JrEQ1yrOFKstmi
MobfUCMYtHb94196/fOs1hbcU4WX0gMvH8ckGjuNgQKBgQDbj7cyqFY7d3btsLR5
le/UUHn0fIYrVI4J3UNOC3K5GiJkpPdgSgNSYCmQZ6t7WKfsh5NeA4BOdKEBBFQd
+Ao1tjAmKe0xjiGoOVDGVT2mmE6GGkR+8o7Iu7eG3JJ6Rb/HG8mhjJus8HVPhYlN
5pQZI45vuLt+AcDRNF5UR2hV8QKBgQD/kpwos9KiF+Lh19BFYuFqk6uk6jDj7urR
qCwNnD1Z6m4zXJ3kgEoe/RGdWIBdOuZs3goX9z36wWurWmR1cpPPm7deWQh8Kpwa
rK0Pef7bHYdW3O27Vp+ofi9tVDOMnleOKYcxqPWKljMSFsYTuLTDPDAEs2/XYoTb
A8GbpZMM7QKBgQC3Msw+RX1mMwEVxNphlPy+38NSf1CH+Q5nJqrRBPZdgnPuHEOY
oUjyQ/CKYCYSTmAXGOyahjtZuzDYI194fg0u0eyM+3DBZ4Gn1uMSCe/eALmMZYB/
mC4RSagRrBvAUFB7dzEfTa5bd2u6xYZSbLFn5GYlzDMNKbNUG+kVU1u4UQKBgQC9
7fafSQP/8kBqFjhxWEqtKZElveUXfzaDGLekZbgyWkYLZYjxh2WOIgQ1KtA6eMtF
nL1jkho9gADEFykRH1U5tD9Rnljv7bqVGD6EgeUkcY1iwEzsLDP6w8v7M9gbaJEo
1TPjc+0GURkjGEmb3mh3rwMAe1lid5TELsZzJVljAQKBgQCtFC5jt8e83ZavJ5QI
O8uLmYw53sgKh0A0V6b0Gdtb43WFwjklq06x0UvmOIkWDGeInoG1++vni6kKs+fI
uL+2bEiLihRW+6dEVtkdLp7ubr8wGC8Khg79j8KGWFpk5UnnkgVFcNko1XyoXj5j
Mdcoc6fLHcNU3ck83PBHSXuahg==
-----END PRIVATE KEY-----
"#;

        let test_pub_key = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA2zHlYGzjiOrk/yY+Caj9
xUz9spgUxteY3DmXpDMYHmK/w+WJ+BvDmclanOVHz51HtZ2WMSDs+RupFbHE6DVT
36sr7MqfcKVdI2YlKTsBu87+rErhuHCF44NWQcnDTRGmC6pIuMcP8n3ovuOgx2J/
zGFO0oqZhkiIHAUVxIsb52VOFslJdfoX87E3FlHurO1A0f0i5vD55lACwGiockK6
OOr0kzFA/FtNXa3oZ8bZPjIyzj51iNtTe681ad33nCp/q2AWSVmYaGjCGnihYAEl
ardbtfWY7NbARYYmHoJEYf1DtL/MAzhDeZr+0bJc05GhJDSdQ41K3A2wHEGXrgHc
HQIDAQAB
-----END PUBLIC KEY-----"#;

        let mut jwt = ClientSignJwt::claims();
        jwt.iss(client_id)
            .sub(user_id)
            .aud("https://shuttle.pub/stellar")
            .exp(OffsetDateTime::now_utc() + Duration::days(90))
            .jti(ticket);
        let enc = jwt.encode(test_privkey)?;

        println!("{:?}", enc);

        let dec = enc.decode(test_pub_key)?;

        println!("{:#?}", dec);

        Ok(())
    }
}
