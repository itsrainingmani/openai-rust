mod open_ai_api {
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

    pub struct Config {
        pub key: String,
        pub organization: String,
    }

    struct Models {}

    struct Completions {}
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let org = String::from("orgstring");
        let key = String::from("keystring");
        let client = open_ai_api::Client::new(org, key);
        assert_eq!(client.config.key, String::from("keystring"));
    }

    #[test]
    fn test_config_org_and_key_validation() {
        let org = String::from("orgstring");
        let key = String::from("keystring");
        let client = open_ai_api::Client::new(org, key);
        assert_eq!(client.config.key, String::from("keystring"));
    }
}
