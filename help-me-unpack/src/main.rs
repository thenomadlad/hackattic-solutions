
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate tokio;
extern crate rand;
extern crate sha2;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use rand::random;
use sha2::{Sha256, Digest};
use std::time::{Instant};

#[derive(Serialize, Deserialize, Debug)]
struct Dough {
    bytes: String
}

#[derive(Serialize, Debug)]
struct Bread {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64
}


async fn get_problem_data() -> Result<Dough, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://hackattic.com/challenges/help_me_unpack/problem?access_token=943c7463da86a0bf")
        .await?
        .json::<Dough>()
        .await?;

    Ok(resp)
}

async fn submit(nonce: usize) -> Result<Bread, Box<dyn std::error::Error>> {
    let bread = Bread { nonce };

    let client = reqwest::Client::new();
    let res = client.post("https://hackattic.com/challenges/help_me_unpack/solve?access_token=943c7463da86a0bf")
        .json(&bread)
        .send()
        .await?
        .json::<Value>()
        .await;

    println!("Submitting: {:?}", res);
    Ok(bread)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
