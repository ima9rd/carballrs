extern crate glob;
extern crate reqwest;
extern crate serde_json;
extern crate version_compare;
use std::collections::HashMap;

mod rattletrap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;


fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}

pub fn json_replay (replay_path: &str)  {
    let rattletrap: HashMap<String, String> = find_rattletrap_commands();
    let com_path: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let replay_path_full = com_path + "\\" + replay_path;
    println!("{:#?}", replay_path_full);
    let output;
    if cfg!(target_os = "windows") {
    output = Command::new(&rattletrap["windows"])
        .arg("--compact")
        .arg("-i")
        .arg(&replay_path_full)
        .output()
        .expect("failed to execute process");
    } else if cfg!(target_os = "linux") {
        output = Command::new("sh")
        .arg(&rattletrap["linux"])
        .arg("--compact")
        .arg("-i")
        .arg(&replay_path_full)
        .output()
        .expect("failed to execute process");
    } else {
        output = Command::new(&rattletrap["osx"])
        .arg("--compact")
        .arg("-i")
        .arg(&replay_path_full)
        .output()
        .expect("failed to execute process");
    };
    let json_str: String = format!("{}", String::from_utf8_lossy(&output.stdout));
    let replay: serde_json::Value = serde_json::from_str(json_str.as_str()).unwrap();
    let replay_data = &replay["content"]["body"]["frames"];
    let properties = &replay["header"]["body"]["properties"]["value"];
    let replay_id = &properties["Id"]["value"];
    let map = &properties["MapName"]["value"];
    let name = &properties["ReplayName"];
    let match_type = &properties["MatchType"]["value"];
    let team_size = &properties["TeamSize"]["value"];
    let date_string = &properties["Date"]["value"]["str"];
    let replay_version = &properties["ReplayVersion"]["value"]["int"];
    let mut players: Vec<serde_json::Value> = Vec::new();
    let it = &properties["PlayerStats"]["value"]["array"].as_str();
    println!("{:#?}", properties);
}


fn find_rattletrap_commands() -> HashMap<String, String> {
    let path_str: &str = "rattletrap\\"; 
    let mut files: Vec<String> = Vec::new(); 
    rattletrap::check_version::scan_dir(path_str, &mut files);
    let mut commands: HashMap<String, String> = HashMap::new();
    for file in files {
        let f: Vec<&str> = file.split("-").collect();
        let suffix = f[f.len() - 1];
        if suffix.contains("windows") {
            commands.insert("windows".to_owned(), file.to_owned() as String);
        } else if suffix.contains("linux") {
            commands.insert("linux".to_owned(), file.to_owned() as String);
        } else if suffix.contains("osx") {
            commands.insert("osx".to_owned(), file.to_owned() as String);
        }
    }
    commands
}


fn main() {
    json_replay("replay.replay");
}