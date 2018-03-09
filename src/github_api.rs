use std::env;
use std::mem;
const API_BASE_URL:&str = "https://api.github.com/";

pub struct GithubAPI{}

impl GithubAPI{
    pub fn new() -> GithubAPI{
        GithubAPI {}
    }
    pub fn profile(&mut self)-> &str{
        let url = format!(
            "{}{}{}{}{}",API_BASE_URL,"users/","ItinoseSan","?accesstoken=","77700e4de6d985ca3e0435dd0a8e7c1a175abef9"
        );
        return self.convert_to_statictype(url);
    }
    fn convert_to_statictype(&self,s: String) -> &str { // String convert to static str
        unsafe {
            let ret = mem::transmute(&s as &str);
            mem::forget(s);
            ret
        }
    }
}