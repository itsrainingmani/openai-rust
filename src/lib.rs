mod open_ai_api {

    #[derive(Debug)]
    pub struct Client {
        pub config: Config,
    }

    impl Client {
        /// Creates a new Client given an organization and the secret API key
        pub fn new(organization: String, key: String) -> Self {
            let config = Config { key, organization };

            Client { config }
        }
    }

    #[derive(Debug)]
    pub struct Config {
        pub key: String,
        pub organization: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let org = String::from("orgstring");
        let key = String::from("keystring");
        let client = open_ai_api::Client::new(org, key);

        println!("{:?}", client);
    }

    #[test]
    fn test_config_org_and_key_validation() {
        let org = String::from("orgstring");
        let key = String::from("keystring");
        let client = open_ai_api::Client::new(org, key);

        assert_eq!(client.config.key, String::from("keystring"));
        assert_eq!(client.config.organization, String::from("orgstring"));
    }
}
