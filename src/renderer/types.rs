pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down
}

