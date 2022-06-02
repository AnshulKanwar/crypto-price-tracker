use serde;
use serde::Deserialize;
use std::fmt;

pub struct Client {
    client: reqwest::Client,
    url: String,
}

impl Client {
    pub fn new() -> Client {
        let client = reqwest::Client::builder().build().unwrap();

        let base_url = "https://api.binance.com";
        let url = String::from(base_url);

        Client { client, url }
    }
}

impl Client {
    pub async fn statistics(&self, symbol: String) {
        let url = self.url.clone() + "/api/v3/ticker/24hr";

        let res = self
            .client
            .get(&url)
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json::<Statistics>()
            .await
            .unwrap();

        println!("{}", res);
    }

    pub async fn price(&self, symbol: String) {
        let url = self.url.clone() + "/api/v3/ticker/price";

        let res = self
            .client
            .get(&url)
            .query(&[("symbol", symbol)])
            .send()
            .await
            .unwrap()
            .json::<Price>()
            .await
            .unwrap();

        let price: f32 = res.price.trim().parse().unwrap();

        println!("{} is trading at ${price:.*}", res.symbol, 2, price = price);
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Statistics {
    symbol: String,
    price_change: String,
    price_change_percent: String,

    open_price: String,
    high_price: String,
    low_price: String,

    volume: String,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
Symbol: {},
Price Change: ${},
Price Change Percent: {}%,

Open Price: ${},
High Price: ${}
Low Price: ${},

Volume: {}
        ",
            self.symbol,
            self.price_change,
            self.price_change_percent,
            self.open_price,
            self.high_price,
            self.low_price,
            self.volume
        )
    }
}

#[derive(Deserialize, Debug)]
struct Price {
    symbol: String,
    price: String,
}
