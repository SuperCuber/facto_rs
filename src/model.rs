use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    ops::{Add, DerefMut, Mul},
};

use nannou::{lyon::lyon_tessellation::Orientation, prelude::*};
use rand::{distributions::Standard, prelude::Distribution};

use crate::constants::CELL_SIZE;

#[derive(Debug, Clone)]
pub struct Model {
    pub window: window::Id,
    pub grid: Grid,
    pub items: Vec<Item>,
}

// === Grid ===

#[derive(Debug, Clone, Default)]
pub struct Grid {
    pub grid_items: GridItems,
    pub trains: VecDeque<Train>,
}

pub type GridItems = BTreeMap<Position, GridItem>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub isize, pub isize);

#[derive(Debug, Clone)]
pub enum GridItem {
    Building(Building, Direction),
    Rail(Orientation),
    Intersection(IntersectionType),
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub item: Option<Item>,
    pub cooldown: f64,
}

#[derive(Debug, Clone)]
pub enum IntersectionType {
    /// Direction is the left corner
    Corner(Direction),
    /// Direction is the middle of the three
    Triple(Direction),
    Quad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
pub enum Building {
    Spawner {
        item: Item,
        timer: RefCell<f64>,
    },
    Crafter {
        item: Item,
        contents: RefCell<BTreeMap<Item, usize>>,
        timer: RefCell<f64>,
    },
    Submitter {
        item: Item,
        contents: RefCell<BTreeMap<Item, usize>>,
    },
}

// === Item ===
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub color: Srgb,
    pub components: BTreeMap<Item, usize>,
    pub time: f64,
}

#[derive(Clone, Debug)]
pub struct Train {
    pub item: Item,
    pub path: Vec<Position>,
    pub position: usize,
    pub sub_position: f64,
}

// === Utils ===

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Item {}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl GridItem {
    pub fn update(
        &self,
        position: &Position,
        update: &Update,
        grid_items: &GridItems,
        trains: &mut VecDeque<Train>,
    ) {
        match self {
            GridItem::Building(b, _) => b.update(position, update, grid_items, trains),
            GridItem::Rail(..) => {}
            GridItem::Intersection(_) => {}
        }
    }

    pub fn contents(&mut self) -> Option<impl DerefMut<Target = BTreeMap<Item, usize>> + '_> {
        if let GridItem::Building(
            Building::Crafter { contents, .. } | Building::Submitter { contents, .. },
            ..,
        ) = self
        {
            Some(contents.borrow_mut())
        } else {
            None
        }
    }

    pub fn neighbors(&self, self_position: Position) -> Vec<Position> {
        match self {
            GridItem::Building(_, d) => vec![self_position + *d],
            GridItem::Rail(Orientation::Vertical) => vec![
                self_position + Direction::North,
                self_position + Direction::South,
            ],
            GridItem::Rail(Orientation::Horizontal) => vec![
                self_position + Direction::East,
                self_position + Direction::West,
            ],
            GridItem::Intersection(IntersectionType::Quad) => vec![
                self_position + Direction::East,
                self_position + Direction::West,
                self_position + Direction::North,
                self_position + Direction::South,
            ],
            GridItem::Intersection(IntersectionType::Triple(d)) => vec![
                self_position + *d,
                self_position + d.left(),
                self_position + d.right(),
            ],
            GridItem::Intersection(IntersectionType::Corner(d)) => {
                vec![self_position + *d, self_position + d.right()]
            }
        }
    }
}

impl From<Position> for Vec2 {
    fn from(other: Position) -> Vec2 {
        Vec2::from((other.0 as f32 * CELL_SIZE, other.1 as f32 * CELL_SIZE))
    }
}

impl From<Direction> for f32 {
    fn from(other: Direction) -> f32 {
        match other {
            Direction::East => PI / 2.0 * 0.0,
            Direction::North => PI / 2.0 * 1.0,
            Direction::West => PI / 2.0 * 2.0,
            Direction::South => PI / 2.0 * 3.0,
        }
    }
}

impl Direction {
    pub fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Direction::West => Direction::North,
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn to_position(self) -> Position {
        // I'm lazy
        Position(0, 0) + self
    }

    pub fn to_orientation(self) -> Orientation {
        match self {
            Direction::North | Direction::South => Orientation::Vertical,
            Direction::East | Direction::West => Orientation::Horizontal,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Position(self.0, self.1 + 1),
            Direction::South => Position(self.0, self.1 - 1),
            Direction::East => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0 - 1, self.1),
        }
    }
}

impl Mul<isize> for Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        Position(self.0 * rhs, self.1 * rhs)
    }
}

impl Position {
    pub fn direction_towards(self, other: Position) -> Option<Direction> {
        match (self, other) {
            (Position(x1, _), Position(x2, _)) if x1 + 1 == x2 => Some(Direction::East),
            (Position(x1, _), Position(x2, _)) if x1 == x2 + 1 => Some(Direction::West),
            (Position(_, y1), Position(_, y2)) if y1 + 1 == y2 => Some(Direction::North),
            (Position(_, y1), Position(_, y2)) if y1 == y2 + 1 => Some(Direction::South),
            _ => None,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(1..=4) {
            1 => Direction::North,
            2 => Direction::East,
            3 => Direction::West,
            4 => Direction::South,
            _ => unreachable!(),
        }
    }
}
