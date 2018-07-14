extern crate yaml_rust;
use yaml_rust::YamlLoader;

use std::mem;
use std::fs::File;
use std::io::Read;

const API_BASE_URL:&str = "https://api.github.com/";

pub fn get_config_yamlfile(filename: &str) ->&str{
    let mut file = String::new();
    File::open(filename).unwrap().read_to_string(&mut file).unwrap();
    string_to_static_str(file)
}
fn string_to_static_str(s: String) -> &'static str { 
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
        let yaml = get_config_yamlfile("apiconfig.yml");
        let load_yaml = YamlLoader::load_from_str(yaml).unwrap();
        let yaml_parser = &load_yaml[0];
        let name_result = yaml_parser["name"].as_str().unwrap();
        let token_result = yaml_parser["GITHUB_API_TOKEN"].as_str().unwrap();
        let  username :&str = name_result;
        let token :&str = token_result;

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