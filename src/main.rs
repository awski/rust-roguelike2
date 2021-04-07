use tcod::{Console, colors::WHITE};

const FPS_LIMIT: i32 = 30;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

struct World {
    root: tcod::console::Root,
}
impl World {
    fn handle_keys(&mut self) -> bool {
        use tcod::input::{Key, KeyCode};

        let key = self.root.wait_for_keypress(true);
        match key {
            Key { code: KeyCode::Up, .. } => {},
            Key { code: KeyCode::Left, .. } => {}
            Key { code: KeyCode::Right, .. } => {}
            Key { code: KeyCode::Down, .. } => {}
            Key { code: KeyCode::Enter, alt: true, .. } => {
                self.root.set_fullscreen(!self.root.is_fullscreen());
            }
            Key { code: KeyCode::Escape, .. } => { return true; }
            _ => {}
        }

        false
    }
}

struct Player {
    pos_x: i32,
    pos_y: i32,
    glyph: char,
}

impl Player {
    fn new() -> Self {
        Player {
            pos_x: 0,
            pos_y: 0,
            glyph: '@',
        }
    }

    fn set_pos(&mut self, pos_x: i32, pos_y: i32) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    } 
}

fn main() {
    let root = tcod::console::Root::initializer()
        .font("res/arial10x10.png", tcod::console::FontLayout::Tcod)
        .font_type(tcod::console::FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rust-roguelike2")
        .init();
    
    let mut world = World { root };
    let mut player = Player::new();
    player.set_pos(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);

    tcod::system::set_fps(FPS_LIMIT);

    while !world.root.window_closed() {
        world.root.set_default_foreground(tcod::colors::WHITE);
        world.root.clear();
        world.root.put_char(player.pos_x, player.pos_y, player.glyph, tcod::console::BackgroundFlag::None);
        world.root.flush();
        if world.handle_keys() {
            break;
        }
    }
}