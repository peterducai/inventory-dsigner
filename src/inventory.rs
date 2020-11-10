use std::fs;

pub mod inventory {

    #[derive(Debug)]
    pub struct Inventory {
        pub name: String,
        pub hosts: String,
        pub groups: String,
        pub vars: String
    }

    pub fn LoadFile() {
        // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    }
}


// [atlanta]
// host1 http_port=80 maxRequestsPerChild=808
// host2 http_port=303 maxRequestsPerChild=909
