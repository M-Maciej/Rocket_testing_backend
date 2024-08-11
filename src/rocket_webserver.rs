use rocket::{Rocket,Build};

use super::routes::routing::*;
use super::routes::request_guards::*;
use super::routes::request_local_state::*;

use std::sync::atomic::AtomicUsize;



pub async fn launch_rocket() -> Rocket<Build> {
    let count=AtomicUsize::new(0);
    let some_counter=Counter{count};
    let config=Config{count:AtomicUsize::new(0)};
    let db = Database {
        users: vec![
            User { id: 1, username: "regular_user".into(), is_admin: false },
            User { id: 2, username: "admin_user".into(), is_admin: true },
        ],
    };

    rocket::build()
        .manage(some_counter)
        .manage(config)
        .manage(db)
        .mount("/", routes![index, world, hi, cool, traveller, everything,
         user, user_int,user_str,search, sensitive,counting_route
        ,double_guard,id_local_state])
        .mount("/hello", routes![world])
        .attach(rocket::fairing::AdHoc::on_request("Set Cookies", |request, _| {
            Box::pin(async move {
                // Simulate setting a cookie (user ID 1 is a regular user, ID 2 is an admin)
                request.cookies().add_private(rocket::http::Cookie::new("user_id", "2"));
            })
        }))
        
}
