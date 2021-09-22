pub mod inventory {

    // #[derive(Debug)]
    // pub struct Inventory {
    //     pub name: String,
    //     pub hosts: String,
    //     pub groups: String,
    //     pub vars: String
    // }
    static mut INVENTORY: Vec<String> = Vec::new();
    
    pub fn init_struct_inventory() {
        let mut inven = Vec::new();
        inven.push("---".to_string());
        unsafe {
            INVENTORY.push("# GENERATED WITH INVENTORY DSIGNER".to_string());
            println!("\n- INVENTORY ------------\n{:?}", INVENTORY);
        }
        
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
        create_dir("group_vars");
        create_dir("host_vars");
    }

    fn create_dir(path: &str){
        let r = std::fs::create_dir_all(path);
        match r {
            Err(e) => println!("error creating {}: {}", path, e),
            Ok(_) => println!("created {}: OK", path),
        }
    }

    pub fn dirs_to_inventory() {
        println!("dirs 2 inventory");
    }

    pub fn write_inventory_to_file(filename: String) {
        let data = "#write_inventory_to_file data";
        std::fs::write(filename, data).expect("Unable to write file");
    }

    pub fn generate_sample_inventory() {
        let data = "# GENERATED WITH INVENTORY DSIGNER
#VARS
ntp_server: acme.example.org
database_server: storage.example.org

#HOSTS
[atlanta]
host1 http_port=80 maxRequestsPerChild=808
host2 http_port=303 maxRequestsPerChild=909

[webservers]
www[01:50].example.com

www[01:50:2].example.com # increments between sequence numbers
db-[a:f].example.com  # alphabetic range

#HOST WITH VARS
localhost              ansible_connection=local
other1.example.com     ansible_connection=ssh        ansible_user=myuser  ansible_port=5555 ansible_host=192.0.2.50

# GROUP VARS
[atlanta:vars]
ntp_server=ntp.atlanta.example.com
proxy=proxy.atlanta.example.com

# /etc/ansible/group_vars/raleigh # can optionally end in '.yml', '.yaml', or '.json'
# /etc/ansible/group_vars/webservers
# /etc/ansible/host_vars/foosball";
        std::fs::write("inventory", data).expect("Unable to write file");
    }

    pub fn sldkfj() {
        let v2 = vec![1; 10];
        println!("[{}]", v2.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "));
    }
}