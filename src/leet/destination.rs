use crate::leet::airoport;
use chrono::{DateTime, Local};
#[derive(Debug)]
pub struct Destination {
    airplane: String,
    number_airplane: String,
    number_flight: String,
    time_to_take_off: DateTime<Local>,
    airport_take_off: airoport::Airport,
    airport_landed: airoport::Airport,
}
impl Destination {
    pub fn new(
        airplane: String,
        number_plane: String,
        number_flight: String,
        time_to_take_off: DateTime<Local>,
        airport_take_off: airoport::Airport,
        airport_landed: airoport::Airport,
    ) -> Self {
        Destination {
            airplane,
            number_flight,
            time_to_take_off,
            number_airplane: number_plane,
            airport_landed,
            airport_take_off,
        }
    }
}
