use clap::Parser;

use crypto_price_tracker::Client;

#[derive(Parser, Debug)]
struct Args {
    coin: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = Client::new();

    client.ohlcv(args.coin).await;
}
