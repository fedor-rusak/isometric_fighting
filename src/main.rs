use ggez::*;

struct GameState {
    dt: std::time::Duration
}

impl ggez::event::EventHandler for GameState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }
}



fn main() {
    let state = &mut GameState { dt: std::time::Duration::new(0, 0) };

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) =
        ContextBuilder::new("hello_ggez", "isometric_fighting")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);

    println!("Works!");
}