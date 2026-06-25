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

    let mut min_map_visible = true;

    loop {
        clear_background(palette::BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::M) {
            min_map_visible = !min_map_visible;
        }
        
        if min_map_visible {
            draw_map(&map);
            draw_camera(&cam);
        }

        camera_dir(&mut cam);
        update_cam_pos(&mut cam);
        camera_collisions(&mut cam, &map);

        test_ray(&cam, &map);

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
            cam.pos = resolve_collisions(cam.pos, cell_top_pos, cell_bottom_pos);
        } 

    }
    }
}

fn resolve_collisions(cam_pos: Vec2, min: Vec2, max: Vec2) -> Vec2 {
    let mut new_pos = cam_pos;

    let overlap_left   = (cam_pos.x + CAMERA_SIZE) - min.x;
    let overlap_right  = max.x - (cam_pos.x - CAMERA_SIZE);
    let overlap_top    = (cam_pos.y + CAMERA_SIZE) - min.y;
    let overlap_bottom = max.y - (cam_pos.y - CAMERA_SIZE);

    let overlap = Vec2::new(
        overlap_left.min(overlap_right),
        overlap_top.min(overlap_bottom));
    
    if overlap.x < overlap.y {
        if overlap_left < overlap_right { new_pos.x -= overlap_left; }
        else { new_pos.x += overlap_right; }
    } else {
        if overlap_top < overlap_bottom { new_pos.y -= overlap_top; }
        else { new_pos.y += overlap_bottom }
    }

    new_pos
}

fn test_ray(cam: &Camera, map: &Map) {
    draw_line(
        cam.pos.x, 
        cam.pos.y, 
        cam.pos.x + cam.dir.x * CELL_SIZE * (map.width as f32), 
        cam.pos.y + cam.dir.y * CELL_SIZE * (map.height as f32), 
        3., 
        GREEN);
    
    let unit_pos = scale_down_position(cam.pos);
    let mut map_pos = map_position(unit_pos);

    let delta_dist = Vec2::new(
        (1. / cam.dir.x).abs(),
        (1. / cam.dir.y).abs()
    );

    let mut side_dist = Vec2::new(0., 0.);
    if cam.dir.x > 0. { 
        side_dist.x = (map_pos.x + 1. - unit_pos.x) * delta_dist.x; 
    } else { 
        side_dist.x = (unit_pos.x - map_pos.x) * delta_dist.x; 
    }
    if cam.dir.y > 0. { 
        side_dist.y = (map_pos.y + 1. - unit_pos.y) * delta_dist.y; 
    } else { 
        side_dist.y = (unit_pos.y - map_pos.y) * delta_dist.y; 
    }

    let step = Vec2::new(
        if cam.dir.x > 0.0 { 1. } else { -1. },
        if cam.dir.y > 0.0 { 1. } else { -1. },
    );

    let mut hit_x: bool = false;
    while map.data[((map_pos.y as usize) * map.width) + (map_pos.x as usize)] == 0 {

        let t = side_dist.x.min(side_dist.y);
        let hit = unit_pos + cam.dir * t;
        let new_pos = scale_up_position(hit);

        draw_circle(new_pos.x, new_pos.y, 3., YELLOW);
        if side_dist.x < side_dist.y {
            hit_x = true;
            side_dist.x += delta_dist.x;
            map_pos.x += step.x;
        } else {
            hit_x = false;
            side_dist.y += delta_dist.y;
            map_pos.y += step.y;
        }
    }
    test_project_ray(hit_x, delta_dist, side_dist);

}

fn test_project_ray(hit_x: bool, delta_dist: Vec2, side_dist: Vec2) {
    let perp_dist = if hit_x {
        side_dist.x - delta_dist.x
    } else {
        side_dist.y - delta_dist.y
    };

    let line_height = screen_height() / perp_dist;
    let x_pos = screen_width() / 2.;
    let start = screen_height() / 2. - line_height / 2.;
    let end = screen_height() / 2. + line_height / 2.;

    draw_line(x_pos, start, x_pos, end, 5., LIME);
}

fn scale_up_position(pos: Vec2) -> Vec2 {
    Vec2::new(
        pos.x * CELL_SIZE,
        pos.y * CELL_SIZE)
}

fn scale_down_position(cam_pos: Vec2) -> Vec2 {
    Vec2::new(
        cam_pos.x / CELL_SIZE, 
        cam_pos.y / CELL_SIZE)
}

fn map_position(pos: Vec2) -> Vec2 {
    Vec2::new(
        pos.x.floor(), 
        pos.y.floor())
}