extern crate serde_json;
use self::serde_json::{ Value, Error };

pub struct JSON{}

impl JSON{
    pub fn new()-> JSON{
        JSON{}
    }
    pub  fn login(&self,json: &str) -> Result<(), Error> {
        let parser: Value = serde_json::from_str(json)?;
        println!("Your github login id is {}",parser["login"]);
        Ok(())
    }
    pub  fn name(&self,json: &str) -> Result<(), Error> {
        let parser: Value = serde_json::from_str(json)?;
        println!("Your github name is {}", parser["name"]);
        Ok(())
    }
    pub fn bio(&self,json: &str) -> Result<(), Error> {
        let parser: Value = serde_json::from_str(json)?;
        println!("Your bio \n{}", parser["bio"]);
        Ok(())
    }
    pub fn location(&self,json: &str) -> Result<(), Error> {
        let parser: Value = serde_json::from_str(json)?;
        println!("Your location is {}",parser["location"]);
        Ok(())
    }
    pub fn gist(&self, json: &str)-> Result<(), Error>{
         let parser: Value = serde_json::from_str(json)?;
        println!("Your gist count is {}",parser["public_gists"]);
        Ok(())
    }
}