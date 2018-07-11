extern crate clap;
extern crate reqwest;

mod github_api;
mod http;
mod json;
use clap::{ App, Arg };
use github_api::GithubAPI;
use http::HttpRequest;
use json::JSON;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

pub struct Checkhub{}

impl Checkhub{
    pub fn new()->Checkhub{
        Checkhub{}
    }
    pub fn run(&self){
        let tool = App::new("checkhub")
        .version("0.0.1")
        .author("Itinose <dhelitus@gmail.com>")
        .about("CLI tool which can check github user informations")
        .usage("./check-hub [INFO NAME]")
        .arg(Arg::with_name("INFO NAME")
        .help("GitHub user informations(current version supporting)
        - name 
        - login
        - bio 
        - gist-count 
        - follow-count
        - follower-count
        - location") 
       .required(true)          
    );
    self.parse_argument(tool);      
    }                                                    
    fn parse_argument(&self,tool: App){
        let github = GithubAPI::new();
        let client = HttpRequest::new();
        let url = github.profile();
        let json = client.get_request_json(url);
        let json_decoder = JSON::new();
        let maches = tool.get_matches();
      
        if let Some(arg) = maches.value_of("INFO NAME"){
            if arg.starts_with("bio"){
                json_decoder.bio(json);
            }else if arg.starts_with("login"){
                json_decoder.login(json);
            }else if arg.starts_with("name"){
                json_decoder.name(json);
            }else if arg.starts_with("gist-count"){
                json_decoder.gist(json);
            }else if arg.starts_with("follow-count"){
                json_decoder.follow_count(json);
            }else if arg.starts_with("follower-count"){
                json_decoder.follower_count(json);
            }else if arg.starts_with("location"){
                json_decoder.location(json);
            }else{
                println!("Invaild argument value.");
            }
        }
    }
}
fn main(){
    let checkhub = Checkhub::new();
    checkhub.run();
}

#[test]
fn test_parse_config(){
    let yaml = github_api::get_config_yamlfile("apiconfig.yml");
    let docs = YamlLoader::load_from_str(yaml).unwrap();
    let doc = &docs[0];
    println!("{}",yaml);
    let result = doc["GITHUB_API_TOKEN"].as_str().unwrap();
    let result2 = doc["name"].as_str().unwrap();
    let str_result:&str = result;
    println!("Your API token={}",str_result);
    println!("Your github login name={}",result2);
}
