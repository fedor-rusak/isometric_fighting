extern crate cgmath;
extern crate ggez;

use std::env;
use std::path;
use std::time::Duration;
use std::collections::HashMap;

use ggez::event::{self, Button, KeyCode, KeyMods};
use ggez::*;

struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    speed: f32
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            speed: 1.5
        }
    }
}

struct GameState {
    pos_x: f32,
    pos_y: f32,
    fps: u32,
    input: InputState,
    time_passed_from_last_frame: Duration,
    zero_x: f32,
    zero_y: f32,
    edge_length: f32,
    visited_tiles_map: HashMap<String, bool>,
    floor: graphics::Image,
    floor_colored: graphics::Image,
    avatar: graphics::Image,
    avatar_other_angle: graphics::Image,
    sound: audio::Source,
    background_audio: audio::Source
}

impl GameState {

    fn new(ctx: &mut Context, fps: u32) -> GameResult<GameState> {
        let floor_tile = graphics::Image::new(ctx, "/tile.png")?;
        let floor_tile_colored = graphics::Image::new(ctx, "/tile_colored.png")?;
        let avatar_face = graphics::Image::new(ctx, "/avatar.png")?;
        let avatar_face_other_angle = graphics::Image::new(ctx, "/avatar_other_angle.png")?;
        let grass_step = audio::Source::new(ctx, "/grass_foot_step.ogg")?;
        let mut river_and_birds = audio::Source::new(ctx, "/river_and_birds.ogg")?;
        river_and_birds.set_repeat(true);

        let state = GameState {
            pos_x: 390.0,
            pos_y: 300.0,
            fps: fps,
            input: InputState::default(),
            time_passed_from_last_frame: Duration::new(0, 0),
            zero_x: 400.0,
            zero_y: -300.0,
            edge_length: 30.0,
            visited_tiles_map: HashMap::new(),
            floor: floor_tile,
            floor_colored: floor_tile_colored,
            avatar: avatar_face,
            avatar_other_angle: avatar_face_other_angle,
            sound : grass_step,
            background_audio: river_and_birds
        };

        Ok(state)
    }
  
}

impl ggez::event::EventHandler for GameState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut timeframe = Duration::new(0, 0);

        let mut modifier = 1.0;

        if (self.input.up || self.input.down) && (self.input.left || self.input.right) {
            modifier = 0.85; //diagonal movement for 1.0 means sin45*1.0
        }

        let mut xaxis = 0.0;
        let mut yaxis = 0.0;

        if (self.input.left && !self.input.down) || (self.input.up && !self.input.right) {
            xaxis = -1.0;
        }
        else if (!self.input.left && self.input.down) || (!self.input.up && self.input.right) {
            xaxis = 1.0;
        }

        if (self.input.left && !self.input.up) || (self.input.down && !self.input.right) {
            yaxis = 1.0;
        }
        else if (!self.input.left && self.input.up) || (!self.input.down && self.input.right) {
            yaxis = -1.0;
        }

        self.pos_x += xaxis * self.input.speed * modifier;
        self.pos_y += yaxis * self.input.speed * modifier;


        while timer::check_update_time(ctx, self.fps) {
            timeframe = timeframe + timer::delta(ctx);
        }


        self.time_passed_from_last_frame = timeframe;


        if (xaxis != 0.0 || yaxis != 0.0) && self.sound.playing() == false {
            let _result = self.sound.play();
            self.sound.set_volume(0.5);
        }

        if self.background_audio.playing() == false {
            let _result = self.background_audio.play();
        }

        //not sure why but needs this half part...
        let tile_i = (self.pos_x / self.edge_length + 0.5) as i32;
        let tile_j = (self.pos_y / self.edge_length + 0.5) as i32;

        let key = format!("{}_{}", tile_i, tile_j);

        if  !self.visited_tiles_map.contains_key(&key) {
            self.visited_tiles_map.insert(key, true);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("Draw event! time_passed_from_last_frame = {}ns", self.time_passed_from_last_frame.subsec_nanos());

        const TILE_HEIGHT_HALF: f32 = 30.0;
        const TILE_WIDTH_HALF: f32 = 50.0;

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        //floor
        for i in 0..20 {
            for j in 0..20 {
                let x = self.zero_x + TILE_WIDTH_HALF * (i-j) as f32;
                let y = self.zero_y + TILE_HEIGHT_HALF * (i+j) as f32;
                
                //because our tile is 100x60 and we have 'center coords' for it in x and y
                let dst = cgmath::Point2::new(x - TILE_WIDTH_HALF, y - TILE_HEIGHT_HALF);

                let key = format!("{}_{}",i,j);

                let mut image = &self.floor;
                if self.visited_tiles_map.contains_key(&key) {
                    image = &self.floor_colored;
                }

                graphics::draw(ctx, image, (dst,))?;
            }
        }


        {
            let avatar_x = self.zero_x + (self.pos_x - self.pos_y) * TILE_WIDTH_HALF / self.edge_length;
            let avatar_y = self.zero_y + (self.pos_x + self.pos_y) * TILE_HEIGHT_HALF / self.edge_length;

            //because images are drawn from left top corner and not from center
            //and current avatar image is 100x100
            let dst = cgmath::Point2::new(avatar_x-50.0, avatar_y-50.0);

            let mut to_draw = &self.avatar_other_angle;

            if (self.input.up || self.input.down) && (self.input.left || self.input.right) {
                to_draw = &self.avatar;
            }
            graphics::draw(ctx, to_draw, (dst,))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {
                self.input.up = true;
                self.input.down = false;
            },
            KeyCode::Down => {
                self.input.down = true;
                self.input.up = false;
            },
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
                self.pos_x = 390.0;
                self.pos_y = 300.0;
                self.visited_tiles_map = HashMap::new();
            }
            _ => (), // Do nothing
        }
    }

    fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, id: usize) {
        println!("Controller button pressed: {:?} Controller_Id: {}", btn, id);
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

    let (ref mut ctx, ref mut event_loop) =
        ContextBuilder::new("hello_ggez", "isometric_fighting")
        .add_resource_path(resource_dir)
        .conf(config)
        .build()
        .unwrap();


    const DESIRED_FPS: u32 = 60;
    let state = &mut GameState::new(ctx, DESIRED_FPS).unwrap();

    let _result = event::run(ctx, event_loop, state);

    println!("Demo finished successfully!");
}