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
            aud: "".to_string(),
            exp: 0,
            jti: "".to_string(),
            iat: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }

    pub fn decode(self, pubkey: impl AsRef<[u8]>) -> Result<ClientDecodeJwt, KernelError> {
        let key = DecodingKey::from_rsa_pem(pubkey.as_ref())?;
        let val = Validation::new(Algorithm::RS512);
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
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDkEVd+zWLER7H/
XkcC8jHojBzlNj0QRDwyiGeIu+GTJdbYXUZ6a5gFdSJiQO/T0JhGhX0Q23LwOBL/
vPWZsW/oTzcIpzGfvvg0HJQRiixmBGfxFgultTyoXVd21dhl+TzA1tJZbEVFcVZ1
JbvzbZoLA46G2gX4aaXqEEqbgHD8PN6pZQkcTjEaEsruvzUiFPTx4ODQKemYyRxn
ES6NQcbuxN20++pFW81RsCAf4c1bc+EfCjPXLzdGRAT/NAz6etl7il7yL7Dm6tHJ
uY7r5LMozZDXLqA+iRjmG/DvWTvd5dgZGP1+jZNYbFc0BSkXp+n8Ki1tJ41Wc1cU
wd2P/KudAgMBAAECggEABozs9hUQwichA1RXi8vIXS/btfBR3qwRt+MtE236CPpB
lSNQs+bFOLA9lZQx5I6Ni8ZSakUzTuz9YV+xr4FmKnD5WVlGq++V52vutiDsLN3x
h6jPcCn9clnZT5jNcgME7gZS3QNRa1uEVTGz7UZM8gYJy5vAJ3tkADi0O9/Q3vMU
DbZNtFInjONlmE7HenuVG5JHEPDMnGGa2gWEyFjPqqoQl2WkC3tQTgiNHKWqnb6G
IlqS7S57wfzINBB0xyATL0ccALII7VCtG0oXD9/5X/knJngOyIoOR01plfpAijQ6
LKghxzBdnenzSKUYiQp7pwlzdbkUX8dPVSXEmqLqYQKBgQD98bDD4RT/LeE8d8YY
DbCgO+JxzGY5vK/XJt1lmgGpnhCqVFLlujkuckyqdqYmbR6RBv46LlUoP6cawEI9
U6ookHPkIopRMqoOupumnehVdmtGAtOgUc45GPYP4c+QYj872nNsGd5aoxip58Sp
GES0nrEqNj7TfDdacAfFeifBuQKBgQDl6gV/T0e4JnRnSopYy0Kc6CBBslFe6hPY
F6s4iPPcd/RElkIvmA1MIvSZRDbPlbtgND0f0Y0XE5h936dk/iXr/FHrZsimSV1P
aszkul+/wtBM917F/2JRFlA8cCksquuaLNWQCPRhyb2YXQToJ/VzoN64KdMt4Yfc
Tbe5pTJ7BQKBgFONgSaB2UG4m8IkenYRkwq1iWT95qiaj5SPwgqa6G8hhcQ1KG1T
n1rL2rO/hB1ii8sV7PHBqt0qTFX96g15iKP+G1N1leLvc4qTFYF1tSrhYpxPc8ft
AphAwT0qg/uyh7Gux4lK3aEexo+opUkppflzuUD1RiiZVzEXaUctUQVpAoGBAI9y
1f86veq9w3722R7Rozw28HlNAswPjFYVXnUlrdFu3m80uRLs8c8BOE+waepW59P8
g+6oVjmBtnFC7DRtBcgZZBtVw/dMavW0EybaygbhTAnZhb5Pu04Qd/tdl7MQ6XMG
ajE2BRRGRQ8daoRW+iQsyQlqvyIvRw9DRlCEK9zpAoGBAOt5b+dv9QfoAUjxx1dH
bg+skddR4yECjixizTZW0MzmpJX+XyjEadxGzfRNFUigrOKBPKDZszATfHjkTPEW
jkVDCHoCcH933Jm9OoyDBipOcS+3nJXICmTYJbQctnV39ifHaHc33vfPwD30QCbp
JlWfCE3u+4pT68/Fc8kOq6oT
-----END PRIVATE KEY-----"#;

        let test_pub_key = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5BFXfs1ixEex/15HAvIx
6Iwc5TY9EEQ8MohniLvhkyXW2F1GemuYBXUiYkDv09CYRoV9ENty8DgS/7z1mbFv
6E83CKcxn774NByUEYosZgRn8RYLpbU8qF1XdtXYZfk8wNbSWWxFRXFWdSW7822a
CwOOhtoF+Gml6hBKm4Bw/DzeqWUJHE4xGhLK7r81IhT08eDg0CnpmMkcZxEujUHG
7sTdtPvqRVvNUbAgH+HNW3PhHwoz1y83RkQE/zQM+nrZe4pe8i+w5urRybmO6+Sz
KM2Q1y6gPokY5hvw71k73eXYGRj9fo2TWGxXNAUpF6fp/CotbSeNVnNXFMHdj/yr
nQIDAQAB
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
