mod inventory;

use std::env;
use crate::inventory::inventory::Inventory;

fn main() {
    println!("Inventory dSigner");
    
    let mut invmain = Inventory{
        name: String::from("example inventory"),
        hosts: String::from("hosts"),
        groups: String::from("groups"),
        vars: String::from("vars")
    };

    //test
    invmain.name = String::from("in1");
    println!("{:?}", invmain);

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
