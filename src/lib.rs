pub mod open_ai_api {

    #[derive(Debug)]
    pub struct Client {
        pub config: Config,
    }

    impl Client {
        /// Creates a new Client given a secret API key
        pub fn new(key: String) -> Self {
            let config = Config {
                key,
                organization: None,
            };

            Client { config }
        }
        /// Creates a new Client given an organization and the secret API key
        pub fn new_with_org(key: String, organization: String) -> Self {
            let config = Config {
                key,
                organization: Some(organization),
            };

            Client { config }
        }
    }

    #[derive(Debug)]
    pub struct Config {
        pub key: String,
        pub organization: Option<String>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let key = String::from("keystring");
        let client = open_ai_api::Client::new(key);

        println!("{:?}", client);
    }
}
