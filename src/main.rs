extern crate hyper;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::io::Read;
use hyper::{Client};
use hyper::header::{Authorization};
use std::time::{Duration};
use std::thread::sleep;

fn check_url(uri: &str, auth_token: &str) -> bool {
    let client = Client::new();
    println!("Getting... {}", uri);
    // maybe conditional assignment. with or without auth_token
    let mut res = if auth_token.len() == 0 {
                        client.get(uri).send().unwrap()
                  } else {
                        client.get(uri)
                              .header(Authorization(auth_token.to_owned()))
                              .send()
                              .unwrap()
                  };

    // probably need to switch this to a match statement to capture DNS failure
    // or myabe use the try! macro
    if hyper::Ok == res.status {
            let mut s = String::new();
            res.read_to_string(&mut s).unwrap();
        if String::from(env::var("VERBOSE_MODE").unwrap()) == "true" {
            println!("{:?}", s);
        }
        return true
    } else {
        return false;
    }
}

// another function that will alert someone, maybe slack webhook.
fn notify() {
    println!("bummer");
}

fn main() {
    dotenv().ok();

    let uri = env::var("URL").expect("URL must be set");
    let auth_token = env::var("AUTH")
                          .expect("URL must be set. At least an empty string.");

    let mut attempts = 0;
    const MAX_ATTEMPTS: i32 = 5;

    while !check_url(&uri, &auth_token) {
        println!("site not responding, rechecking...");
        sleep(Duration::from_secs(60));
        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            notify();
            break;
        }
    }
}
