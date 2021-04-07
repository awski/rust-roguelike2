use tcod::{Console, colors::WHITE};

const FPS_LIMIT: i32 = 30;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

struct World {
    root: tcod::console::Root,
    con: tcod::console::Offscreen,
}
impl World {
    fn handle_keys(&mut self, player: &mut Object) -> bool {
        use tcod::input::{Key, KeyCode};

        let key = self.root.wait_for_keypress(true);
        match key {
            Key { code: KeyCode::Up, .. } =>    { player.move_by(0 ,-1); },
            Key { code: KeyCode::Left, .. } =>  { player.move_by(-1, 0); },
            Key { code: KeyCode::Right, .. } => { player.move_by(1 , 0); },
            Key { code: KeyCode::Down, .. } =>  { player.move_by(0 , 1); },
            Key { code: KeyCode::Enter, alt: true, .. } => {
                self.root.set_fullscreen(!self.root.is_fullscreen());
            },
            Key { code: KeyCode::Escape, .. } => { return true; },
            _ => {}
        }

        false
    }
}

// struct Player {
//     pos_x: i32,
//     pos_y: i32,
//     glyph: char,
// }

// impl Player {
//     fn new() -> Self {
//         Player {
//             pos_x: 0,
//             pos_y: 0,
//             glyph: '@',
//         }
//     }

//     fn set_pos(&mut self, pos_x: i32, pos_y: i32) {
//         self.pos_x = pos_x;
//         self.pos_y = pos_y;
//     } 
// }

struct Object {
    pos_x: i32,
    pos_y: i32,
    glyph: char,
    color: tcod::colors::Color,
}
impl Object {
    fn new(x: i32, y: i32, glyph: char, color: tcod::colors::Color) -> Self {
        Object { pos_x: x, pos_y: y, glyph, color} 
    }
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.pos_x += dx;
        self.pos_y += dy;
    }
    fn draw(&self, console: &mut dyn tcod::console::Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.pos_x, self.pos_y, self.glyph, tcod::console::BackgroundFlag::None);
    }
}


fn main() {
    let root = tcod::console::Root::initializer()
        .font("res/arial10x10.png", tcod::console::FontLayout::Tcod)
        .font_type(tcod::console::FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rust-roguelike2")
        .init();
    let main_con = tcod::console::Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut world = World { root, con: main_con };

    let mut player = Object::new(10,10,'@',tcod::colors::WHITE);
    let mut npc = Object::new(25,25,'@',tcod::colors::YELLOW);

    let mut objects = Vec::new();
    objects.push(player);
    objects.push(npc);

    tcod::system::set_fps(FPS_LIMIT);

    while !world.root.window_closed() {
        world.con.set_default_foreground(tcod::colors::WHITE);
        world.con.clear();

        for obj in &objects {
            obj.draw(&mut world.con);
        }

        tcod::console::blit(
            &world.con,
            (0,0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut world.root,
            (0,0),
            1.0,
            1.0);

        world.root.flush();
        //TODO: fix player ref(by object id?)
        if world.handle_keys(&mut objects[0]) {
            break;
        }
    }
}