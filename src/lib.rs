
use resphys::*;
use serde::{Deserialize, Serialize};

pub mod box_game;

pub type Vec2 = resphys::Vec2;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum TagType {
    Tile,
    Player,
}