use crate::leet::airplane::Plane;
use crate::leet::destination::Destination;

#[derive(Debug, Clone)]
pub struct Airport {
    default_size: i32,
    is_closed: bool,
    is_snow: bool,
    planes: Vec<Plane>,
    destinations: Vec<Destination>,
}

impl Airport {
    pub fn new(default_size: i32) -> Self {
        Airport {
            default_size,
            is_snow: false,
            is_closed: false,
            planes: vec![],
            destinations: vec![],
        }
    }
    pub fn landed(&mut self, plane: Plane) {
        if (self.planes.len() as i32) < self.default_size {
            self.planes.push(plane);
        } else {
            panic!("The Airport is full")
        }
    }

    pub fn add_destination(&mut self, destination: Destination) {
        self.destinations.push(destination);
    }
}
