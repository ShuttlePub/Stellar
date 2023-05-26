use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use kernel::entities::{Account, Client, ClientId, UserId};
use super::{GenIds, Config, constants::{CONFIG, GENNED, CACHED}};
use crate::DriverError;

pub fn generate(path: impl AsRef<Path>) -> Result<Option<(Account, Client)>, DriverError> {
    let path = path.as_ref();

    if path.join(CONFIG).as_path().exists()
        && path.join(GENNED).as_path().exists() {
        return Ok(None)
    }

    let mut config = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.join(CONFIG).as_path())?;

    let mut genned = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.join(GENNED).as_path())?;

    let mut cached = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.join(CACHED).as_path())?;


    let generated = writein(&mut config, &mut genned, &mut cached)?;

    tracing::info!("+-------------------------------------------------------------------------------+");
    tracing::info!("* Config file generated.");
    tracing::info!("* It is strongly recommended to edit the default values as they are not secure.");
    tracing::info!("+-------------------------------------------------------------------------------+");

    Ok(Some(generated))
}

fn writein(
    config: &mut File,
    genned: &mut File,
    cached: &mut File
) -> Result<(Account, Client), DriverError> {
    let default = Config::default();

    let toml = toml::to_string(&default)?;

    config.write_all(toml.as_bytes())?;
    config.flush()?;

    let admin_id = UserId::default();
    let stellar_id = ClientId::default();

    let ids = GenIds::new(admin_id, stellar_id);
    let ids = rmp_serde::to_vec(&ids)?;

    genned.write_all(&ids)?;
    genned.flush()?;

    let mut origin = Vec::new();
    let _ = config.read_to_end(&mut origin)?;

    let hashed = blake3::hash(toml.as_bytes());

    cached.write_all(hashed.as_bytes())?;
    cached.flush()?;

    println!("{:?}", hashed.as_bytes());

    default.formed(admin_id, stellar_id)
}


#[cfg(test)]
mod tests {
    use super::generate;

    #[ignore = "Depend on file manipulation, this test will ignore."]
    #[test]
    fn gen() -> anyhow::Result<()> {
        generate("../")?;
        Ok(())
    }
}