use std::{cell::RefCell, collections::{BTreeMap, VecDeque}};

use nannou::{lyon::lyon_tessellation::Orientation, prelude::*};

use crate::model::*;

pub fn generate() -> (Grid, Vec<Item>) {
    let item = Item {
        id: 0,
        color: Srgb::new(1.0, 0.0, 0.0),
        components: BTreeMap::new(),
        spawning_time: 7.0,
        crafting_time: 3.0,
    };
    let mut item2_components = BTreeMap::new();
    item2_components.insert(item.clone(), 1);
    let item2 = Item {
        id: 1,
        color: Srgb::new(0.0, 1.0, 0.0),
        components: item2_components,
        spawning_time: 7.0,
        crafting_time: 3.0,
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
                item: item.clone(),
                timer: RefCell::new(0.0),
            },
            Direction::North,
        ),
    );
    grid_items.insert(
        Position(3, 4),
        GridItem::Building(
            Building::Crafter {
                item: item2.clone(),
                contents: RefCell::new(BTreeMap::new()),
                timer: RefCell::new(0.0),
            },
            Direction::South,
        ),
    );

    // Connect
    grid_items.insert(Position(1, 1), GridItem::Rail(Orientation::Vertical));
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
            IntersectionType::Triple(Direction::North),
        ),
    );

    let grid = Grid {
        grid_items,
        trains: VecDeque::new(),
    };
    let items = vec![item, item2];

    (grid, items)
}
