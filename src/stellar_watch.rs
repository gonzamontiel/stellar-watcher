use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://horizon-testnet.stellar.org/";
const TIME_PERIOD_MS: u64 = 5000;

#[derive(Deserialize, Debug)]
pub struct Transaction {
    id: String,
    successful: bool,
    hash: String,
    ledger: u32,
    created_at: String,
    source_account: String,
    source_account_sequence: String,
    fee_account: String,
    fee_charged: String,
    max_fee: String,
    operation_count: u32,
    envelope_xdr: String,
    result_xdr: String,
    result_meta_xdr: String,
    fee_meta_xdr: String,
    memo_type: String,    
}

fn build_api_url(mut path: String) -> Option<Url> {
    if !path.is_empty() {
        path = "accounts/".to_owned() + path.as_str() + "/";
    }
    Url::parse(&*format!("{}{}{}", BASE_URL, path, "transactions")).ok()
}

fn filter_shown_transactions(transactions: &Vec<Transaction>) -> &Vec<Transaction> {
    transactions
}

pub fn start(address: Option<String>, watch: bool) {
    let client = Client::new();
    let path = address.clone().unwrap_or(String::from(""));
    let url = build_api_url(path).unwrap();
    let mut n = 0;
    if address.is_some() && watch {
        println!("Watching for new transactions for {}", address.unwrap());
    } else if address.is_none() {
        println!("Watching for new transactions");
    }

    loop {
        n += 1;
        println!("Loop n°{}", n);
        let data = fetch_horizon_api(&client, &url);
        match data {
            Result::Ok(v) => {
                let res = extract_transactions(&v);
                match res {
                    Some(transactions) => {
                        let mut _transactions = &transactions.unwrap();
                        if watch {
                            _transactions = filter_shown_transactions(_transactions);
                        }
                        for t in _transactions {
                            println!("The transaction id is {}", t.id)
                        }
                    }
                    None => println!("The obtained data has an invalid JSON structure.")
                }
        },
            Result::Err(e) => println!("error {:#?}", e)
        }
        std::thread::sleep(std::time::Duration::from_millis(TIME_PERIOD_MS));
    }
}

fn extract_transactions(data: &serde_json::Value) -> Option<serde_json::Result<Vec<Transaction>>> {
    let parsed = data["_embedded"]["records"].as_array();
    let mut v = Vec::new();
    match parsed {
        Some(arr) => {
            for tr in arr {
                // An extra step: to_string -> from_str is needed
                // in order to own a value; otherwise tr is not owned
                // and can't be used as in serde_json::from_value()
                let s = serde_json::to_string(tr).ok()?;
                v.push(serde_json::from_str(&s).ok()?);
            } 
            Some(Ok(v))
        },
        None => None
    }
}


fn fetch_horizon_api(client: &Client, api_call: &Url) -> reqwest::Result<serde_json::Value> {
    println!("Fetching {}", api_call);
    let resp = client
        .get(api_call.as_str())
        .send()?
        .json::<serde_json::Value>();
    Ok(resp?) 
}