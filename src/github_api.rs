use std::mem;

const API_BASE_URL:&str = "https://api.github.com/";
const GITHUB_USER_NAME:&str = "Your user name";
const API_TOKEN:&str = "Your token";
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
        return self.url_with_param("/starred");
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
