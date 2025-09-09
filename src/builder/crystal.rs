use {crate::Vector3, std::collections::HashSet};

pub fn build_crystal(crystal_grid: &HashSet<Vector3<i32>>) -> HashSet<Vector3<i32>> {
    let mut crystal_writes: HashSet<Vector3<i32>> = HashSet::new();

    for position in crystal_grid {
        crystal_writes.insert(position.clone() + Vector3::new(0, 1, 0));
    }

    crystal_writes
}
