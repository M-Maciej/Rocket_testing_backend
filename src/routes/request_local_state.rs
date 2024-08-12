use rocket::request::{self,Request,Outcome,FromRequest};
use std::sync::atomic::AtomicUsize;
use rocket::{http::Status,State};
use rocket::outcome::IntoOutcome;
use rocket::outcome::try_outcome;
use rocket::http::Cookie;



static ID_COUNTER:AtomicUsize= AtomicUsize::new(0);

pub struct RequestID(pub usize);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r RequestID{
    type Error=();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(request.local_cache(|| {
            // Try to get the RequestID from a cookie
            let request_id = request.cookies()
                .get_private("request_id")
                .and_then(|cookie| cookie.value().parse().ok())
                .unwrap_or_else(|| {
                    // If no cookie is found, generate a new RequestID
                    let new_id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    // Store the new RequestID in a cookie
                    request.cookies().add_private(Cookie::new("request_id", new_id.to_string()));
                    new_id
                });

            RequestID(request_id)
        }))
    }  
}

// Simulated Database Struct
pub struct Database {
    pub users: Vec<User>,
}

impl Database {
    fn get_user(&self, id: i32) -> Option<User> {
        self.users.iter().find(|&user| user.id == id).cloned()
    }
}

// User Struct
#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub is_admin: bool,
}

// Admin Struct
pub struct Admin<'r> {
    pub user: &'r User,
}

// Implementing the User Guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_result = request.local_cache_async(async {
            let db = request.guard::<&State<Database>>().await.succeeded()?;
            request.cookies()
                .get_private("user_id")
                .and_then(|cookie| cookie.value().parse().ok())
                .and_then(|id| db.get_user(id))
        }).await;

        user_result.as_ref().or_forward(Status::Unauthorized)
    }
}

// Implementing the Admin Guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin<'r> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<&User>().await);
        if user.is_admin {
            Outcome::Success(Admin { user })
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

