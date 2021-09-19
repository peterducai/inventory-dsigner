pub mod inventory {

    #[derive(Debug)]
    pub struct Inventory {
        pub name: String,
        pub hosts: String,
        pub groups: String,
        pub vars: String
    }

    pub fn load_file(filename: String) {
        println!("In file {}", filename);
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
}


// [atlanta]
// host1 http_port=80 maxRequestsPerChild=808
// host2 http_port=303 maxRequestsPerChild=909

// [webservers]
// www[01:50].example.com

// www[01:50:2].example.com # increments between sequence numbers
// db-[a:f].example.com  # alphabetic range

// localhost              ansible_connection=local
// other1.example.com     ansible_connection=ssh        ansible_user=myuser  ansible_port=5555 ansible_host=192.0.2.50

// GROUP VARS
// [atlanta:vars]
// ntp_server=ntp.atlanta.example.com
// proxy=proxy.atlanta.example.com

// /etc/ansible/group_vars/raleigh # can optionally end in '.yml', '.yaml', or '.json'
// /etc/ansible/group_vars/webservers
// /etc/ansible/host_vars/foosball

// VARS

// ---
// ntp_server: acme.example.org
// database_server: storage.example.org