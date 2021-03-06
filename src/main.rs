
#[macro_use]
extern crate serde_derive;

extern crate glob;
extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate version_compare;
extern crate protoc_rust;
extern crate protobuf;

mod rattletrap;
// mod protos;

use std::collections::HashMap;
use glob::glob;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;
use protoc_rust::Customize;
use std::path::PathBuf;


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
    let replay_path_full = com_path + "/" + replay_path;
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

fn recursive_glob<'a>(dir_pattern: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new(); 
    for entry in glob(&dir_pattern).unwrap() {
        match entry {
            Ok(path) => {
                files.push(path.to_str().unwrap().to_owned());
                }
            Err(e) => println!("{:?}", e),
        }
    }
    files
}

fn find_rattletrap_commands() -> HashMap<String, String> {
    let path_str: &str = "src/rattletrap/"; 
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

fn generate_protobuf() {
    let files = [
        "api/game.proto",
        "api/metadata/camera_settings.proto",
        "api/metadata/game_metadata.proto",
        "api/metadata/mutators.proto",
        "api/metadata/player_loadout.proto",
        "api/party.proto",
        "api/player.proto",
        "api/player_id.proto",
        "api/stats/ball_stats.proto",
        "api/stats/data_frame.proto",
        "api/stats/events.proto",
        "api/stats/game_stats.proto",
        "api/stats/player_stats.proto",
        "api/stats/stats.proto",
        "api/stats/team_stats.proto",
        "api/team.proto"
    ];
    protoc_rust::run(protoc_rust::Args {
	    out_dir: "src/protos",
	    input: &files,
	    includes: &["."],
	    customize: Customize {
            serde_derive: Some(true),
	        ..Default::default()
	    },
	}).expect("protoc");
}
fn main() {
    let proto_dir = std::path::Path::new("src/protos/game.rs");
    if !proto_dir.exists() {
        generate_protobuf();
    };
    json_replay("src/rattletrap/replay.replay");
}