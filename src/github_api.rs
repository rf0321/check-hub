#[allow(unused_must_use)]
#[warn(unused_must_use)]
use std::mem;
use std::fs;
use std::fs::File;
use std::io::Read;



const API_BASE_URL:&str = "https://api.github.com/";

pub fn get_token(filename: &str)->&str{
    let mut token = String::new();
    File::open(filename).unwrap().read_to_string(&mut token).unwrap();
    string_to_static_str(token)
}

pub fn get_name(filename: &str)->&str{
    let mut username = String::new();
    File::open(filename).unwrap().read_to_string(&mut username).unwrap();
    string_to_static_str(username)
}

fn string_to_static_str(s: String) -> &'static str { //  ReadLine's String convert to static str
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
pub struct GithubAPI{}

impl GithubAPI{
    pub fn new() -> GithubAPI{
        GithubAPI {}
    }
    
    pub fn profile(&self)-> &str{
        let username = get_name("NameFile");
        let token = get_token("TokenFile");
        let url = format!(
            "{}{}{}{}{}",API_BASE_URL,"users/",username,"?accesstoken=",token
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
