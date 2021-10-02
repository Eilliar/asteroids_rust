use bracket_lib::prelude::*;
// Constants
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 =50;
const FRAME_DURATION: f32 = 75.0;

// Enums
enum GameMode {
    Menu,
    Playing,
    End,
}

// Structs
// State represents a snapshot of the current game
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

// Traits
impl State {
    fn new() -> Self {
        State { 
            player: Player::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm){
        ctx.cls();
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }
        
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.thrust(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
        
        self.player.render(ctx);
        ctx.print_centered(SCREEN_HEIGHT - 3, "Press SPACE to Thrust.");
        ctx.print_centered(SCREEN_HEIGHT - 2, "(Q) to Quit.");
    }

    fn restart(&mut self){
        self.player = Player::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Another Asteroids Clone");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {    
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.x, 
            self.y, 
            YELLOW, 
            BLACK, 
            to_cp437('A'));
    }

    fn thrust(&mut self) {
        // TODO add thurst logic
    }
}


// Main
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Another Asteroids Clone")
        .build()?; // ? operator requires a Result Type to be returned

    main_loop(context, State::new())
}
