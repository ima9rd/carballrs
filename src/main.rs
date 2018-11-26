extern crate glob;
extern crate reqwest;
extern crate serde_json;
extern crate version_compare;

mod rattletrap;

fn main() {
    rattletrap::check_version::check_version();
}