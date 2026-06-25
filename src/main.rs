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
        camera_collisions(&mut cam, &map);

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

fn distance(dx: f32, dy: f32) -> f32 {
    dx * dx + dy * dy
}

fn camera_collisions(cam: &mut Camera, map: &Map) {
    for y in 0..map.height {
    for x in 0..map.width {
        if map.data[y * map.width + x] == 0 {
            continue;
        }

        let cell_top_pos = Vec2::new(
            (x as f32) * CELL_SIZE,
            (y as f32) * CELL_SIZE );

        let cell_bottom_pos = Vec2::new(
            cell_top_pos.x + CELL_SIZE,
            cell_top_pos.y + CELL_SIZE );
        
        let closest_pos = Vec2::new(
            cam.pos.x.clamp(cell_top_pos.x, cell_bottom_pos.x),
            cam.pos.y.clamp(cell_top_pos.y, cell_bottom_pos.y) );
        
        let dist = distance(
            cam.pos.x - closest_pos.x,
            cam.pos.y - closest_pos.y );
        
        if dist <= CAMERA_SIZE * CAMERA_SIZE {
            cam.pos = collision_placement(cam.pos, cell_top_pos, cell_bottom_pos, closest_pos);
            println!("Cam pos: {}", collision_placement(cam.pos, cell_top_pos, cell_bottom_pos, closest_pos));

            // ! For debug only
            draw_circle(closest_pos.x, closest_pos.y, 3., palette::DARK_RED);
        } 

    }
    }
}

fn collision_placement(cam_pos: Vec2, top_pos: Vec2, bottom_pos: Vec2, closest_pos: Vec2) -> Vec2 {
    let mut new_cam_pos = cam_pos;
    
    // Y
    if closest_pos.y == top_pos.y {
        new_cam_pos.y = top_pos.y - CAMERA_SIZE;
    } else if closest_pos.y == bottom_pos.y {
        new_cam_pos.y = bottom_pos.y + CAMERA_SIZE;
    }

    // X
    if closest_pos.x == top_pos.x {
        new_cam_pos.x = top_pos.x - CAMERA_SIZE;
    } else if closest_pos.x == bottom_pos.x {
        new_cam_pos.x = bottom_pos.x + CAMERA_SIZE;
    }

    new_cam_pos
}
