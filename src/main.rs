use macroquad::prelude::*;
use macroquad::window::*;
use macroquad::audio::*;

mod palette;

const CELL_SIZE: f32 = 32.;
const BRIGHTNESS_FACTOR: f32 = 1.5;

const CAMERA_SIZE: f32 = 8.;
const CAMERA_SPEED: f32 = 1.5;
const CAMERA_ROT_SPEED: f32 = 5.;

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

struct Camera {
    fov: f32,
    angle: f32,
    pos: Vec2,
    dir: Vec2,
    vel: Vec2,
}

struct Texture {
    image: Image,
    pixels: Vec<[u8;4]>
}

#[macroquad::main("Raycaster")]
async fn main() {
    request_new_screen_size(1080., 720.);

    let theme_music = load_sound("resources/sounds/theme.ogg").await.unwrap();
    play_sound(
        &theme_music, 
        PlaySoundParams {
            looped: true,
            volume: 0.75
    });

    let sky_image = load_texture("resources/sprites/sky.png").await.unwrap();
    let shotgun_image = load_texture("resources/sprites/shotgun.png").await.unwrap();
    
    let textures = [
        load_texture_data(load_image("resources/textures/banner.png").await.unwrap()),
        load_texture_data(load_image("resources/textures/basic_wall.png").await.unwrap()),
        load_texture_data(load_image("resources/textures/brick_wall.png").await.unwrap())
    ];

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
        angle: 0.,
        pos: Vec2::new(40., 40.),
        dir: Vec2::new(0., 0.),  
        vel: Vec2::new(0., 0.),
    };

    let mut min_map_visible = false;
    let mut gun_visible = false;

    loop {
        clear_background(palette::BLACK);

        // Background
        draw_texture_ex(
            &sky_image, 
            0., 
            0., 
            WHITE,
            DrawTextureParams { 
                dest_size: Some(vec2(screen_width(), screen_height() / 2.)), 
                ..Default::default()
            }
        );
        draw_rectangle(0., (screen_height()) / 2. - 30., screen_width(), (screen_height()) / 2. + 30., palette::BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::Tab) {
            min_map_visible = !min_map_visible;
        } else if is_key_pressed(KeyCode::H) {
            gun_visible = !gun_visible;
        }

        camera_dir(&mut cam);
        update_cam_pos(&mut cam);
        camera_collisions(&mut cam, &map);

        projection(&cam, &map);

        if min_map_visible {
            draw_min_map(&map);
            draw_camera(&cam);
        }

        // Shotgun
        if gun_visible {
            draw_texture_ex(
                &shotgun_image, 
                screen_width() / 2. - shotgun_image.width() * 0.5 / 2., 
                screen_height() - shotgun_image.height() * 0.5 + 30., 
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        shotgun_image.width() * 0.5,
                        shotgun_image.height() * 0.5,
                    )),
                    ..Default::default()
                },
            )
        };

        let fps = format!("FPS: {}", get_fps());
        draw_text(&fps, 20., 40., 30., GREEN);

        next_frame().await
    }
}

fn load_texture_data(texture: Image) -> Texture {
    let texture_data= texture.clone();
    Texture {
        image: texture,
        pixels: texture_data.clone().get_image_data().to_vec()
    }
}

fn draw_min_map(map: &Map) {
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
    if is_key_down(KeyCode::D) {
        cam.angle -= CAMERA_ROT_SPEED;
    } else if  is_key_down(KeyCode::A) {
        cam.angle += CAMERA_ROT_SPEED;
    }

    cam.dir = polar_to_cartesian(1., cam.angle.to_radians());
}

fn update_cam_pos(cam: &mut Camera) {
    let dir: f32;

    if is_key_down(KeyCode::W) {
        dir = 1.;
    } else if is_key_down(KeyCode::S) {
        dir = -1.;
    } else {
        dir = 0.;
    }

    cam.vel = cam.dir * dir * CAMERA_SPEED;
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

fn projection(cam: &Camera, map: &Map) {
    let unit_pos = scale_down_position(cam.pos);
    let map_pos = map_position(unit_pos);

    let fov_factor = (cam.fov / 2.).tan();
    let cam_plane = Vec2::new(cam.dir.y * fov_factor, -cam.dir.x * fov_factor);

    for x in 0..(screen_width() as usize) {
        let x_draw_pos = 2. * x as f32 / screen_width() - 1.0;
        let ray_dir = Vec2::new(
            cam.dir.x + cam_plane.x * x_draw_pos,
            cam.dir.y + cam_plane.y * x_draw_pos);
        single_ray_projection(&unit_pos, &map_pos, map, ray_dir, &(x as f32));
    }
}

fn single_ray_projection(unit_pos: &Vec2, map_pos: &Vec2, map: &Map, dir: Vec2, x_draw: &f32) {
    let mut map_pos = *map_pos;

    let step = Vec2::new(
        if dir.x > 0. { 1. } else {-1.},
        if dir.y > 0. { 1. } else {-1.},
    );

    let delta_dist = Vec2::new(
        1. / dir.x.abs(),
        1. / dir.y.abs());
    
    let mut side_dist = Vec2::new(
        if dir.x > 0. { (map_pos.x + 1. - unit_pos.x) * delta_dist.x }
        else { (unit_pos.x - map_pos.x) * delta_dist.x },
        if dir.y > 0. { (map_pos.y + 1. - unit_pos.y) * delta_dist.y }
        else { (unit_pos.y - map_pos.y) * delta_dist.y },
    );

    let mut hit_x = false;
    let mut i: usize = 0;

    while map.data[ ((map_pos.y as usize) * map.width) + (map_pos.x as usize) ] == 0 && i <= map.width * map.height {
        if side_dist.x < side_dist.y {
            hit_x = true;
            side_dist.x += delta_dist.x;
            map_pos.x += step.x;
        } else {
            hit_x = false;
            side_dist.y += delta_dist.y;
            map_pos.y += step.y;
        }
        i += 1;
    }

    project_ray(hit_x, delta_dist, side_dist, x_draw);
}

fn project_ray(hit_x: bool, delta_dist: Vec2, side_dist: Vec2, x_pos: &f32) {
    let perp_dist = if hit_x {
        side_dist.x - delta_dist.x
    } else {
        side_dist.y - delta_dist.y
    };

    let line_height = (screen_height() / perp_dist).min(screen_height());
    let start = screen_height() / 2. - line_height / 2.;
    let end = screen_height() / 2. + line_height / 2.;

    draw_line(
        *x_pos, 
        start, 
        *x_pos, 
        end, 
        1., 
        palette::pseudo_light_interpolation(LIGHTGRAY, ( BRIGHTNESS_FACTOR / perp_dist ).min(1.)));
}

fn _scale_up_position(pos: Vec2) -> Vec2 {
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