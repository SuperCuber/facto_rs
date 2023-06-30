use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
};

use nannou::{lyon::lyon_tessellation::Orientation, prelude::*};

use crate::model::*;

pub fn generate() -> (Grid, Vec<Item>) {
    let red = Item {
        id: 1,
        color: Srgb::new(1.0, 0.0, 0.0),
        components: BTreeMap::new(),
        spawning_time: 3.0,
        crafting_time: 7.0,
    };
    let mut green_components = BTreeMap::new();
    green_components.insert(red.clone(), 4);
    let green = Item {
        id: 2,
        color: Srgb::new(0.0, 1.0, 0.0),
        components: green_components,
        spawning_time: 3.0,
        crafting_time: 7.0,
    };

    let mut point_components = BTreeMap::new();
    point_components.insert(red.clone(), 1);
    point_components.insert(green.clone(), 1);
    let point = Item {
        id: 0,
        color: Srgb::new(0.0, 0.0, 1.0),
        components: point_components,
        spawning_time: 3.0,
        crafting_time: 7.0,
    };

    let mut grid_items = GridItems::new();

    // Main rail
    for x in 0..5 {
        grid_items.insert(Position(x, 2), GridItem::Rail(Orientation::Horizontal));
    }

    // Buildings
    grid_items.insert(
        Position(1, 0),
        GridItem::Building(
            Building::Spawner {
                item: red.clone(),
                timer: RefCell::new(0.0),
            },
            Direction::North,
        ),
    );
    grid_items.insert(
        Position(3, 4),
        GridItem::Building(
            Building::Crafter {
                item: green.clone(),
                contents: RefCell::new(BTreeMap::new()),
                timer: RefCell::new(0.0),
            },
            Direction::South,
        ),
    );
    grid_items.insert(
        Position(1, 4),
        GridItem::Building(
            Building::Submitter {
                item: point,
                contents: RefCell::new(BTreeMap::new()),
            },
            Direction::South,
        ),
    );

    // Connect
    grid_items.insert(Position(1, 1), GridItem::Rail(Orientation::Vertical));
    grid_items.insert(Position(1, 3), GridItem::Rail(Orientation::Vertical));
    grid_items.insert(Position(3, 3), GridItem::Rail(Orientation::Vertical));

    // Intersection
    grid_items.insert(
        Position(1, 2),
        GridItem::Intersection(
            RefCell::new(Intersection {
                item: None,
                cooldown: 0.0,
            }),
            IntersectionType::Quad,
        ),
    );
    grid_items.insert(
        Position(3, 2),
        GridItem::Intersection(
            RefCell::new(Intersection {
                item: None,
                cooldown: 0.0,
            }),
            IntersectionType::Corner(Direction::West),
        ),
    );

    let grid = Grid {
        grid_items,
        trains: VecDeque::new(),
    };
    let items = vec![red, green];

    (grid, items)
}
