#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    pub fn rotate_right(&self) -> Self {
        match &self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    pub fn rotate_left(&self) -> Self {
        match &self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, PartialOrd, Ord)]
pub struct Position(pub Orientation, pub i64, pub i64);

impl Position {
    pub fn rotate_right(&self) -> Self {
        Position(self.0.rotate_left(), self.1, self.2)
    }
    pub fn rotate_left(&self) -> Self {
        Position(self.0.rotate_right(), self.1, self.2)
    }
    pub fn walk(&self) -> Self {
        match self.0 {
            Orientation::Up => Position(self.0, self.1 - 1, self.2),
            Orientation::Right => Position(self.0, self.1, self.2 + 1),
            Orientation::Down => Position(self.0, self.1 + 1, self.2),
            Orientation::Left => Position(self.0, self.1, self.2 - 1),
        }
    }

    pub fn to_coord(&self) -> Coord {
      Coord(self.1, self.2)
    }
}

pub trait Coor {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn j(&self) -> i64;
    fn i(&self) -> i64;
}

impl Coor for Position {
    fn j(&self) -> i64 {
        self.1
    }

    fn i(&self) -> i64 {
        self.2
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Coord(pub i64, pub i64);

impl Coor for Coord {
    fn j(&self) -> i64 {
        self.0
    }

    fn i(&self) -> i64 {
        self.1
    }
}

impl Coor for (i64, i64) {
    fn j(&self) -> i64 {
        self.0
    }

    fn i(&self) -> i64 {
        self.1
    }
}

impl Coor for (usize, usize) {
    fn j(&self) -> i64 {
        self.0 as i64
    }

    fn i(&self) -> i64 {
        self.1 as i64
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T>(pub Vec<Vec<T>>)
where
    T: Copy + Clone + ToString;

#[allow(dead_code)]
impl<T: Copy + Clone + ToString> Grid<T> {
    pub fn get(&self, c: &impl Coor) -> T {
        self.0[c.j() as usize][c.i() as usize]
    }

    pub fn y(&self) -> i64 {
        self.0.len() as i64
    }

    pub fn x(&self) -> i64 {
        self.0[0].len() as i64
    }

    pub fn within_bounds(&self, c: &impl Coor) -> bool {
        let j = c.j();
        let i = c.i();

        j >= 0 && j < self.y() && i >= 0 && i < self.x()
    }

    pub fn print(&self) {
        for line in &self.0 {
            println!("{}", line.iter().map(|i| i.to_string()).collect::<String>());
        }
    }
}
