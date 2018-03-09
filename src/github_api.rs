use std::mem;
use std::env;
const API_BASE_URL:&str = "https://api.github.com/";
const API_TOKEN:&str = "";
const GITHUB_USER_NAME:&str = "";
pub struct GithubAPI{}

impl GithubAPI{
    pub fn new() -> GithubAPI{
        GithubAPI {}
    }
    fn url_with_param(&self,param:&str)-> &str{
        let username = GITHUB_USER_NAME;
        let token = API_TOKEN;
        let url = format!(
            "{}{}{}{}{}{}",API_BASE_URL,"users/",username,param,"?accesstoken=",token
        );
        return self.convert_to_statictype(url);
    }
     pub fn profile(&self)-> &str{
        let username = GITHUB_USER_NAME;
        let token = API_TOKEN; 
        let url = format!(
            "{}{}{}{}{}",API_BASE_URL,"users/",username,"?accesstoken=",token
        );
        return self.convert_to_statictype(url);
    }
    pub fn follows(&self)->&str{
        return self.url_with_param("/following");
    }
    pub fn followers(&self)->&str{
        return self.url_with_param("/followers");
    }
    pub fn stars(&self)->&str{
        return self.url_with_param("/stars");
    }
    pub fn repository(&self) ->&str{
        return self.url_with_param("/repos");
    }
    fn convert_to_statictype(&self,s: String) -> &str { // String convert to static str
        unsafe {
            let ret = mem::transmute(&s as &str);
            mem::forget(s);
            ret
        }
    }
}

/*#[derive(Default)]
pub struct Config{
    pub username: String,
    token: String
}

impl Config{
    fn get_github_name(&mut self) -> &String {  &self.username }
    fn get_token(&mut self) -> &String { &self.token }

    pub fn set_username(&mut self, x: String) { self.username = x; }
    pub fn set_token(&mut self, x: String) { self.token = x; }
}*/