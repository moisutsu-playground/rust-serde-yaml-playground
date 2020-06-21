extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::prelude::*;

fn main() {
//     let s =
// "
// foo:
//     - list1
//     - list2
// bar:
//     - 1
//     - 2.0
// ";
//     let docs = YamlLoader::load_from_str(s).unwrap();

//     // Multi document support, doc is a yaml::Yaml
//     let doc = &docs[0];

//     // Debug support
//     println!("{:?}", doc);

//     // Index access for map & array
//     assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
//     assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

//     // Chained key/array access is checked and won't panic,
//     // return BadValue if they are not exist.
//     assert!(doc["INVALID_KEY"][100].is_badvalue());

//     // Dump the YAML object
//     let mut out_str = String::new();
//     {
//         let mut emitter = YamlEmitter::new(&mut out_str);
//         emitter.dump(doc).unwrap(); // dump the YAML object to a String
//     }
//     println!("{}", out_str);
    let mut f = File::open("sample.yml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    println!("{:?}", docs);
}
