use crate::{box_game, TagType, Vec2};
use macroquad::prelude::*;
use resphys::*;

pub fn render(game: &box_game::BoxGame) {
    clear_background(BLACK);

    let checksum_string = format!(
        "Frame {}: Checksum {}",
        game.last_checksum().0,
        game.last_checksum().1
    );
    let periodic_string = format!(
        "Frame {}: Checksum {}",
        game.periodic_checksum().0,
        game.periodic_checksum().1
    );

    //let game_state_string = format!("GameState: {}", );

    draw_text_ex(&checksum_string, 20.0, 20.0, TextParams::default());
    draw_text_ex(&periodic_string, 20.0, 40.0, TextParams::default());

    for (_, collider) in game.game_state().colliders.iter() {
        let body = &game.game_state().bodies[collider.owner];
        draw_collider(&collider, body.position);
    }
}

pub fn draw_collider(collider: &Collider<TagType>, position: Vec2) {
    let mut color = match collider.state {
        ColliderState::Solid => BLUE,
        ColliderState::Sensor => YELLOW,
    };
    // Quickly change color's alpha
    let fill_color = color;

    color.a = 0.3;
    // This works because there's currently only AABB shape. Half extents.
    let wh = collider.shape.half_exts;
    let x_pos = FP::to_num::<f32>(position.x() - wh.x() + collider.offset.x());
    let y_pos = FP::to_num::<f32>(position.y() - wh.y() + collider.offset.y());
    draw_rectangle(
        x_pos,
        y_pos,
        FP::to_num::<f32>(wh.x()) * 2.,
        FP::to_num::<f32>(wh.y()) * 2.,
        color,
    );
    draw_rectangle_lines(
        x_pos,
        y_pos,
        FP::to_num::<f32>(wh.x()) * 2.,
        FP::to_num::<f32>(wh.y()) * 2.,
        3.,
        fill_color,
    );
}
