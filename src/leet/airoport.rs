use chrono::{DateTime, Local};
pub struct Airport {
    default_size: i32,
    is_snow: bool,
    is_closed: bool,
    destination_take_off: Vec<Destination>,
}

impl Airport {
    pub fn landed(&mut self, destination: Destination) {
        self.destination_take_off.push(destination)
    }
    pub fn new() -> Self {
        Airport {
            default_size: 2,
            is_closed: false,
            is_snow: true,
            destination_take_off: vec![],
        }
    }

    pub fn get_destinations(&self) -> &Vec<Destination> {
        &self.destination_take_off
    }
}

#[derive(Debug)]
pub struct Destination {
    airplane: String,
    number_airplane: String,
    number_flight: String,
    time_to_take_off: DateTime<Local>,
    airport_take_off: Airport,
    airport_landed: Airport,
}
impl Destination {
    pub fn new(
        airplane: String,
        number_plane: String,
        number_flight: String,
        time_to_take_off: DateTime<Local>,
        airport_take_off: Airport,
        airport_landed: Airport,
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

pub struct Passenger {}
