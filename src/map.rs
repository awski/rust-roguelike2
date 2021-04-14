
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

const MAP_MAX_ROOMS: u32 = 30;
const MAP_ROOM_MAX_SIZE: u32 = 10;
const MAP_ROOM_MIN_SIZE: u32 = 6;

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone)]
pub struct Tile {
    pub blocked: bool,
    pub blocked_sight: bool,
}
impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            blocked_sight: false,
        }
    }
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            blocked_sight: true,
        }
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
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2 / 2), (self.y1 + self.y2 / 2))
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

pub fn create_map() -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let room1 = Rect::new(20, 15, 10, 10);
    let room2 = Rect::new(50, 15, 10, 10);
    create_room(room1, &mut map);
    create_room(room2, &mut map);
    create_h_tunnel(25, 55, 23, &mut map);

    map
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn render_map() {
    unimplemented!();
}
