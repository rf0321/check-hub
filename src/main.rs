extern crate clap;
extern crate reqwest;

mod github_api;

use clap::{ App, Arg};
use github_api::GithubAPI;
use std::io::Read;
use std::mem;

fn main(){ 
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
    - contributions
    - follows
    - followers
    - repository
    - gist 
    - stars") 
    .required(true)          
    );
    parse_command(tool);                                                      
}

fn parse_command(tool: App){
    let mut github = GithubAPI::new();
    let maches = tool.get_matches();
    if let Some(arg) = maches.value_of("INFO NAME"){
        if arg == "name"{
          
        }
        else if arg == "login"{
        }
        else if arg == "bio"{
        }
        else if arg == "contributions"{
        }
        else if arg == "follows"{
        }
        else if arg == "followers"{
        }
        else if arg == "repository"{
        }
        else if arg == "gist"{
        }
        else if arg == "stars"{
        }
        else{
            println!("Undefined args of this tool :{} \nCheck  -h or --help command",arg);
        }
    }
}



fn send_request(url: &str){
    let mut client = reqwest::get(url).unwrap();
    let mut s = String::new();
    client.read_to_string(&mut s);
    println!("{:?}", s);
}