use ggrs::{GGRSEvent, PlayerHandle, PlayerType, SessionState};
use macroquad::prelude::*;
use std::env;
use std::net::SocketAddr;

use super::game_state::*;
use super::render::*;

//const FPS: u64 = 60;
const FPS_INV: f32 = 1. / 60.;
const NUM_PLAYERS: usize = 2;
const INPUT_SIZE: usize = std::mem::size_of::<u8>();

//type TagType = box_game::TagType;
//type Vec2 = crate::Vec2;

pub async fn main() {
    // read cmd line arguments very clumsily
    let mut args: Vec<String> = env::args().collect();
    // Remove the first argument to
    // not have the switch for whether
    // we're a spectator or a p2p
    // process.
    args.remove(0);

    assert!(args.len() >= 4);

    let port: u16 = args[1].parse().unwrap();
    let local_handle: PlayerHandle = args[2].parse().unwrap();
    let remote_handle: PlayerHandle = 1 - local_handle;
    let remote_addr: SocketAddr = args[3].parse().unwrap();

    // create a GGRS session with two players
    let mut sess = ggrs::start_p2p_session(NUM_PLAYERS as u32, INPUT_SIZE, port).unwrap();

    // add players
    sess.add_player(PlayerType::Local, local_handle).unwrap();
    sess.add_player(PlayerType::Remote(remote_addr), remote_handle)
        .unwrap();

    // optionally, add a spectator
    if args.len() > 4 {
        let spec_addr: SocketAddr = args[4].parse().unwrap();
        sess.add_player(PlayerType::Spectator(spec_addr), 2)
            .unwrap();
    }

    // set input delay for the local player
    sess.set_frame_delay(2, local_handle).unwrap();

    // start the GGRS session
    sess.start_session().unwrap();

    // Create a new box game
    let mut game = BoxGame::new();

    // set render settings

    // game loop
    let mut remaining_time = 0.;
    loop {
        remaining_time += get_frame_time();

        while remaining_time >= FPS_INV {
            if sess.current_state() == SessionState::Running {
                // tell GGRS it is time to advance the frame and handle the requests
                let local_input = game.local_input();

                match sess.advance_frame(local_handle, &local_input) {
                    Ok(requests) => game.handle_requests(requests),
                    Err(ggrs::GGRSError::PredictionThreshold) => {
                        //println!("Skipping a frame: PredictionThreshold")
                    }
                    Err(e) => panic!("{}", e),
                }
            }

            remaining_time -= FPS_INV;
        }

        // get newest info from remotes
        sess.poll_remote_clients();

        // handle GGRS events
        for event in sess.events() {
            if let GGRSEvent::WaitRecommendation { skip_frames } = event {
                // frames_to_skip += skip_frames
            }
            println!("Event: {:?}", event);
        }

        // update key state
        game.key_states[0] = is_key_down(KeyCode::W);
        game.key_states[1] = is_key_down(KeyCode::A);
        game.key_states[2] = is_key_down(KeyCode::S);
        game.key_states[3] = is_key_down(KeyCode::D);

        //debug_print(&game);

        render(&game);

        next_frame().await
    }
}

fn debug_print(game: &BoxGame) {
    let checksum_string = format!(
        "Frame {}: Checksum {}",
        game.last_checksum().0,
        game.last_checksum().1
    );

    println!("{}", checksum_string);
    println!(
        "Up: {0}, Down: {1}, Left: {2}, Right: {3}",
        game.key_states[0], game.key_states[1], game.key_states[2], game.key_states[3]
    );
}
