use std::io::Write;
use std::process;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

pub struct Todo {

}

impl Todo {

    pub fn new() -> Result<Self, String> {

        /* Attempt to open todo file. If doesn't exist, then create one */
        let mut todo_file = match File::options().append(true).open("todo.txt") {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create("todo.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("Failed to create todo file {e:?}")
                }
                _ => panic!("Failed to open todo file")
            }
        };

        let bytes_written = todo_file.write(b"Some bytes").unwrap();
        println!("Bytes written {bytes_written}");

        /* Return Todo object */
        Ok(
            Todo{}
        )
    }

    pub fn list(&self) {

        let contents = fs::read_to_string("todo.txt");
        
        match contents {
            Ok(c) => println!("{}", &c),
            Err(result) => println!("Error when opening file (err={})", result)
        }
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