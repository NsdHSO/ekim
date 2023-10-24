#[derive(Debug, Clone)]
pub struct Plane {
    name: String,
}
impl Plane {
    pub fn new(name: String) -> Plane {
        Plane { name }
    }
}
