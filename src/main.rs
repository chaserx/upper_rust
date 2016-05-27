extern crate hyper;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::io::Read;
use hyper::{Client};
use hyper::header::{Authorization};

fn main() {
    dotenv().ok();

    let client = Client::new();
    let uri = env::var("URL").unwrap();
    let auth_token = env::var("AUTH").unwrap();

    println!("Getting... {}", uri);
    let mut res = client.get(&uri)
                        .header(Authorization(auth_token))
                        .send()
                        .unwrap();

    assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{:?}", s);
}
