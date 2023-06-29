use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    ops::DerefMut,
};

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
    pub grid_items: GridItems,
    pub trains: VecDeque<Train>,
}

pub type GridItems = BTreeMap<Position, GridItem>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, Clone)]
pub enum GridItem {
    Building(Building, Direction),
    Rail(Orientation),
    Intersection(RefCell<Intersection>, IntersectionType),
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
    pub spawning_time: f64,
    pub crafting_time: f64,
}

#[derive(Clone, Debug)]
pub struct Train {
    pub item: Item,
    pub position: Position,
    pub sub_position: Vec2,
    pub target: Position,
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
            GridItem::Intersection(i, _) => i.borrow_mut().update(update),
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

impl IntersectionType {
    pub fn is_triple(&self) -> bool {
        matches!(self, IntersectionType::Triple(..))
    }
}

impl Building {
    pub fn requires(
        &self,
        target_item: &Item,
        self_position: &Position,
        trains: &VecDeque<Train>,
    ) -> bool {
        match self {
            Building::Spawner { .. } => false,
            Building::Crafter { item, contents, .. } | Building::Submitter { item, contents } => {
                let desired_count = item
                    .components
                    .iter()
                    .filter(|x| x.0 == target_item)
                    .count();
                let existing_count = contents
                    .borrow()
                    .iter()
                    .filter(|x| x.0 == target_item)
                    .count();
                let incoming_trains = trains.iter().filter(|t| t.target == *self_position).count();

                existing_count + incoming_trains < desired_count
            }
        }
    }
}
