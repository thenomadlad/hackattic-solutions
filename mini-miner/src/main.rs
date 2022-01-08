
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
struct Block {
    data: Vec<Value>,
    nonce: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dough {
    block: Block,
    difficulty: usize
}

#[derive(Serialize, Debug)]
struct Bread {
    nonce: usize
}


async fn get_problem_data() -> Result<Dough, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://hackattic.com/challenges/mini_miner/problem?access_token=943c7463da86a0bf")
        .await?
        .json::<Dough>()
        .await?;
    println!("Problem difficulty: {}", resp.difficulty);

    Ok(resp)
}

async fn submit(nonce: usize) -> Result<Bread, Box<dyn std::error::Error>> {
    let bread = Bread { nonce };

    let client = reqwest::Client::new();
    let res = client.post("https://hackattic.com/challenges/mini_miner/solve?access_token=943c7463da86a0bf")
        .json(&bread)
        .send()
        .await?
        .json::<Value>()
        .await;

    println!("Submitting: {:?}", res);
    Ok(bread)
}

fn check_hash(hash: &[u8], mut difficulty: usize) -> bool {
    let mut idx = 0;
    while difficulty > 8 {
        if hash[idx] != 0 {
            return false;
        }
        idx += 1;
        difficulty -= 8;
    }

    hash[idx] < (2 as u8).pow(8 - difficulty as u32)
}

#[tokio::main]
async fn main() {
    if let Ok(Dough { difficulty, mut block }) = get_problem_data().await {
        let mut solved = false;
        let now = Instant::now();
        
        while !solved {
            block.nonce = Some(random::<usize>());
            print!("nonce = {}\r", block.nonce.expect("how"));

            let mut hasher = Sha256::new();
            //println!("{}", serde_json::to_string(&block).unwrap());
            hasher.update(serde_json::to_string(&block).unwrap());
            let hash = hasher.finalize();
        
            solved = check_hash(hash.as_slice(), difficulty);
            if solved {
                println!(
                    "Found a nonce after {} ms!: {} -> {}",
                    now.elapsed().as_millis(),
                    block.nonce.expect("bruh"),
                    hash.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<String>()
                );
            }
        }

        
        submit(block.nonce.expect("hooow")).await.unwrap();
    }

}
