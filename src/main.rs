extern crate request;
extern crate serde_json;
extern crate serde;
extern crate regex;
extern crate rand;

#[macro_use]
extern crate serde_derive;

use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;
use rand::Rng;
use std::process::Command;

#[derive(Serialize,Deserialize)]
struct TitleList{
    result: Vec<String>,
}

fn get_titles() -> Vec<String>{
    let url = "http://www.onsen.ag/api/shownMovie/shownMovie.json";
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Connection".to_string() , "close".to_string());

    let res = match request::get(&url, &mut headers){
        Ok(res) => res,
	Err(e) => { println!("{}",e); return vec![] }
    };
    let v: TitleList = serde_json::from_str(res.body.as_str()).unwrap();
    v.result
}

fn get_mp3_url(title: &str) -> String{
    let mut url = "http://www.onsen.ag/data/api/getMovieInfo/".to_string();
    url.push_str(title);
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Connection".to_string() , "close".to_string());

    let res = match request::get(&url, &mut headers){
        Ok(res) => res,
	Err(e) => { println!("{}",e); return String::new();}
    };
    let re = Regex::new(r"(\{.*\})").unwrap();
    let json = re.captures(res.body.as_str()).unwrap().at(0).unwrap();
    let v: Value = serde_json::from_str(json).unwrap();
    let mut url = v["moviePath"]["pc"].to_string();
    url.remove(0);
    url.pop();
    url
}

fn get_mp3_urls() -> Vec<String>{
    let v: Vec<_> = get_titles().iter().map(|t| get_mp3_url(t)).collect();
    let v: Vec<String> = v.iter().filter( |s| s.contains("mp3")).map(ToOwned::to_owned).collect();
    v
}

fn main() {
    let mp3_urls = get_mp3_urls();
    let mp3_url = rand::thread_rng().choose(&mp3_urls).unwrap();
    Command::new("mplayer").arg(mp3_url).spawn().expect("mplayer command failed to start");
}
