use crate::leet::airoport::Airport;
#[derive(Debug, Clone)]
pub struct Destination {
    landed: Airport,
    take_off: Airport,
}

impl Destination {
    pub fn new(landed_airport: Airport, take_off_airport: Airport) -> Destination {
        Destination {
            landed: landed_airport,
            take_off: take_off_airport,
        }
    }
}
