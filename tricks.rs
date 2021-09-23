    
    
    let mut v = vec!["a", "b", "a"];
    v.sort_unstable();
    v.dedup();
    
    
    //-------------------------------------------------------------------------
    
    #[derive(Debug)]
    struct Employee {
        firstname: String,
        lastname: String,
    }

    let mut emp1 = Employee {
        firstname: String::from("NoName"),
        lastname: String::from("NoName"),
    };

    emp1.firstname = String::from("Tattva");
    emp1.lastname = String::from("Hegde");

    let mut emp_db: Vec<Employee> = Vec::new();
    emp_db.push(emp1);

    let x = &emp_db[0];

    println!("Hello, world! {:#?}", x);

    OR

    let j = vec![data{g: 3, h: 4}, data{g: 99, h: 42}];

    #[derive(Clone)]
struct Data { ... }

let mut v = vec![];
for x in 0..100 {
    v.push(Data { g: x, h: x });
}

If the Datas are same.

let v = vec![Data { g: 0, h: 0 }; 100];