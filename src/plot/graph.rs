pub type Point = [f32; 2];

pub struct Graph {
    pub data: Box<[Point]>,
}

impl Graph {
    pub fn new(data: Box<[Point]>) -> Self {
        Self { data }
    }
}
