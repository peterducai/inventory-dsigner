extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut verbose = false;
    let mut name = "World".to_string();
    { // this block limits scope of borrows by ap.refer() method
let mut ap = ArgumentParser::new();
ap.set_description("Greet somebody.");
ap.refer(&mut verbose)
.add_option(&["-v", "--verbose"], StoreTrue,
"Be verbose");
ap.refer(&mut name)
.add_option(&["--name"], Store,
"Name for the greeting");
ap.parse_args_or_exit();
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
}



// mod inventory;

// use std::env;
// use crate::inventory::inventory::Inventory;

// fn main() {
//     println!("Inventory dSigner");
    
//     let mut invmain = Inventory{
//         name: String::from("example inventory"),
//         hosts: String::from("hosts"),
//         groups: String::from("groups"),
//         vars: String::from("vars")
//     };

//     //test
//     invmain.name = String::from("in1");
//     println!("{:?}", invmain);

//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let filename = &args[2];

//     println!("Searching for {}", query);
//     println!("In file {}", filename);
// }
