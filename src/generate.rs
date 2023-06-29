use std::collections::BTreeMap;

use nannou::{lyon::lyon_tessellation::Orientation, prelude::*};

use crate::{constants::*, model::*};

pub fn generate() -> (Grid, Vec<Item>) {
    let item = Item {
        color: Srgb::new(1.0, 0.0, 0.0),
        components: vec![],
        spawning_time: 7.0,
        crafting_time: 3.0,
    };
    let item2 = Item {
        color: Srgb::new(0.0, 1.0, 0.0),
        components: vec![],
        spawning_time: 7.0,
        crafting_time: 3.0,
    };

    let mut grid_items = BTreeMap::new();

    // Main rail
    for x in 0..5 {
        grid_items.insert(
            Position(x, 2),
            GridItem::Rail(Orientation::Horizontal),
        );
    }

    // Buildings
    grid_items.insert(
        Position(1, 0),
        GridItem::Building(
            Building::Spawner {
                item: item.clone(),
                timer: 0.0,
            },
            Direction::North,
        ),
    );
    grid_items.insert(
        Position(3, 4),
        GridItem::Building(
            Building::Crafter {
                item: item2.clone(),
                contents: vec![],
                timer: 0.0,
            },
            Direction::South,
        ),
    );

    // Connect
    grid_items.insert(
        Position(1, 1),
        GridItem::Rail(Orientation::Vertical),
    );
    grid_items.insert(
        Position(3, 3),
        GridItem::Rail(Orientation::Vertical),
    );

    // Intersection
    grid_items.insert(
        Position(1, 2),
        GridItem::Intersection(
            Intersection {
                item: None,
                cooldown: 0.0,
            },
            IntersectionType::Triple(Direction::South),
        ),
    );
    grid_items.insert(
        Position(3, 2),
        GridItem::Intersection(
            Intersection {
                item: None,
                cooldown: 0.0,
            },
            IntersectionType::Triple(Direction::North),
        ),
    );

    let grid = Grid {
        grid_items,
        trains: vec![],
    };
    let items = vec![item, item2];

    (grid, items)
}
