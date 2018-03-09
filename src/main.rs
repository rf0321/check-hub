extern crate clap;
extern crate reqwest;

mod github_api;
mod http;
mod json;
use clap::{ App, Arg };
use github_api::GithubAPI;
use http::Http;
use json::JSON;


pub struct Checkhub{}

impl Checkhub{
    pub fn new()->Checkhub{
        Checkhub{}
    }
    pub fn run(&self){
        let tool = App::new("checkhub")
        .version("0.0.1")
        .author("Itinose <dhelitus@gmail.com>")
        .about("CLI tool which can check github user infomations")
        .usage("./check-hub [INFO NAME]")
        .arg(Arg::with_name("INFO NAME")
        .help("GitHub user infomations(current version supporting)
        - name 
        - login
        - bio 
        - follows
        - followers
        - repository
        - gist-count 
        - stars
        - location") 
       .required(true)          
    );
    self.parse_command(tool);      
    }                                                    
    fn parse_command(&self,tool: App){
        let github = GithubAPI::new();
        let client = Http::new();
        let json_decoder = JSON::new();
        let maches = tool.get_matches();
        if let Some(arg) = maches.value_of("INFO NAME"){
            if arg == "name"{
                let url = github.profile();
                let json = client.get_request_json(url);
                 json_decoder.name(json);
            }
            else if arg == "login"{
                let url = github.profile();
                let json = client.get_request_json(url);
                json_decoder.login(json);
            }
            else if arg == "bio"{
                let url = github.profile();
                let json = client.get_request_json(url);
                json_decoder.bio(json);
            }
            else if arg == "gist-count"{
                let url = github.profile();
                let json = client.get_request_json(url);
                json_decoder.gist(json);
            }
            else if arg =="location"{
                let url = github.profile();
                let json = client.get_request_json(url);
                json_decoder.location(json);
            }
            else if arg == "follows"{
                let url = github.follows();
                let json = client.get_request_json(url);
            }
            else if arg == "followers"{
                let url = github.followers();
                let json = client.get_request_json(url);
            }
            else if arg == "repository"{
                let url = github.repository();
                let json = client.get_request_json(url);
            }
            else if arg == "stars"{
                let url = github.stars();
                let json = client.get_request_json(url);
            }
           else{
               println!("Undefined args of this tool :{} \nCheck  -h or --help command",arg);
            }
        }
    }
}


fn main(){
    let checkhub = Checkhub::new();
    checkhub.run();
}