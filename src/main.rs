#[macro_use]
extern crate rocket;
mod currency;
use currency::currency as other_currency;


#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build().mount("/", routes![other_currency::index]).launch().await;
}

