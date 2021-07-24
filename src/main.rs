use macroquad::window::Conf;
use resphys::*;
use serde::{Deserialize, Serialize};

pub mod box_game;

pub type Vec2 = resphys::Vec2;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum TagType {
    Tile,
    Player,
}

fn window_conf() -> Conf {
    let kind = std::env::args().nth(1).unwrap();
    let name = match &kind[..] {
        "p2p" => "Peer to Peer Connection",
        "spectator" => "Spectator Client",
        "sync_test" => "Sync Test",
        _ => panic!("Unexpected argument: {}", kind),
    };

    Conf {
        window_title: name.to_owned(),
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let kind = std::env::args().nth(1).unwrap();
    match &kind[..] {
        "p2p" => crate::box_game::p2p::main().await,
        "spectator" => crate::box_game::spectator::main().await,
        "sync_test" => crate::box_game::sync_test::main().await,
        _ => panic!("Unexpected argument: {}", kind),
    }
}
