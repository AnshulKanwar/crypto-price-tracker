use clap::Parser;

use crypto_price_tracker::Client;

#[derive(Parser)]
#[clap(version)]
struct Args {
    coin: String,

    /// Print 24hr price statistics
    #[clap(short, long)]
    all: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = Client::new();

    if args.all {
        client.statistics(args.coin).await;
    } else {
        client.price(args.coin).await;
    }
}
