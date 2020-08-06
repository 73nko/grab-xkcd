use anyhow::Result;

mod client;
mod cli;


const BASE_URL: &str = "https://xkcd.com";


fn main() -> Result<()>{
    let args = cli::Args::parse();
    let client = client::XkcdClient::new(args);
    client.run();
}
