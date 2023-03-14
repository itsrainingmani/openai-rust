use dotenv::dotenv;

pub fn setup() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
}
