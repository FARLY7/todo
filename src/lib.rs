use std::process;



pub struct Todo {

}


impl Todo {

    pub fn list(&self) {
        println!("List command");
    }

    pub fn add(&self, tasks: &[String]) {
        if tasks.is_empty() {
            eprintln!("'todo add' takes at least 1 argument");
            process::exit(1);
        }

        for task in tasks {
            println!("Task: {}", &task)
        }
    }
}


const HELP_MENU: &str = "
Todo is a simple task organiser written in rust

Usage: todo [COMMAND] [ARGUMENTS]

Available commands:
    - list
        lists all tasks
    - add [tasks]
        Add new task(s)
        Example: todo add \"clean apartment\"
    - help
        Display this help menu
";

pub fn help()
{
    println!("{}", HELP_MENU);
}