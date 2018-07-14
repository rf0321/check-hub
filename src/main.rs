extern crate clap;
extern crate reqwest;

mod github_api;
mod http;
mod json;
use clap::{ App,SubCommand };
use github_api::GithubAPI;
use http::HttpRequest;
use json::JSON;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

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
        .help("GitHub user informations(current version supporting)
        - name 
        - login
        - bio 
        - gist-count 
        - follow-count
        - follower-count
        - location") 
        .subcommand(SubCommand::with_name("name"))
        .subcommand(SubCommand::with_name("login"))
        .subcommand(SubCommand::with_name("bio"))
        .subcommand(SubCommand::with_name("gist-count"))
        .subcommand(SubCommand::with_name("follow-count"))
        .subcommand(SubCommand::with_name("follower-count"))
        .subcommand(SubCommand::with_name("location"))        
        .get_matches();

        let github = GithubAPI::new();
        let client = HttpRequest::new();
        let url = github.profile();
        let json = client.get_request_json(url);
        let json_decoder = JSON::new();
        
        match tool.subcommand_name(){
            Some("name")           => { json_decoder.name(json); },
            Some("login")          => { json_decoder.login(json); },
            Some("bio")            => { json_decoder.bio(json); },
            Some("gist-count")     => { json_decoder.gist_count(json); },
            Some("follow-count")   => { json_decoder.follow_count(json); },
            Some("follower-count") => { json_decoder.follower_count(json); },
            Some("location")       => { json_decoder.location(json); },            
            _ => { println!("Error: You must input subcommand. Please check --help command");}, 
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
