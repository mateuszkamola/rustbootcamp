//use std::io::{Error,BufRead};
use serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket::State;
use std::sync::{atomic::{AtomicUsize,Ordering},Mutex};
#[macro_use] extern crate rocket;

#[derive(Debug,Serialize,Deserialize)]
struct TodoList {
    items: Vec<TodoItem>,
    current_id: usize
}

struct TodoListState {
    items: Mutex<Vec<TodoItem>>,
    current_id: AtomicUsize,
    filestore: String
}

#[derive(Debug,Serialize,Deserialize,Clone)]
#[serde(crate = "rocket::serde")]
struct TodoItem {
    id: usize,
    content: String
}

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = std::env::args().collect();
    let todo_list = load_todo_list(&args[1]);
    let todo_list_state = TodoListState {
        items: Mutex::new(todo_list.items),
        current_id: AtomicUsize::new(todo_list.current_id),
        filestore: args[1].clone()
    };
    rocket::build()
        .manage(todo_list_state)
        .mount("/", routes![list,create])
}

#[get("/notes")]
fn list(todo_list_state: &State<TodoListState>) -> String {
    let items = todo_list_state.items.lock().unwrap().clone();
    let id = todo_list_state.current_id.load(Ordering::SeqCst);
    serde_json::to_string::<TodoList>(&TodoList{items,current_id:id}).unwrap()
}

#[post("/notes", format = "json", data="<todo_item>")]
fn create(todo_item: Json<TodoItem>, todo_list_state: &State<TodoListState>) -> String {
    let curr_id = todo_list_state.current_id.fetch_add(1, Ordering::SeqCst);
    let new_item = TodoItem {
        id: curr_id,
        content: todo_item.content.clone()
    };
    let mut items = todo_list_state.items.lock().unwrap();
    items.push(new_item);
    write_todo_list(&todo_list_state.filestore, &*items, todo_list_state.current_id.load(Ordering::SeqCst));
    format!("Added {}", todo_item.content)
}

fn write_todo_list(filestore: &String, items: &Vec<TodoItem>, id: usize) {
    let todo_list = TodoList {
        items: items.clone(),
        current_id: id
    };
    match std::fs::File::create(filestore) {
        Ok(f) => match serde_json::to_writer(f, &todo_list) {
            Ok(_) => println!("Serialized successfuly"),
            Err(x) => println!("Error when serializing {}", x)
        },
        Err(x) => println!("Error when serializing {}", x)
    };
    
}

fn load_todo_list(filestore: &String) -> TodoList {
    let default_todo_list = TodoList {
        items: Vec::new(),
        current_id: 1
    };
    let file = std::fs::File::open(filestore);
    let todo_list = match file {
        Ok(f) => match serde_json::from_reader(f) {
            Ok(data) => data,
            Err(_) => default_todo_list
        },
        Err(_) => default_todo_list
    };
    return todo_list

}
