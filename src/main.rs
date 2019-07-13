extern crate cgmath;
extern crate ggez;

use std::collections::HashMap;
use std::env;
use std::path;
use std::time::Duration;

use ggez::audio::SoundSource;
use ggez::event::{self, KeyCode, KeyMods};
use ggez::*;

struct AvatarState {
    pos_x: f32,
    pos_y: f32,
    direction_is_diagonal: bool,
}

struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    speed: f32,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            speed: 1.5,
        }
    }
}

struct TileDimensions {
    world_width: f32,
    world_length: f32,
    projected_width: f32,
    projected_height: f32,
}

struct AvatarImgStruct {
    width: f32,
    height: f32,
    avatar: graphics::Image,
    avatar_other_angle: graphics::Image,
}

struct FloorImgStruct {
    width: f32,
    // height: f32, //not used :\
    default: graphics::Image,
    colored: graphics::Image,
}

struct Projection {
    width: f32,
    height: f32,
    camera_center_pos_x: f32, //current camera X in 'world' coords
    camera_center_pos_y: f32, //current camera Y in 'world' coords
}

struct GameState {
    avatar_state: AvatarState,
    fps: u32,
    tile_dimensions: TileDimensions,
    input: InputState,
    time_passed_from_last_frame: Duration,
    projection: Projection,
    visited_tiles_map: HashMap<String, bool>,
    floor_img_struct: FloorImgStruct,
    avatar_img_struct: AvatarImgStruct,
    sound: audio::Source,
    background_audio: audio::Source,
}

impl GameState {
    fn new(ctx: &mut Context, fps: u32) -> GameResult<GameState> {
        let floor_tile = graphics::Image::new(ctx, "/tile.png")?;
        let floor_tile_colored = graphics::Image::new(ctx, "/tile_colored.png")?;
        let avatar_face = graphics::Image::new(ctx, "/avatar.png")?;
        let avatar_face_other_angle = graphics::Image::new(ctx, "/avatar_other_angle.png")?;
        let mut grass_step = audio::Source::new(ctx, "/grass_foot_step.ogg")?;
        grass_step.set_volume(0.5);
        let mut river_and_birds = audio::Source::new(ctx, "/river_and_birds.ogg")?;
        river_and_birds.set_repeat(true);

        let state = GameState {
            avatar_state: AvatarState {
                pos_x: 15.0, //current avatar X in 'world' coords
                pos_y: 15.0, //current avatar Y in 'world' coords
                direction_is_diagonal: false,
            },
            fps,
            tile_dimensions: TileDimensions {
                world_width: 30.0,
                world_length: 30.0,
                projected_width: 50.0, //if we IGNORE Y this is as max difference from 0 to world width when projected
                projected_height: 30.0, //if we IGNORE X this is as max difference from 0 to world length when projected
            },
            input: InputState::default(),
            time_passed_from_last_frame: Duration::new(0, 0),
            projection: Projection {
                width: 800.0,
                height: 600.0,
                camera_center_pos_x: 90.0,
                camera_center_pos_y: 30.0,
            },
            visited_tiles_map: HashMap::new(),
            floor_img_struct: FloorImgStruct {
                width: 100.0,
                // height: 60.0,  /not used :\
                default: floor_tile,
                colored: floor_tile_colored,
            },
            avatar_img_struct: AvatarImgStruct {
                width: 100.0,
                height: 100.0,
                avatar: avatar_face,
                avatar_other_angle: avatar_face_other_angle,
            },
            sound: grass_step,
            background_audio: river_and_birds,
        };

        Ok(state)
    }
}

/// This method *hides* some complexity to move things in isometric perspective
/// using actually 3d coords.
fn project(
    projection: &Projection,
    tile_dimensions: &TileDimensions,
    x: f32,
    y: f32,
) -> (f32, f32) {
    let pixels_moved_per_x_one_step = tile_dimensions.projected_width / tile_dimensions.world_width;
    let pixels_moved_per_y_one_step =
        tile_dimensions.projected_height / tile_dimensions.world_length;

    let camera_shift_x = (projection.width / 2.0)
        - (projection.camera_center_pos_x - projection.camera_center_pos_y)
            * pixels_moved_per_x_one_step;
    let camera_shift_y = (projection.height / 2.0)
        - (projection.camera_center_pos_x + projection.camera_center_pos_y)
            * pixels_moved_per_y_one_step;

    let result_x = camera_shift_x + (x - y) * pixels_moved_per_x_one_step;
    let result_y = camera_shift_y + (x + y) * pixels_moved_per_y_one_step;

    (result_x, result_y)
}

fn handle_movement_input(input_state: &InputState, x: f32, y: f32) -> (f32, f32) {
    //movement is calculated in 'world' coordinates. NOT projection pixels!
    let modifier =
        if (input_state.up || input_state.down) && (input_state.left || input_state.right) {
            0.85 //diagonal movement for 1.0 means sin45*1.0
        } else {
            1.0
        };

    let xaxis = if (input_state.left && !input_state.down) || (input_state.up && !input_state.right)
    {
        -1.0
    } else if (!input_state.left && input_state.down) || (!input_state.up && input_state.right) {
        1.0
    } else {
        0.0
    };

    let yaxis = if (input_state.left && !input_state.up) || (input_state.down && !input_state.right)
    {
        1.0
    } else if (!input_state.left && input_state.up) || (!input_state.down && input_state.right) {
        -1.0
    } else {
        0.0
    };

    let result_x = x + xaxis * input_state.speed * modifier;
    let result_y = y + yaxis * input_state.speed * modifier;

    (result_x, result_y)
}

fn is_moving(input_state: &InputState) -> bool {
    input_state.up || input_state.down || input_state.left || input_state.right
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut timeframe = Duration::new(0, 0);

        //movement
        let (new_x, new_y) = handle_movement_input(
            &self.input,
            self.avatar_state.pos_x,
            self.avatar_state.pos_y,
        );
        self.avatar_state.pos_x = new_x;
        self.avatar_state.pos_y = new_y;
        self.projection.camera_center_pos_x = new_x;
        self.projection.camera_center_pos_y = new_y;

        if is_moving(&self.input) {
            self.avatar_state.direction_is_diagonal =
                (self.input.up || self.input.down) && (self.input.left || self.input.right);
        }

        //sound
        if is_moving(&self.input) && !self.sound.playing() {
            let _result = self.sound.play();
        }

        if !self.background_audio.playing() {
            let _result = self.background_audio.play();
        }

        //collision detection
        let tile_i = (self.avatar_state.pos_x / self.tile_dimensions.world_width) as i32;
        let tile_j = (self.avatar_state.pos_y / self.tile_dimensions.world_length) as i32;

        let key = format!("{}_{}", tile_i, tile_j);

        self.visited_tiles_map.entry(key).or_insert(true);

        //frame time stabilizer
        while timer::check_update_time(ctx, self.fps) {
            timeframe += timer::delta(ctx);
        }

        self.time_passed_from_last_frame = timeframe;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("Draw event! time_passed_from_last_frame = {}ns", self.time_passed_from_last_frame.subsec_nanos());

        //we fill screen with fixed color
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        //floor drawing
        for i in 0..20 {
            for j in 0..20 {
                let tile_start_pos_x = self.tile_dimensions.world_width * i as f32;
                let tile_start_pos_y = self.tile_dimensions.world_length * j as f32;

                let (x, y) = project(
                    &self.projection,
                    &self.tile_dimensions,
                    tile_start_pos_x,
                    tile_start_pos_y,
                );
                //because 0,0 of tile is top,center of actual image in isometric projection
                let render_coords = cgmath::Point2::new(x - self.floor_img_struct.width / 2.0, y);

                let key = format!("{}_{}", i, j);

                let image = if self.visited_tiles_map.contains_key(&key) {
                    &self.floor_img_struct.colored
                } else {
                    &self.floor_img_struct.default
                };

                graphics::draw(ctx, image, (render_coords,))?;
            }
        }

        //avatar
        {
            let (avatar_x, avatar_y) = project(
                &self.projection,
                &self.tile_dimensions,
                self.avatar_state.pos_x,
                self.avatar_state.pos_y,
            );
            //because avatar image center (ant not left top corner) represents character position
            let render_coords = cgmath::Point2::new(
                avatar_x - self.avatar_img_struct.width / 2.0,
                avatar_y - self.avatar_img_struct.height / 2.0,
            );

            let to_draw = if self.avatar_state.direction_is_diagonal {
                &self.avatar_img_struct.avatar
            } else {
                &self.avatar_img_struct.avatar_other_angle
            };

            graphics::draw(ctx, to_draw, (render_coords,))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => {
                self.input.up = true;
                self.input.down = false;
            }
            KeyCode::Down => {
                self.input.down = true;
                self.input.up = false;
            }
            KeyCode::Left => {
                self.input.left = true;
                self.input.right = false;
            }
            KeyCode::Right => {
                self.input.right = true;
                self.input.left = false;
            }
            KeyCode::Escape => quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Down => {
                self.input.down = false;
            }
            KeyCode::Up => {
                self.input.up = false;
            }
            KeyCode::Left => {
                self.input.left = false;
            }
            KeyCode::Right => {
                self.input.right = false;
            }
            KeyCode::Space => {
                self.avatar_state.pos_x = 15.0;
                self.avatar_state.pos_y = 15.0;
                self.projection.camera_center_pos_x = 15.0;
                self.projection.camera_center_pos_y = 15.0;
                self.visited_tiles_map = HashMap::new();
            }
            _ => (), // Do nothing
        }
    }
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let config = conf::Conf::new();

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "isometric_fighting")
        .add_resource_path(resource_dir)
        .conf(config)
        .build()
        .unwrap();

    const DESIRED_FPS: u32 = 60;
    let state = &mut GameState::new(ctx, DESIRED_FPS).unwrap();

    let _result = event::run(ctx, event_loop, state);

    println!("Demo finished successfully!");
}
