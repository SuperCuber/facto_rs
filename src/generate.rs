use std::collections::BTreeMap;

use nannou::{lyon::lyon_tessellation::Orientation, prelude::*};

use crate::{constants::*, model::*};

pub fn generate() -> (Grid, Vec<Item>) {
    let mut grid_items = BTreeMap::new();
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

    for x in 0..3 {
        for y in 0..3 {
            match (x + y) % 3 {
                0 => {
                    grid_items.insert(
                        Position(x, y),
                        GridItem::Building(
                            Building::Spawner {
                                item: item.clone(),
                                timer: 0.0,
                            },
                            Direction::South,
                        ),
                    );
                }
                1 => {
                    grid_items.insert(
                        Position(x, y),
                        GridItem::Building(
                            Building::Crafter {
                                item: item2.clone(),
                                contents: vec![],
                                timer: 0.0,
                            },
                            Direction::East,
                        ),
                    );
                }
                2 => {
                    grid_items.insert(
                        Position(x, y),
                        GridItem::Rail(
                            if x % 2 == 0 {
                                Orientation::Horizontal
                            } else {
                                Orientation::Vertical
                            },
                            RailSize::Small,
                        ),
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    let grid = Grid {
        grid_items,
        trains: vec![],
    };
    let items = vec![item, item2];

    (grid, items)
}
