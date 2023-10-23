#[derive(Debug)]
pub struct Airport {
    default_size: i32,
    is_snow: bool,
    is_closed: bool,
}

impl Airport {
    pub fn new() -> Self {
        Airport {
            default_size: 2,
            is_closed: false,
            is_snow: true,
        }
    }
}
