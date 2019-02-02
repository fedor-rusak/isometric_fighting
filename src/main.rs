extern crate cgmath;
extern crate ggez;

use std::env;
use std::path;
use std::time::Duration;

use ggez::event::{self, Button, KeyCode, KeyMods};
use ggez::*;

struct InputState {
    xaxis: f32,
    yaxis: f32,
    speed: f32
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            speed: 2.0
        }
    }
}

struct GameState {
    pos_x: f32,
    pos_y: f32,
    fps: u32,
    input: InputState,
    time_passed_from_last_frame: Duration,
    floor: graphics::Image,
    avatar: graphics::Image,
    avatar_other_angle: graphics::Image,
    sound: audio::Source,
}

impl GameState {

    fn new(ctx: &mut Context, fps: u32) -> GameResult<GameState> {
        let floor_tile = graphics::Image::new(ctx, "/tile.png")?;
        let avatar_face = graphics::Image::new(ctx, "/avatar.png")?;
        let avatar_face_other_angle = graphics::Image::new(ctx, "/avatar_other_angle.png")?;
        let grass_step = audio::Source::new(ctx, "/grass_foot_step.ogg")?;

        let state = GameState {
            pos_x: 20.0,
            pos_y: 20.0,
            fps: fps,
            input: InputState::default(),
            time_passed_from_last_frame: Duration::new(0, 0),
            floor: floor_tile,
            avatar: avatar_face,
            avatar_other_angle: avatar_face_other_angle,
            sound : grass_step
        };

        Ok(state)
    }
  
}

impl ggez::event::EventHandler for GameState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut timeframe = Duration::new(0, 0);

        self.pos_x += self.input.xaxis * (1.0/0.6) * self.input.speed;
        self.pos_y += self.input.yaxis * self.input.speed;

        while timer::check_update_time(ctx, self.fps) {
            timeframe = timeframe + timer::delta(ctx);
        }
 
        self.time_passed_from_last_frame = timeframe;

        if (self.input.xaxis != 0.0 || self.input.yaxis != 0.0) && self.sound.playing() == false {
            let _result = self.sound.play();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("Draw event! time_passed_from_last_frame = {}ns", self.time_passed_from_last_frame.subsec_nanos());

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        //floor
        for i in 0..8 {
            for j in 0..10 {
                let dst = cgmath::Point2::new(100.0*i as f32, 60.0*j as f32);
                graphics::draw(ctx, &self.floor, (dst,))?;
            }
        }

        //avatar
        let dst = cgmath::Point2::new(self.pos_x, self.pos_y);

        let mut to_draw = &self.avatar_other_angle;

        if (self.input.xaxis.abs() + self.input.yaxis.abs()) == 2.0 {
            to_draw = &self.avatar;
        }
        graphics::draw(ctx, to_draw, (dst,))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {
                self.input.yaxis = -1.0;
            },
            KeyCode::Down => {
                self.input.yaxis = 1.0;
            },
            KeyCode::Left => {
                self.input.xaxis = -1.0;
            }
            KeyCode::Right => {
                self.input.xaxis = 1.0;
            }
            KeyCode::Escape => quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up | KeyCode::Down => {
                self.input.yaxis = 0.0;
            }
            KeyCode::Left | KeyCode::Right => {
                self.input.xaxis = 0.0;
            }
            KeyCode::Space => {
                self.pos_x = 400.0 - 50.0; //because image are drawn from left top corner and not from center
                self.pos_y = 300.0 - 50.0; // and current avatar image is 100x100
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