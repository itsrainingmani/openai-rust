pub mod open_ai_api {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Config {
        pub key: String,
        pub organization: Option<String>,
    }

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

        #[tokio::main]
        pub async fn authenticate(&self) -> Result<(), Box<dyn std::error::Error>> {
            let resp = reqwest::get("https://httpbin.org/ip")
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            println!("{:#?}", resp);
            Ok(())
        }
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
