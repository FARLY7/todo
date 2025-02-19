// use std::env;

struct TodoEntry {
    name: String,
    done: bool
}

struct TodoList {
    name: String,
    entries: Vec<TodoEntry>
}
 
impl TodoList {

    fn print(&self) {
        println!("{}", self.name);
        for entry in &self.entries {
            if entry.done {
                println!("[x] {}", entry.name);
            } else {
                println!("[ ] {}", entry.name);
            }
            
        }
    }
}

// impl std::fmt::Display for TodoList {    
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
//         // let mut out = String::new();
//         // out.push_str(&self.name);
//         write!(fmt, "{}", &self.name)
//     }
// }


fn main()
{
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // println!("Args: {}", args.len());

    let shopping_list = TodoList {
        name: "Shopping".to_string(),
        entries: vec![
            TodoEntry { name: "milk".to_string(),  done: true  },
            TodoEntry { name: "bread".to_string(), done: false }
        ]
    };

    let chores_list = TodoList {
        name: "Chores".to_string(),
        entries: vec![
            TodoEntry { name: "dishes".to_string(),  done: false  },
            TodoEntry { name: "washing".to_string(), done: false }
        ]
    };


    let todo_lists: Vec<TodoList> = vec![shopping_list, chores_list];

    for list in todo_lists {
        list.print();
    }
    
    // let arg1 = args.get(1);
    // match arg1 {
    //     Some(arg1) => println!("Arg 1: {arg1}"), 
    //     None => println!("Arg 1: None"),
    // }

    // let arg2 = args.get(2);
    // match arg2 {
    //     Some(arg2) => println!("Arg 2: {arg2}"), 
    //     None => println!("Arg 2: None"),
    // }
 
}
