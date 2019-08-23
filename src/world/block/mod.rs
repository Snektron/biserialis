#[derive(Debug)]
pub enum Visibility {
    Solid,
    Invisible
}

#[derive(Debug)]
pub struct BlockProperties {
    pub display_name: &'static str,
    pub visibility: Visibility
}

pub type BlockKindId = u8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BlockState {
    pub id: BlockKindId
}

pub struct BlockRegistry {
    props: Vec<BlockProperties>
}

impl BlockRegistry {
    pub fn new() -> Self {
        BlockRegistry {
            props: Vec::new()
        }
    }

    pub fn register(&mut self, props: BlockProperties) -> Option<BlockKindId> {
        if self.props.len() == BlockKindId::max_value() as usize {
            return None;
        }

        self.props.push(props);
        Some((self.props.len() - 1) as u8)
    }

    pub fn properties_for(&self, kind: BlockKindId) -> &BlockProperties {
        &self.props[kind as usize]
    }
}