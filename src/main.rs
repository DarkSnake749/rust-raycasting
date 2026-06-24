use macroquad::prelude::*;
mod palette;

const CELL_SIZE: f32 = 32.;
const CAMERA_SIZE: f32 = 8.;
const CAMERA_SPEED: f32 = 1.5;

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

struct Camera {
    fov: f32,
    pos: Vec2,
    dir: Vec2,
    vel: Vec2,
}

#[macroquad::main("Raycaster")]
async fn main() {
    request_new_screen_size(1080., 720.);

    let map= Map { 
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

    let mut cam = Camera { 
        fov: 90.,
        pos: Vec2::new(40., 40.),
        dir: Vec2::new(0., 0.),  
        vel: Vec2::new(0., 0.),
    };

    loop {
        clear_background(palette::BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        draw_map(&map);
        draw_camera(&cam);

        camera_dir(&mut cam);
        update_cam_pos(&mut cam);

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

fn draw_camera(cam: &Camera) {
    draw_circle(cam.pos.x, cam.pos.y, CAMERA_SIZE, palette::RED);
}

fn camera_dir(cam: &mut Camera) {
    let dir_vec = Vec2 {
         x: mouse_position().0 - cam.pos.x, 
         y: mouse_position().1 - cam.pos.y
    };

    cam.dir = dir_vec.normalize();

    // ! For debug only
    //draw_line(cam.x, cam.y, mouse_position().0, mouse_position().1, 3., palette::DARK_RED);
}

fn update_cam_pos(cam: &mut Camera) {
    if !is_key_down(KeyCode::W) {
        return;
    }

    cam.vel = cam.dir * CAMERA_SPEED;
    cam.pos += cam.vel;
}
