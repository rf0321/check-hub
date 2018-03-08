extern crate reqwest;
use std::io::Read;

fn api_request(url:&str){
    let mut resp = reqwest::get(url).unwrap();
    let mut s = String::new();
    resp.read_to_string(&mut s);
    println!("{:?}", s);
}

fn contribution_request(username:&str){
    let mut resp = reqwest::get("").unwrap();
    let mut s = String::new();
    resp.read_to_string(&mut s);
    println!("{:?}", s);
}