use crate::box_game::*;
use ggrs::{GGRSError, GGRSEvent, SessionState};
use macroquad::prelude::*;
//use std::env;
//use std::net::SocketAddr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//const FPS: u64 = 60;
const FPS_INV: f32 = 1. / 60.;
const NUM_PLAYERS: usize = 2;
const INPUT_SIZE: usize = std::mem::size_of::<u8>();
const CHECK_DISTANCE: u32 = 7;

//type TagType = box_game::TagType;

pub async fn main() {
    // create a GGRS session for a spectator
    let mut sess =
        ggrs::start_synctest_session(NUM_PLAYERS as u32, INPUT_SIZE, CHECK_DISTANCE).unwrap();

    // start the GGRS session
    sess.start_session().unwrap();

    let local_player = 0;

    // set input delay for any player you want
    sess.set_frame_delay(2, local_player).unwrap();

    // Create a new box game
    let mut game = BoxGame::new();

    let mut remaining_time = 0.;

    // game loop
    loop {
        remaining_time += get_frame_time();
        while remaining_time >= FPS_INV {
            // tell GGRS it is time to advance the frame and handle the requests
            let local_input = game.local_input();

            match sess.advance_frame(local_player, &local_input) {
                Ok(requests) => game.handle_requests(requests),
                Err(ggrs::GGRSError::MismatchedChecksum{frame}) => {
                    print_to_file(game.log());
                    panic!("Desynced at frame {}: MismatchedChecksum", frame);
                }
                Err(e) => panic!("{}", e),
            }
            //let requests = sess.advance_frame(local_player, &local_input).unwrap();

            // handle requests
            // game.handle_requests(requests);

            remaining_time -= FPS_INV;
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

fn print_to_file(my_string: &String)
{
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(my_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}