use crate::box_game::*;
use ggrs::{GGRSError, GGRSEvent, SessionState};
use macroquad::prelude::*;
use std::env;
use std::net::SocketAddr;

//const FPS: u64 = 60;
const FPS_INV: f32 = 1. / 60.;
const NUM_PLAYERS: usize = 2;
const INPUT_SIZE: usize = std::mem::size_of::<u8>();

//type TagType = box_game::TagType;

pub async fn main() {
    // read cmd line arguments very clumsily
    let mut args: Vec<String> = env::args().collect();
    // Remove the first argument to
    // not have the switch for whether
    // we're a spectator or a p2p
    // process.
    args.remove(0);

    assert_eq!(args.len(), 3);

    let port: u16 = args[1].parse().unwrap();
    let host_addr: SocketAddr = args[2].parse().unwrap();

    // create a GGRS session for a spectator
    let mut sess =
        ggrs::start_p2p_spectator_session(NUM_PLAYERS as u32, INPUT_SIZE, port, host_addr).unwrap();

    // start the GGRS session
    sess.start_session().unwrap();

    // Create a new box game
    let mut game = BoxGame::new();

    let mut remaining_time = 0.;

    // game loop
    loop {
        remaining_time += get_frame_time();

        while remaining_time >= FPS_INV {
            if sess.current_state() == SessionState::Running {
                // tell GGRS it is time to advance the frame and handle the requests
                match sess.advance_frame() {
                    Ok(requests) => game.handle_requests(requests),
                    Err(GGRSError::PredictionThreshold) => {
                        println!("Skipping a frame: Waiting for input from host.");
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
            println!("Event: {:?}", event);
            if let GGRSEvent::Disconnected { .. } = event {
                panic!("Disconnected from host.");
            }
        }

        render(&game);

        next_frame().await
    }
}
