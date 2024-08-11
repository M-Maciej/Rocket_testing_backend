use std::path::{Path,PathBuf};
use rocket::{fs::NamedFile, State};
use std::sync::atomic::{AtomicUsize,Ordering};
use super::request_local_state::*;

#[get("/")]
pub fn index() -> &'static str{
    "hello world"
}
#[get("/world")]              // <- route attribute
pub fn world() -> &'static str {  // <- request handler
    "hello, world!!!!!!!!!"
}

#[get("/hi/<name>")]
pub fn hi(name:&str) -> String{
    format!("Hi, {}!",name)
}

#[get("/traveller/<_>/see")]
pub fn traveller()-> String{
    "hello traveller".to_string()
}

#[get("/hello/<name>/<age>/<cool>")]
pub fn cool(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/user/<id>")]
pub fn user(id: usize) -> String{ 
format!("Hello usize {}",id)
}

#[get("/user/<id>", rank = 2)]
pub fn user_int(id: isize) -> String{ 
    format!("Hello isize {}",id)
}

#[get("/user/<id>", rank = 3)]
pub fn user_str(id: &str) -> String{ 
    format!("Hello &str {}",id)
}

#[get("/<_..>", rank = 4)]
pub fn everything() -> &'static str {
    "Hey, you're here."
}

#[get("/search?<query>")]
pub fn search(query: Option<String>) -> String {
    // Sample local data array
    let data = vec!["Alice", "Bob", "Charlie", "Dave", "Eve"];

    // Handle the query parameter safely
    match query {
        Some(query) => {
            let query = query.to_lowercase();
            
            // Search for items in the data array that match the query
            let results: Vec<&str> = data
            .iter()
            .filter(|&&item| item.to_lowercase().contains(&query))
            .cloned()  // Dereference &&str to &str
            .collect();


            // Return search results or a message if nothing was found
            if results.is_empty() {
                format!("No results found for '{}'.", query)
            } else {
                format!("Found {} result(s): {:?}", results.len(), results)
            }
        }
        None => "Please provide a search query.".to_string(),
    }
}
pub struct Counter{
    pub count:AtomicUsize
}
pub struct Config{
    pub count:AtomicUsize
}

#[get("/count")]
pub fn counting_route(hit_count: &State<Counter>)->String{
    let current_count=hit_count.count.load(Ordering::Relaxed);
    format!("Current count: {}",current_count)
}

#[get("/double")]
pub fn double_guard(hit_count: &State<Counter>, config:&State<Config>)->String{
    let current_count=hit_count.count.load(Ordering::Relaxed);
    let current_config=config.count.load(Ordering::Relaxed);
    format!("Current count: {}, Current Config: {}",current_count,current_config)
}

//Request Local State
#[get("/local/state")]
pub fn id_local_state(id: &RequestID) -> String{
    format!("This is request #{}.",id.0)
}

// Admin Dashboard Route
#[get("/dashboard")]
fn admin_dashboard(admin: Admin) -> String {
    format!("Welcome to the admin dashboard, {}!", admin.user.username)
}

// User Dashboard Route (fallback if not admin)
#[get("/dashboard", rank = 2)]
fn user_dashboard(user: &User) -> String {
    format!("Welcome to the user dashboard, {}!", user.username)
}
