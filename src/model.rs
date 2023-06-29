use std::collections::BTreeMap;

use nannou::{lyon::lyon_tessellation::Orientation, prelude::*};

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
    pub grid_items: BTreeMap<Position, GridItem>,
    pub trains: Vec<Train>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, Clone)]
pub enum GridItem {
    Building(Building, Direction),
    Rail(Orientation),
    Intersection(Intersection, IntersectionType),
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub item: Option<Item>,
    pub cooldown: f64,
}

#[derive(Debug, Clone)]
pub enum IntersectionType {
    Triple(Direction),
    Quad,
}

#[derive(Debug, Clone, Copy)]
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
        timer: f64,
    },
    Crafter {
        item: Item,
        contents: Vec<Item>,
        timer: f64,
    },
    Submitter {
        contents: Vec<Item>,
    },
}

// === Item ===
#[derive(Clone, Debug)]
pub struct Item {
    pub color: Srgb,
    pub components: Vec<Item>,
    pub spawning_time: f64,
    pub crafting_time: f64,
}

#[derive(Clone, Debug)]
pub struct Train {
    pub item: Item,
    pub position: Position,
}

// === Utils ===
impl GridItem {
    pub fn update(&mut self, update: &Update) {
        match self {
            GridItem::Building(b, _) => b.update(update),
            GridItem::Rail(..) => {}
            GridItem::Intersection(i, _) => i.update(update),
        }
    }
}

impl From<Position> for Vec2 {
    fn from(other: Position) -> Vec2 {
        Vec2::from((other.0 as f32 * CELL_SIZE, other.1 as f32 * CELL_SIZE))
    }
}

impl From<&Direction> for f32 {
    fn from(other: &Direction) -> f32 {
        match other {
            Direction::East => PI / 2.0 * 0.0,
            Direction::North => PI / 2.0 * 1.0,
            Direction::West => PI / 2.0 * 2.0,
            Direction::South => PI / 2.0 * 3.0,
        }
    }
}
