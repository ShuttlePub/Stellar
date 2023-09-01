use super::{
    constants::{CACHED, CONFIG, GENNED},
    model::Config,
    GenIds,
};
use crate::DriverError;
use kernel::prelude::entities::{Account, Client};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub fn load(path: impl AsRef<Path>) -> Result<Option<(Account, Client)>, DriverError> {
    let path = path.as_ref();

    let mut config = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path.join(CONFIG).as_path())?;

    let mut genned = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path.join(GENNED).as_path())?;

    let mut cached = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.join(CACHED).as_path())?;

    let Some(config) = checkin(&mut config, &mut cached)? else {
        return Ok(None)
    };

    let loaded = loadin(config, &mut genned)?;

    Ok(Some(loaded))
}

/// The hash information of the previous load data of config data is read
/// from the cache and compared to see if there is any difference in the information.
///
/// ### Successfully Return Value Kind
/// * [`Ok(None)`] -  if there is no difference in information and the data was loaded successfully
/// * [`Ok(Some(Config))`] - if there is a difference and the data was loaded successfully.
fn checkin(config: &mut File, cached: &mut File) -> Result<Option<Config>, DriverError> {
    let mut origin = Vec::new();
    let _ = config.read_to_end(&mut origin)?;

    let hashed = blake3::hash(&origin);

    println!("{:?}", hashed.as_bytes());

    let mut cache: Vec<u8> = Vec::new();
    let _ = cached.read_to_end(&mut cache)?;

    if cache.ne(hashed.as_bytes()) {
        cached.seek(SeekFrom::Start(0))?;
        cached.write_all(hashed.as_bytes())?;
        cached.flush()?;
        let config = toml::from_str::<Config>(
            origin
                .into_iter()
                .map(char::from)
                .collect::<String>()
                .as_str(),
        )?;
        return Ok(Some(config));
    }

    Ok(None)
}

fn loadin(config: Config, genned: &mut File) -> Result<(Account, Client), DriverError> {
    let mut ids: Vec<u8> = Vec::new();
    genned.read_to_end(&mut ids)?;

    let GenIds {
        admin_id,
        stellar_id,
    } = rmp_serde::from_slice(&ids)?;

    config.formed(admin_id, stellar_id)
}

#[cfg(test)]
mod tests {
    use super::load;
    use std::fs::File;
    use std::io::Read;

    #[ignore = "Depend on file manipulation, this test will ignore."]
    #[test]
    fn file_hash() -> anyhow::Result<()> {
        let mut file = File::open("../stellar.toml")?;
        let mut bin: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut bin)?;
        let hash = blake3::hash(&bin);
        let hash = hash.to_string();
        println!("{}", hash);
        Ok(())
    }

    #[ignore = "Depend on file manipulation, this test will ignore."]
    #[test]
    fn init() -> anyhow::Result<()> {
        load("../")?;
        Ok(())
    }
}
