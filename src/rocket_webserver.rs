use rocket::{Rocket,Build};

use super::routes::routing::*;
use super::routes::request_guards::*;
use super::routes::request_local_state::*;
use super::routes::counter::*;

use std::sync::atomic::AtomicUsize;



pub async fn launch_rocket() -> Rocket<Build> {
    let config=Config{count:AtomicUsize::new(0)};
    let db = Database {
        users: vec![
            User { id: 1, username: "regular_user".into(), is_admin: false },
            User { id: 2, username: "admin_user".into(), is_admin: true },
        ],
    };
    let initial_count = 0;
    

    rocket::build()
        .manage(config)
        .manage(db)
        .manage(Counter::new(initial_count))
        .mount("/", routes![index, world, hi, cool, traveller, everything,
         user, user_int,user_str,search, sensitive,counting_route
        ,double_guard,id_local_state,
        user_dashboard,admin_dashboard])
        .mount("/hello", routes![world])
        .attach(rocket::fairing::AdHoc::on_request("Set Cookies", |request, _| {
            Box::pin(async move {
                // Simulate setting a cookie (user ID 1 is a regular user, ID 2 is an admin)
                request.cookies().add_private(rocket::http::Cookie::new("user_id", "2"));
            })
        }))
        
}
