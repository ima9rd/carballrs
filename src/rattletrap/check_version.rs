extern crate reqwest;

const URL: &str = "https://api.github.com/repos/tfausak/rattletrap/releases/latest";

fn main() {
    let mut res = reqwest::get(URL);
}