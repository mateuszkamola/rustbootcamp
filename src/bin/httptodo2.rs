//use std::io::{Error,BufRead};
use serde::{Serialize,Deserialize};
use rocket::State;
#[macro_use] extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    let todo_list = load_todo_list();
    rocket::build()
        .manage(todo_list)
        .mount("/", routes![list])
}

#[get("/notes")]
fn list(todo_list_state: &State<TodoList>) -> String {
    serde_json::to_string::<TodoList>(todo_list_state).unwrap()
}

fn load_todo_list() -> TodoList {
    let default_todo_list = TodoList{
        items: Vec::new(),
        current_id: 1,
        dirty: false
    };
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]);
    let todo_list = match file {
        Ok(f) => match serde_json::from_reader(f) {
            Ok(data) => data,
            Err(_) => default_todo_list
        },
        Err(_) => default_todo_list
    };
    return todo_list

}
