extern crate hyper;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::io::Read;
use hyper::{Client};
use hyper::header::{Authorization};

// probably should make this return a boolean and take URL and AUTH arguments
fn check_url(uri: String, auth: String) {
    let client = Client::new();
    println!("Getting... {}", uri);
    let mut res = client.get(&uri)
                        .header(Authorization(auth))
                        .send()
                        .unwrap();

    assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{:?}", s);
}

fn main() {
    dotenv().ok();

    let uri = env::var("URL").unwrap();
    let auth_token = env::var("AUTH").unwrap();
    check_url(uri, auth_token);
}
