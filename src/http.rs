extern crate reqwest;
use std::io::Read;
use std::mem;

pub struct HttpRequest{}

impl HttpRequest{
    pub fn new() ->HttpRequest{
        HttpRequest{}
    }
    pub fn get_request_json(&self,url: &str)-> &str{
        let mut resp = reqwest::get(url).unwrap();
        let mut s = String::new();
        resp.read_to_string(&mut s);
        return self.convert_to_statictype(s);
    }
    fn convert_to_statictype(&self,s: String) -> &str { // String convert to static str
        unsafe {
            let ret = mem::transmute(&s as &str);
            mem::forget(s);
            ret
        }
    }
}