use crate::leet::passenger;
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
