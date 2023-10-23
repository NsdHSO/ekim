mod leet;
use crate::leet::airoport::Airport;
use crate::leet::destination;
use chrono::Local;
use leet::airoport;

fn main() {
    let mut timisoara = Airport::new();
    let mut bucuresti = Airport::new();
    let destination = destination::Destination::new(
        "Bcr".to_string(),
        "AB312".to_string(),
        "Bcr".to_string(),
        Local::now(),
        timisoara,
        bucuresti,
    );

    timisoara.landed(destination);
    println!("{:?}", timisoara.get_destinations())
}
