use ::glob::glob;
use ::version_compare::{Version, VersionCompare};

const URL: &str = "https://api.github.com/repos/tfausak/rattletrap/releases/latest";
const RATTLETRAP_PATH: &str = "src/rattletrap/";

fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url)?.text()?;
    Ok(res)
}

fn parse_json(s: String) -> Result<serde_json::Value, serde_json::Error> {
    let j: serde_json::Value = serde_json::from_str(&s[..])?;
    Ok(j)
}

fn scan_dir(s: &str, files: &mut Vec<String>) {
    let dir: String = s.to_owned() + "*";
    for entry in glob(&dir).unwrap() {
        match entry {
            Ok(path) => {
                if path.extension().unwrap() != "rs" {
                    files.push(path.to_str().unwrap().to_owned());
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn version_from_fname(fname: String) -> String {
    let s: String = fname.to_owned();
    let str_vec: Vec<&str> = s.split("-").collect();
    let ver_str = str_vec[str_vec.len() - 2];
    let ver: Option<Version> = Version::from(ver_str);
    ver.unwrap().as_str().to_owned()
}

pub fn check_version() {
    let s: String = fetch_url(URL).unwrap();
    let j: serde_json::Value = parse_json(s).unwrap();
    let github_ver: &str = j["name"].as_str().unwrap();
    let mut files: Vec<String> = Vec::new();
    scan_dir(RATTLETRAP_PATH, &mut files);
    for file in files {
        let f: String = file.to_owned();
        let mut ver_str: String = version_from_fname(f).as_str().to_owned();
        if ver_str.chars().count() > 0 {
            if VersionCompare::compare_to(
                &github_ver,
                ver_str.as_str(),
                &version_compare::CompOp::from_sign(">").unwrap(),
            ).unwrap()
            {
                println!("Update available!");
                println!("GitHub version: {}", github_ver);
                println!(
                    "Current version: {}",
                    Version::from(ver_str.as_str()).unwrap().as_str()
                );
                break;
            }
        }
    }
}
