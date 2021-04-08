//TODO(#1): rework map impl

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone)]
pub struct Tile {
    pub blocked: bool,
    pub blocked_sight: bool,
}
impl Tile {
    pub fn empty() -> Self {
        Tile { blocked: false, blocked_sight: false }
    }
    pub fn wall() -> Self {
        Tile { blocked: true, blocked_sight: true }
    }
}

struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

pub fn xx_create_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map[40][20] = Tile::wall();
    map[30][20] = Tile::wall();

    map
}

pub fn create_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let room1 = Rect::new(20, 15, 10, 10);
    let room2 = Rect::new(50, 15, 10, 10);
    create_room(room1, &mut map);
    create_room(room2, &mut map);

    map
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

pub fn render_map() {
    unimplemented!();
}
