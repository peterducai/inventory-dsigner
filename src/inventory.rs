pub mod inventory {

    #[derive(Debug)]
    pub struct Inventory {
        pub name: String,
        pub hosts: String,
        pub groups: String,
        pub vars: String
    }

    pub fn load_file(filename: String) {
        println!("reading {} ........\n", filename);
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        println!("{}", data);
    }

//     // The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
//     fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//     where P: AsRef<Path>, {
//         let file = File::open(filename)?;
//         Ok(io::BufReader::new(file).lines())
//     }

    pub fn inventory_to_dirs() {
        println!("inv 2 dirs");
    }

    pub fn dirs_to_inventory() {
        println!("dirs 2 inventory");
    }
}