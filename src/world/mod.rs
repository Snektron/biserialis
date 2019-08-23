pub mod block;
pub mod chunk;

pub type WorldTimestamp = u64;

// Used to indicate the spatial location of a single voxel.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Coordinates {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Coordinates {
            x, y, z
        }
    }
}