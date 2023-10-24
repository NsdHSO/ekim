mod leet;
use crate::leet::airoport::Airport;
use crate::leet::airplane::Plane;
use crate::leet::destination::Destination;
use chrono::Local;
use leet::airoport;

fn main() {
    let mut timisoara = Airport::new(2);
    let mut bucuresti = Airport::new(2);
    let mut b32 = Plane::new("b43".to_string());
    let mut b33 = Plane::new("b43".to_string());
    let mut b34 = Plane::new("b43".to_string());

    timisoara.landed(b32);
    timisoara.landed(b33);

    let mut my_vacation = Destination::new(timisoara.clone(), bucuresti.clone());
    timisoara.add_destination(my_vacation.clone());
    println!("{:?}", timisoara)
}
