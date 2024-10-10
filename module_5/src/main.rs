#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::content::RawJson;

struct CustomHeader<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CustomHeader<'r> {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("X-Custom-Header") {
            Some(value) => Outcome::Success(CustomHeader(value)),
            None => Outcome::Error((Status::BadRequest, Status::BadRequest)),
        }
    }
}

#[get("/headers")]
fn headers(
    custom_header: Result<CustomHeader, Status>,
) -> Result<RawJson<String>, Status> {
    match custom_header {
        Ok(CustomHeader(value)) => Ok(RawJson(format!("{{\"X-Custom-Header\": \"{}\"}}", value))),
        Err(_) => Err(Status::BadRequest),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![headers])
}
