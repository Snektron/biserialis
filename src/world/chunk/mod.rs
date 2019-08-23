use std::convert::{From, Into};
use std::collections::HashMap;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use crate::world::{Coordinates, WorldTimestamp};
use crate::world::block::BlockState;

// The size of the side of a single chunk in blocks - 16 blocks in each dimension.
pub const CHUNK_EXTENT: usize = 16;

// The amount of blocks that fit inside a single chunk.
pub const CHUNK_VOLUME: usize = CHUNK_EXTENT * CHUNK_EXTENT * CHUNK_EXTENT;

// Used to indicate the spatial position of a chunk.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ChunkCoordinates {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl ChunkCoordinates {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        ChunkCoordinates {
            x, y, z
        }
    }
}

impl From<Coordinates> for ChunkCoordinates {
    fn from(coords: Coordinates) -> Self {
        ChunkCoordinates {
            x: coords.x / CHUNK_EXTENT as i64,
            y: coords.y / CHUNK_EXTENT as i64,
            z: coords.z / CHUNK_EXTENT as i64
        }
    }
}

impl From<ChunkCoordinates> for Coordinates {
    fn from(ccoords: ChunkCoordinates) -> Self {
        Coordinates {
            x: ccoords.x * CHUNK_EXTENT as i64,
            y: ccoords.y * CHUNK_EXTENT as i64,
            z: ccoords.z * CHUNK_EXTENT as i64
        }
    }
}

// Stores states of blocks in a chunk
pub struct BlockStorage {
    blocks: [BlockState; CHUNK_VOLUME]
}

impl BlockStorage {
    pub const fn with_default(default_block: BlockState) -> Self {
        BlockStorage {
            blocks: [default_block; CHUNK_VOLUME]
        }
    }

    // Get the index of the voxel stored at (x, y, z) relative to the chunk's base coordinates.
    // x, y and z must be smaller than CHUNK_EXTENT, or this function returns an invalid index.
    const fn index_of(offset: (usize, usize, usize)) -> usize {
        offset.0 as usize + (offset.1 as usize + offset.2 as usize * CHUNK_EXTENT) * CHUNK_EXTENT
    }

    const fn offset_of(index: usize) -> (usize, usize, usize) {
        (index % CHUNK_EXTENT, index / CHUNK_EXTENT % CHUNK_EXTENT, index / CHUNK_EXTENT / CHUNK_EXTENT)
    }

    // Return the block state at a certain position relative to the chunk's base coordinates.
    // x, y, and z must be smaller than CHUNK_EXTENT, lest this function panics.
    const fn block_at(&self, offset: (usize, usize, usize)) -> BlockState {
        self.blocks[Self::index_of(offset)]
    }

    // Set the blocks state at a certain position relative to the chunk's base coordinates.
    // x, y, and z must be smaller than CHUNK_EXTENT, lest this function panics.
    fn set_block_at(&mut self, state: BlockState, offset: (usize, usize, usize)) {
        self.blocks[Self::index_of(offset)] = state;
    }
}

pub struct Chunk {
    last_modified: WorldTimestamp,
    blocks: BlockStorage
}
