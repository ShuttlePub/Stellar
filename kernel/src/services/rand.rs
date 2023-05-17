use rand::distributions::{Alphanumeric, Distribution};

pub struct RandomizeService;

impl RandomizeService {
    pub fn gen_str<F, T>(len: usize, func: F) -> T
        where F: Fn(String) -> T
    {
        let gen = Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(len)
            .map(char::from)
            .collect::<String>();
        func(gen)
    } 
}


#[cfg(test)]
mod tests {
    use super::RandomizeService;

    #[derive(Debug)]
    struct TestDomain(String);

    impl TestDomain {
        pub fn new(value: impl Into<String>) -> Self {
            Self(value.into())
        }
    }

    #[test]
    fn test_randomize() -> anyhow::Result<()> {
        let domain = RandomizeService::gen_str(64, TestDomain::new);
        println!("{:?}", domain);
        Ok(())
    }
}