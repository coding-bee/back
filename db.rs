#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::request::Form;
use rocket::response::content;
use std::sync::{Mutex, Arc};

#[derive(Debug, FromForm)]
struct Row {
    key: String,
    value: String,
}

#[get("/get")]
fn get(rows: State<Arc<Mutex<Vec<Row>>>>) -> content::Json<String> {
    let rows = rows.lock().unwrap();
    content::Json(serde_json::to_string(&*rows).unwrap())
}

#[post("/insert", data = "<row>")]
fn insert(row: Form<Row>, rows: State<Arc<Mutex<Vec<Row>>>>) -> content::Json<String> {
    let mut rows = rows.lock().unwrap();
    rows.push(row.into_inner());
    content::Json(String::from("{\"message\": \"Row inserted successfully.\"}"))
}

#[post("/delete", data = "<key>")]
fn delete(key: Form<String>, rows: State<Arc<Mutex<Vec<Row>>>>) -> content::Json<String> {
    let mut rows = rows.lock().unwrap();
    let index = rows.iter().position(|r| r.key == *key);
    if let Some(index) = index {
        rows.remove(index);
        content::Json(String::from("{\"message\": \"Row deleted successfully.\"}"))
    } else {
        content::Json(String::from("{\"error\": \"Key not found.\"}"))
    }
}

fn main() {
    rocket::ignite()
        .manage(Arc::new(Mutex::new(Vec::<Row>::new())))
        .mount("/", routes![get, insert, delete])
        .launch();
}
