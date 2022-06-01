use serde;
use serde::Deserialize;

pub struct Client {
    client: reqwest::Client,
    url: String,
}

impl Client {
    pub fn new() -> Client {
        let client = reqwest::Client::builder().build().unwrap();

        let base_url = "https://api.binance.com";
        let url = String::from(base_url.to_owned() + "/api/v3/ticker/price");

        Client { client, url }
    }

    pub async fn ohlcv(&self, symbol: String) {
        let res = self
            .client
            .get(&self.url)
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json::<Price>()
            .await
            .unwrap();

        println!("{} is trading at {}", res.symbol, res.price);
    }
}

#[derive(Deserialize, Debug)]
struct Price {
    symbol: String,
    price: String,
}
