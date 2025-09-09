use crate::Vector3;

pub struct ChunkGrid {
    grid: Vec<Vector3<i32>>,
}

impl ChunkGrid {
    pub fn new() -> Self {
        Self { grid: Vec::new() }
    }

    pub fn is_in_range(&self, position: &Vector3<i32>, range: f64) -> bool {
        self.grid
            .iter()
            .any(|&p| (*position - p).magnitude() < range)
    }

    pub fn insert(&mut self, point: Vector3<i32>) {
        self.grid.push(point);
    }

    pub fn elements(self) -> Vec<Vector3<i32>> {
        self.grid
    }
}
