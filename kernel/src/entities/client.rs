use destructure::Destructure;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, Deserialize, Serialize, Destructure)]
pub struct Client {
}