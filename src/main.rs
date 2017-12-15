extern crate reqwest;
extern crate dotenv;

use dotenv::dotenv;
// use std::io::Read;
use std::env;
use std::time::{Duration};
use std::thread::sleep;
use reqwest::{Url};

fn get_uri_status(uri: &str) -> u16 {
    let url = Url::parse(uri);
    match url {
        Err(why) => panic!("{:?}", why),
        Ok(url) => {
            println!("Checking: {:?}", url.to_string());
            let resp = reqwest::get(url.as_str());
            let status = resp.unwrap().status();
            println!("Response: {}", status);
            return status.as_u16();
        }
    }
}

// another function that will alert someone, maybe slack webhook.
fn notify() {
    println!("Bummer. Failed to acquire an OK status from the requested site.");
}

fn successful_status(status: u16) -> bool {
    match status {
        200 => return true,
        201 => return true,
        202 => return true,
        _ => return false
    }
}

fn main() {
    dotenv().ok();

    let uri = env::var("URL").unwrap();
    let mut attempts = 0;
    const MAX_ATTEMPTS: i32 = 5;

    while !successful_status(get_uri_status(&uri)) {
        println!("The requested site did not respond with a successful status code, rechecking in 60 seconds.");
        sleep(Duration::from_secs(60));
        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            notify();
            break;
        }
    }
}
