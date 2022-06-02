use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;



#[derive(Serialize, Deserialize, Debug)]
struct FtxFunding {
    success: bool,
    result: Vec<Ticker>,
}


#[derive(Serialize, Deserialize, Debug)]
struct Ticker {
    id: u32,
    liquidation: bool,
    price: f32,
    side: String,
    size: f32,
    time: String,
}

impl FtxFunding {
    async fn get(symbol: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://ftx.com/api/markets/{}/trades",
            symbol
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<FtxFunding>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    
    //let api_key = "YOUR API KEY".to_string();
    let args: Vec<String> = env::args().collect();
    let mut symbol: String = "BTC/USD".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a company symbol, it has defaulted to bitcoin.");
    } else {
        symbol = args[1].clone();
    }

    let res = FtxFunding::get(&symbol).await?;
    println!("{}'s current stock price: {:#?}", symbol, res.result[0].price);

    Ok(())
}

