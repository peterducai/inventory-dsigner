extern crate argparse;
use inventory_dsigner::inventory::inventory;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut verbose = false;
    // const version: String = "";
    const INTERNAL_VERSION: i32 = 1632059132;
    let mut version = "0.0.1 build 1632059132".to_string();
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbose)
        .add_option(&["-v", "--verbose"], StoreTrue,
        "Be verbose");
        ap.refer(&mut version)
        .add_option(&["--version"], Store,
        "version");
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("version is {}", version);
    }
    println!("Inventory dSigner {} build {} \"Ulotrichous\"", version, INTERNAL_VERSION);
    inventory::load_file("inventory".to_string());
    inventory::generate_sample_inventory();
    inventory::init_struct_inventory();
}



// mod inventory;

// use std::env;
// use crate::inventory::inventory::Inventory;

// fn main() {
//     println!("Inventory dSigner");
    
//     let mut invmain = Inventory{
//         version: String::from("example inventory"),
//         hosts: String::from("hosts"),
//         groups: String::from("groups"),
//         vars: String::from("vars")
//     };

//     //test
//     invmain.version = String::from("in1");
//     println!("{:?}", invmain);

//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let fileversion = &args[2];

//     println!("Searching for {}", query);
//     println!("In file {}", fileversion);
// }
