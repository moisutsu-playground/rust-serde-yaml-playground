extern crate yaml_rust;

use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut f = File::open("sample.yml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];
    let output = match &doc["output"] {
        Yaml::String(s) => s,
        _ => "default",
    };
    println!("{}", output);
    let threshold = match &doc["threshold"] {
        Yaml::String(s) => s,
        _ => "default",
    };
    println!("{}", threshold);
}
