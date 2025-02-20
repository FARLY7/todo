use std::env;
use todo::{help, Todo};

fn main()
{
    let todo = Todo{};
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "help" | "--help" | "-h" | _ => help()
        }
    }
}
