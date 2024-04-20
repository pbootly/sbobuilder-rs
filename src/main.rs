use std::collections::HashMap;

use sbobuilder_rs::{info_file_parser::SboInfo, packages::build_package_tree};

fn main() {
    let path = "./slackbuilds";
    let packages: HashMap<String, SboInfo> = HashMap::new();
    let tree = build_package_tree(path, "i3", &packages);
    println!("{:?}", tree);
}
