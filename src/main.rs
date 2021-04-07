mod map;

use tcod::{Console, colors::WHITE};

const FPS_LIMIT: i32 = 30;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

type Map = Vec<Vec<Tile>>;

const COLOR_DARK_WALL: tcod::colors::Color = tcod::colors::Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: tcod::colors::Color = tcod::colors::Color {
    r: 50,
    g: 50,
    b: 150,
};

struct World {
    root: tcod::console::Root,
    con: tcod::console::Offscreen,
    map: Map,
}
impl World {
    fn handle_keys(&mut self, player: &mut Object) -> bool {
        use tcod::input::{Key, KeyCode};

        let key = self.root.wait_for_keypress(true);
        match key {
            Key { code: KeyCode::Up, .. } =>    { player.move_by(self, 0 ,-1); },
            Key { code: KeyCode::Left, .. } =>  { player.move_by(self, -1, 0); },
            Key { code: KeyCode::Right, .. } => { player.move_by(self, 1 , 0); },
            Key { code: KeyCode::Down, .. } =>  { player.move_by(self, 0 , 1); },
            Key { code: KeyCode::Enter, alt: true, .. } => {
                self.root.set_fullscreen(!self.root.is_fullscreen());
            },
            Key { code: KeyCode::Escape, .. } => { return true; },
            _ => {}
        }

        false
    }
    fn create_map() -> Map {
        let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        map[40][20] = Tile::wall();
        map[30][20] = Tile::wall();

        map
    }
    fn render_all(&mut self, objects: &[Object]) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let wall = self.map[x as usize][y as usize].blocked_sight;
                if wall {
                    self.con.set_char_background(x, y, COLOR_DARK_WALL, tcod::console::BackgroundFlag::Set);
                } else {
                    self.con.set_char_background(x, y, COLOR_DARK_GROUND, tcod::console::BackgroundFlag::Set);
                }
            }
        }

        for obj in objects {
            obj.draw(&mut self.con);
        }

        tcod::console::blit(
            &self.con,
            (0,0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut self.root,
            (0,0),
            1.0,
            1.0);
    }
}

#[derive(Clone)]
struct Tile {
    blocked: bool,
    blocked_sight: bool,
}
impl Tile {
    fn empty() -> Self {
        Tile { blocked: false, blocked_sight: false }
    }
    fn wall() -> Self {
        Tile { blocked: true, blocked_sight: true }
    }
}

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
    fn move_by(&mut self, world: &World, dx: i32, dy: i32) {
        if !world.map[(self.pos_x + dx) as usize][(self.pos_y + dy) as usize].blocked {
            self.pos_x += dx;
            self.pos_y += dy;
        }
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
    let main_con = tcod::console::Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut map = World::create_map();
    let mut world = World { root, con: main_con, map };
    let mut player = Object::new(10,10,'@',tcod::colors::WHITE);
    let mut npc = Object::new(25,25,'@',tcod::colors::YELLOW);

    let mut objects = Vec::new();
    objects.push(player);
    objects.push(npc);

    tcod::system::set_fps(FPS_LIMIT);

    while !world.root.window_closed() {
        world.con.clear();

        for obj in &objects {
            obj.draw(&mut world.con);
        }

        world.render_all(&objects);

        world.root.flush();
        //TODO: fix player ref(by object id?)
        if world.handle_keys(&mut objects[0]) {
            break;
        }
    }
}