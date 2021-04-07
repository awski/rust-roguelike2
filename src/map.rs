//TODO(#1): rework map impl

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

pub fn create_map() {
    unimplemented!();
}

pub fn create_room() {
    unimplemented!();
}

pub fn render_map() {
    unimplemented!();
}
