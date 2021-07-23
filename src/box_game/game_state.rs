use ggrs::{Frame, GGRSRequest, GameInput, GameState, GameStateCell, NULL_FRAME};
use macroquad::prelude::*;
use resphys::*;
use serde::{Deserialize, Serialize};
use crate::{TagType, Vec2};

//const FPS: u64 = 60;
const FPS_INV: f32 = 1. / 60.;

const NUM_PLAYERS: usize = 2;
const CHECKSUM_PERIOD: i32 = 100;
const PLAYER_SPEED: i32 = 10;

// pub const PLAYER_COLORS: [Color; 2] = [BLUE, ORANGE];

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 600;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

/// Computes the fletcher16 checksum, copied from wikipedia: <https://en.wikipedia.org/wiki/Fletcher%27s_checksum>
fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for index in 0..data.len() {
        sum1 = (sum1 + data[index] as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    // physics data
    body_handle: BodyHandle,
    collider_handle: ColliderHandle,
    // game state
    is_grounded: bool,
}

fn check_grounded(physics: &mut PhysicsWorld<TagType>, player: &mut Player) -> bool {
    // check if there are no collisions
    if physics
        .collisions_of(player.collider_handle)
        .peekable()
        .peek()
        .is_none()
    {
        return false;
    } else {
        for (_, info) in physics.collisions_of(player.collider_handle) {
            //println!("info: {:?}", info);
            if info.normal.y() > 0 {
                return true;
            }
        }
        return false;
    }
}

fn controls(mut velocity: Vec2, player: &Player, input: u8) -> Vec2 {
    if input & INPUT_DOWN != 0 {
        velocity.set_y(PLAYER_SPEED);
    }
    // jump
    if input & INPUT_UP != 0 {
        velocity.set_y(-PLAYER_SPEED);
    }
    // stop moving if not pressing things
    if input & INPUT_DOWN == 0 && input & INPUT_UP == 0 {
        velocity.set_y(0);
    }

    // move left
    if input & INPUT_RIGHT != 0 {
        velocity.set_x(PLAYER_SPEED);
    }
    // move right
    if input & INPUT_LEFT != 0 {
        velocity.set_x(-PLAYER_SPEED);
    }
    // stop moving if not pressing things
    if input & INPUT_LEFT == 0 && input & INPUT_RIGHT == 0 {
        velocity.set_x(0);
    }

    //*velocity.x_mut() = velocity.x().max(FP::from_num(-128)).min(FP::from_num(128));

    velocity
}

fn physics_update(
    physics: &mut PhysicsWorld<TagType>,
    bodies: &mut resphys::BodySet,
    colliders: &mut resphys::ColliderSet<TagType>,
    player: &mut Player,
    input: u8,
) {
    let player_body = &mut bodies[player.body_handle];

    // get collision

    // set if grounded
    player.is_grounded = check_grounded(physics, player);
    // set movement

    /*
    let gravity = Vec2::from(0, -5);

    // gravity only happens when not grounded
    if !player.is_grounded {
        player_body.velocity = player_body.velocity + gravity;
    } else {
        player_body.velocity.set_y(0);
    }
    */

    player_body.velocity = controls(player_body.velocity, player, input);

    //println!("{}", player_body.velocity);
    //println!("{}", player.is_grounded);

    //player_body.velocity = player_body.velocity.mul_scalar(FPS_INV);

    physics.step(FP::from_num(FPS_INV), bodies, colliders);
}

pub struct BoxGame {
    game_state: BoxGameState,
    pub key_states: [bool; 4],
    //font: PathBuf,
    last_checksum: (Frame, u64),
    periodic_checksum: (Frame, u64),
}

impl BoxGame {
    pub fn new() -> Self {
        Self {
            game_state: BoxGameState::new(),
            key_states: [false; 4],
            //font,
            last_checksum: (NULL_FRAME, 0),
            periodic_checksum: (NULL_FRAME, 0),
        }
    }

    pub fn game_state(&self) -> &BoxGameState {
        &self.game_state
    }

    pub fn last_checksum(&self) -> (i32, u64) {
        self.last_checksum
    }

    pub fn periodic_checksum(&self) -> (i32, u64) {
        self.periodic_checksum
    }

    pub fn handle_requests(&mut self, requests: Vec<GGRSRequest>) {
        for request in requests {
            match request {
                GGRSRequest::LoadGameState { cell } => self.load_game_state(cell),
                GGRSRequest::SaveGameState { cell, frame } => self.save_game_state(cell, frame),
                GGRSRequest::AdvanceFrame { inputs } => self.advance_frame(inputs),
            }
        }
    }

    fn save_game_state(&mut self, cell: GameStateCell, frame: Frame) {
        assert_eq!(self.game_state.frame, frame);
        let buffer = bincode::serialize(&self.game_state).unwrap();
        let checksum = fletcher16(&buffer) as u64;

        cell.save(GameState::new(frame, Some(buffer), Some(checksum)));
    }

    fn load_game_state(&mut self, cell: GameStateCell) {
        let state_to_load = cell.load();
        self.game_state = bincode::deserialize(&state_to_load.buffer.unwrap()).unwrap();
    }

    fn advance_frame(&mut self, inputs: Vec<GameInput>) {
        // increase the frame counter
        self.game_state.frame += 1;

        for i in 0..NUM_PLAYERS {
            // get input of that player
            let input;
            // check if the player is disconnected (disconnected players might maybe do something different)
            if inputs[i].frame == NULL_FRAME {
                input = 4; // disconnected players spin
            } else {
                input = bincode::deserialize(inputs[i].input()).unwrap();
            }

            physics_update(
                &mut self.game_state.physics,
                &mut self.game_state.bodies,
                &mut self.game_state.colliders,
                &mut self.game_state.players[i],
                input,
            );
        }

        // TODO: inefficient to serialize the gamestate here just for the checksum
        // remember checksum to render it later
        let buffer = bincode::serialize(&self.game_state).unwrap();
        let checksum = fletcher16(&buffer) as u64;
        self.last_checksum = (self.game_state.frame, checksum);
        if self.game_state.frame % CHECKSUM_PERIOD == 0 {
            self.periodic_checksum = (self.game_state.frame, checksum);
        }
    }

    #[allow(dead_code)]
    pub fn local_input(&self) -> Vec<u8> {
        // Create a set of pressed Keys.
        let mut input: u8 = 0;

        // ugly, but it works...
        if self.key_states[0] {
            input |= INPUT_UP;
        }
        if self.key_states[1] {
            input |= INPUT_LEFT;
        }
        if self.key_states[2] {
            input |= INPUT_DOWN;
        }
        if self.key_states[3] {
            input |= INPUT_RIGHT;
        }

        bincode::serialize(&input).unwrap()
    }
}

// BoxGameState holds all relevant information about the game state
#[derive(Serialize, Deserialize, Debug)]
pub struct BoxGameState {
    pub frame: i32,
    pub players: Vec<Player>,
    pub physics: PhysicsWorld<TagType>,
    pub bodies: BodySet,
    pub colliders: ColliderSet<TagType>,
}

impl BoxGameState {
    pub fn new() -> Self {
        let mut physics = PhysicsWorld::new();
        let mut bodies = BodySet::new();
        let mut colliders = ColliderSet::new();
        let mut players = Vec::<Player>::new();

        world_generation(&mut physics, &mut bodies, &mut colliders);

        // generate all players
        for i in 0..NUM_PLAYERS as i32 {
            let x = WINDOW_WIDTH as i32 / 2 + (2 * i - 1) * (WINDOW_WIDTH as i32 / 4);
            let y = WINDOW_HEIGHT as i32 / 2;

            // create player physics data
            let player_body = resphys::builder::BodyDesc::new()
                .with_position(Vec2::from(x, y))
                .self_collision(false)
                .build();
            let player_collider = resphys::builder::ColliderDesc::new(
                AABB {
                    half_exts: Vec2::from(16., 32.),
                },
                TagType::Player,
            );

            let player_bhandle = bodies.insert(player_body);
            let _player_chandle = colliders
                .insert(
                    player_collider.build(player_bhandle),
                    &mut bodies,
                    &mut physics,
                )
                .unwrap();

            let player = Player {
                body_handle: player_bhandle,
                collider_handle: _player_chandle,
                is_grounded: false,
            };

            players.push(player);
        }

        Self {
            frame: 0,
            players,
            physics,
            bodies,
            colliders,
        }
    }

    pub fn physics_mut(&mut self) -> &mut PhysicsWorld<TagType> {
        &mut self.physics
    }

    pub fn bodies_mut(&mut self) -> &mut BodySet {
        &mut self.bodies
    }

    pub fn colliders_mut(&mut self) -> &mut ColliderSet<TagType> {
        &mut self.colliders
    }
}

fn world_generation(
    physics: &mut PhysicsWorld<TagType>,
    bodies: &mut BodySet,
    colliders: &mut ColliderSet<TagType>,
) {
    for x in (0..=768).step_by(32) {
        add_tile(physics, bodies, colliders, Vec2::from(16 + x, 16));
    }
    for y in (32..=544).step_by(32) {
        add_tile(physics, bodies, colliders, Vec2::from(16, 16 + y));
    }
    for y in (32..=544).step_by(32) {
        add_tile(physics, bodies, colliders, Vec2::from(768 + 16, 16 + y));
    }
    for x in (32..=768 - 32).step_by(32) {
        add_tile(physics, bodies, colliders, Vec2::from(16 + x, 544 + 16));
    }
}

fn add_tile(
    physics: &mut PhysicsWorld<TagType>,
    bodies: &mut resphys::BodySet,
    colliders: &mut resphys::ColliderSet<TagType>,
    position: Vec2,
) {
    let body3 = resphys::builder::BodyDesc::new()
        .with_position(position)
        .make_static()
        .build();
    let collider3 = resphys::builder::ColliderDesc::new(
        AABB {
            half_exts: Vec2::from(16., 16.),
        },
        TagType::Tile,
    );
    let body3_handle = bodies.insert(body3);
    colliders.insert(collider3.build(body3_handle), bodies, physics);
}
