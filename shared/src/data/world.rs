use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Block {
    Grass,
    Dirt,
    Stone,
    Bedrock,
}
