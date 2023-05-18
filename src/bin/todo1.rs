use std::io::{Error,BufRead};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct TodoList {
    items: Vec<TodoItem>,
    current_id: usize,
    dirty: bool
}

#[derive(Debug,Serialize,Deserialize)]
struct TodoItem {
    id: usize,
    content: String
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]);
    let default_todo_list = TodoList{
        items: Vec::new(),
        current_id: 1,
        dirty: false
    };
    let mut todo_list = match file {
        Ok(f) => match serde_json::from_reader(f) {
            Ok(data) => data,
            Err(_) => default_todo_list
        },
        Err(_) => default_todo_list
    };
    
    println!("Choose action from set [add, list, del]");
    let input = std::io::stdin().lock();

    for line in input.lines() {
        handle(&mut todo_list, line, &args[1]);
    }
}



fn handle(todo_list: &mut TodoList, line: Result<String, Error>, data_file: &String) {
    match line {
        Ok(line) => handle_line(todo_list, line),
        Err(x) => panic!("{}", x)
    }
    if todo_list.dirty {
        let file = std::fs::File::create(data_file).unwrap();
        serde_json::to_writer(file, &todo_list).unwrap();
    }
}

fn handle_line(todo_list: &mut TodoList, line: String) {
    match line.split_once(" ") {
        Some(("add", x)) => todo_list.add_item(x.to_string()),
        Some(("list", _)) => todo_list.list_items(),
        Some(("del", x)) => todo_list.delete_item(x.to_string()),
        Some((cmd, _)) => panic!("Unknown command {}", cmd),
        None => panic!("No space in command input!")
    }
}

impl TodoList {
    fn add_item(&mut self, line: String) {
        println!("Adding item: {}", line);
        let id = self.current_id;
        self.current_id += 1;
        self.items.push(TodoItem {
            id,
            content: line
        });
        self.dirty = true;
    }

    fn list_items(&mut self) {
        for it in self.items.iter() {
            println!("Item {}: {}", it.id, it.content);
        }
    }

    fn delete_item(&mut self, line: String) {
        match line.parse::<usize>() {
            Ok(x) => { 
                let note = self.items.remove(x);
                println!("Removed note {}: {}", x, note.content);
                self.dirty = true;
            },
            Err(e) => println!("Error occured when parsing note number for deletion {} ", e)
        }
    }

}
