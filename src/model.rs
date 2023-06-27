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
    Building(Building),
    Rail(Orientation, RailSize),
    Intersection(Intersection, IntersectionType),
}

#[derive(Debug, Clone)]
pub enum RailSize {
    Big,
    Small,
}

#[derive(Debug, Clone)]
pub struct Intersection {
    item: Option<Item>,
}

#[derive(Debug, Clone)]
pub enum IntersectionType {
    // Triple
    LeftUpRight,
    UpRightDown,
    RightDownLeft,
    DownLeftUp,

    // Quad
    FourDirection,
}

#[derive(Debug, Clone)]
pub enum Building {
    Spawner { item: Item, timer: f64 },
    Crafter { item: Item, contents: Vec<Item>, timer: f64 },
    Submitter { contents: Vec<Item> },
}

// === Item ===
#[derive(Clone, Debug)]
pub struct Item {
    pub color: Srgb,
    pub components: Vec<Item>,
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
            GridItem::Building(b) => b.update(update),
            GridItem::Rail(_, _) => todo!(),
            GridItem::Intersection(_, _) => todo!(),
        }
    }
}

impl From<Position> for Vec2 {
    fn from(other: Position) -> Vec2 {
        Vec2::from((other.0 as f32 * CELL_SIZE, other.1 as f32 * CELL_SIZE))
    }
}
