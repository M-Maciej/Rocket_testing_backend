use rocket::form::name::Key;
use rocket::request::{self,FromRequest,Outcome,Request};
use rocket::outcome::IntoOutcome;
use rocket::http::Status;

struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool{
            key== "valid_api_key"
        }

        match req.headers().get_one("x-api-key"){
            None => Outcome::Error((Status::BadRequest,ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_)=> Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    
    
    }
}

#[get("/sensitive")]
pub fn sensitive(key: ApiKey<'_>) -> &'static str {
    "Sensitive data."
}