use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
};

use nannou::{color::Hue, lyon::lyon_tessellation::Orientation, prelude::*};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{constants::*, model::*};

pub fn generate() -> (Grid, Vec<Item>) {
    let items = generate_recipes(&mut StdRng::seed_from_u64(2));
    dbg!(&items);
    let buildings = generate_buildings(&items);
    dbg!(&buildings);

    let red = Item {
        id: 1,
        color: Srgb::new(1.0, 0.0, 0.0),
        components: BTreeMap::new(),
        time: 2.0,
    };
    let green = Item {
        id: 2,
        color: Srgb::new(0.0, 1.0, 0.0),
        components: BTreeMap::new(),
        time: 2.0,
    };
    let mut yellow_components = BTreeMap::new();
    yellow_components.insert(red.clone(), 1);
    yellow_components.insert(green.clone(), 1);
    let yellow = Item {
        id: 3,
        color: Srgb::new(1.0, 1.0, 0.0),
        components: yellow_components,
        time: 2.5,
    };

    let mut point_components = BTreeMap::new();
    point_components.insert(yellow.clone(), 1);
    let point = Item {
        id: 0,
        color: Srgb::new(0.0, 0.0, 0.0),
        components: point_components,
        time: 0.0,
    };

    let mut grid_items = GridItems::new();

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
        Position(1, 4),
        GridItem::Building(
            Building::Spawner {
                item: green,
                timer: RefCell::new(0.0),
            },
            Direction::South,
        ),
    );
    grid_items.insert(
        Position(3, 4),
        GridItem::Building(
            Building::Crafter {
                item: yellow.clone(),
                contents: RefCell::new(BTreeMap::new()),
                timer: RefCell::new(0.0),
            },
            Direction::South,
        ),
    );
    grid_items.insert(
        Position(3, 0),
        GridItem::Building(
            Building::Submitter {
                item: point,
                contents: RefCell::new(BTreeMap::new()),
            },
            Direction::North,
        ),
    );

    // Connect
    grid_items.insert(Position(1, 1), GridItem::Rail(Orientation::Vertical));
    grid_items.insert(Position(1, 3), GridItem::Rail(Orientation::Vertical));
    grid_items.insert(Position(2, 2), GridItem::Rail(Orientation::Horizontal));
    grid_items.insert(Position(3, 3), GridItem::Rail(Orientation::Vertical));
    grid_items.insert(Position(3, 1), GridItem::Rail(Orientation::Vertical));

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
            IntersectionType::Quad,
        ),
    );

    let grid = Grid {
        grid_items,
        trains: VecDeque::new(),
    };
    let items = vec![red, yellow];

    (grid, items)
}

fn generate_recipes(rng: &mut StdRng) -> Vec<Item> {
    let item_count = rng.gen_range(MIN_ITEMS..=MAX_ITEMS);
    // First x items are spawnable (no components), components always smaller idx than parent
    // Last item is "point"
    let starting_hue: f32 = rng.gen();
    let starting_color = Hsv::new(starting_hue, 1.0, 1.0);
    let mut items: Vec<Item> = (0..item_count)
        .map(|i| Item {
            id: i,
            color: starting_color
                .shift_hue((1.0 / item_count as f32) * (i as f32))
                .into(),
            components: BTreeMap::new(),
            time: rng.gen_range(MIN_ITEM_TIME..=MAX_ITEM_TIME),
        })
        .collect();

    for item_idx in MAX_SPAWNABLE_ITEMS..item_count {
        let component_count = rng.gen_range(1..=MAX_COMPONENTS);
        let (before, after) = items.split_at_mut(item_idx);
        let item = after.first_mut().unwrap();
        for _ in 0..component_count {
            let component_idx = rng.gen_range(0..before.len());
            *item
                .components
                .entry(before[component_idx].clone())
                .or_default() += 1;
        }
    }

    let point = items.last().unwrap();
    let needed_for_point: Vec<usize> = recursive_needed_for(point)
        .into_iter()
        .map(|i| i.id)
        .collect();
    let point = point.id;
    items.retain(|i| i.id == point || needed_for_point.contains(&i.id));

    items
}

fn recursive_needed_for(item: &Item) -> Vec<&Item> {
    item.components
        .keys()
        .flat_map(|k| Some(k).into_iter().chain(recursive_needed_for(k)))
        .collect()
}

fn generate_buildings(items: &[Item]) -> Vec<Building> {
    todo!()
}
