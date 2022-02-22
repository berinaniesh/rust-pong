use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

// struct MainState {
// }
// 
// impl event::EventHandler for MainState {
//     fn update(&mut self, ctx: &mut Context) -> GameResult {
//         Ok(())
//     }
//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         Ok(())
//     }
// }
// 
// impl MainState {
//     pub fn new() -> Self {
//         MainState{}
//     }
// }


fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rust-pong", "Berin Aniesh");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Pong written in rust");
    //let mut state = MainState::new();
    //event::run(ctx, event_loop, &mut state);
    Ok(())
}
