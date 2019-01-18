extern crate cgmath;
extern crate ggez;

use std::env;
use std::path;
use std::time::Duration;

use ggez::event::{self, Axis, Button, KeyCode, KeyMods, MouseButton};
use ggez::*;

struct GameState {
    fps: u32,
    time_passed_from_last_frame: Duration,
    avatar: graphics::Image
}

impl GameState {

    fn new(ctx: &mut Context, fps: u32) -> GameResult<GameState> {
        let image1 = graphics::Image::new(ctx, "/dragon1.png")?;

        let state = GameState {
            fps: fps,
            time_passed_from_last_frame: Duration::new(0, 0),
            avatar: image1
        };

        Ok(state)
    }
  
}

impl ggez::event::EventHandler for GameState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut timeframe = Duration::new(0, 0);

        while timer::check_update_time(ctx, self.fps) {
            timeframe = timeframe + timer::delta(ctx);
        }
 
        self.time_passed_from_last_frame = timeframe;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("Draw event! time_passed_from_last_frame = {}ns", self.time_passed_from_last_frame.subsec_nanos());

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let dst = cgmath::Point2::new(20.0, 20.0);
        graphics::draw(ctx, &self.avatar, (dst,))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Escape => quit(ctx),
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

    event::run(ctx, event_loop, state);

    println!("Demo finished successfully!");
}