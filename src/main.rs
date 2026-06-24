use macroquad::prelude::*;
mod palette;

const CELL_SIZE: f32 = 32.;
const PLAYER_SIZE: f32 = 16.;

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

#[macroquad::main("Raycaster")]
async fn main() {
    request_new_screen_size(1080., 720.);

    let map: Map = Map { 
        data: vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 0, 1, 0, 0, 0, 0, 1, 1, 1,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 1, 1, 0, 0, 0, 1,
            1, 1, 0, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ], 
        width: 10, 
        height: 10,
    };

    loop {
        clear_background(palette::BLACK);
        draw_map(&map);

        next_frame().await
    }
}

fn draw_map(map: &Map) {
    for y in 0..map.height {
    for x in 0..map.width  {
        let idx = y * map.width + x;
        let wall = map.data[idx] != 0;

        if wall {
            draw_rectangle(
                (x as f32) *CELL_SIZE, (y as f32) *CELL_SIZE, 
                CELL_SIZE, CELL_SIZE, 
                palette::LIGHT_GREY
            );
            continue;
        }

        draw_rectangle(                
            (x as f32) *CELL_SIZE, (y as f32) *CELL_SIZE, 
            CELL_SIZE, CELL_SIZE, 
            palette::DARK_GREY
        );
    }}
}
