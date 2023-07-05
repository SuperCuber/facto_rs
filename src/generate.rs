use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
};

use nannou::{color::Hue, lyon::lyon_tessellation::Orientation, prelude::*};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{constants::*, model::*};

pub fn generate() -> (Grid, Vec<Item>) {
    let mut rng = StdRng::seed_from_u64(1);
    let items = generate_recipes(&mut rng);
    // dbg!(&items);
    let grid_items = generate_grid_items(&items, &mut rng);

    let grid = Grid {
        grid_items,
        trains: VecDeque::new(),
    };
    (grid, items)
}

fn generate_recipes(rng: &mut StdRng) -> Vec<Item> {
    let item_count = rng.gen_range(MIN_ITEMS..=MAX_ITEMS);
    // First x items are spawnable (no components), components always smaller idx than parent
    // Last item is "point"
    let starting_hue: f32 = rng.gen();
    let starting_color = Hsv::new(starting_hue * 360.0, 1.0, 1.0);
    let mut items: Vec<Item> = (0..item_count)
        .map(|i| Item {
            id: i,
            color: starting_color
                .shift_hue((360.0 / item_count as f32) * (i as f32))
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

fn generate_grid_items(items: &[Item], rng: &mut StdRng) -> GridItems {
    let mut grid_items = GridItems::new();

    let buildings = buildings_for(items.last().unwrap());
    let grid_size = buildings.len() as isize / 2;

    for x in (-grid_size)..grid_size {
        grid_items.insert(Position(x, 0), GridItem::Rail(Orientation::Horizontal));
    }
    for y in (-grid_size)..grid_size {
        grid_items.insert(Position(0, y), GridItem::Rail(Orientation::Vertical));
    }
    grid_items.insert(
        Position(0, 0),
        GridItem::Intersection(IntersectionType::Quad),
    );

    for b in buildings.into_iter() {
        loop {
            let direction: Direction = rng.gen();
            let connection_distance = rng.gen_range(2..grid_size);
            let connection_position = direction.to_position() * connection_distance;
            let offset_direction = if rng.gen_bool(0.5) {
                direction.left()
            } else {
                direction.right()
            };

            let connection_entry = grid_items
                .get_mut(&connection_position)
                .expect("intersection or rail");
            if matches!(connection_entry, GridItem::Intersection(..)) {
                // taken, try again
                continue;
            }

            *connection_entry = GridItem::Intersection(IntersectionType::Triple(offset_direction));
            let mut building_position = connection_position;
            let offset = rng.gen_range(1..connection_distance);
            for _ in 0..offset {
                building_position = building_position + offset_direction;
                grid_items.insert(
                    building_position,
                    GridItem::Rail(offset_direction.to_orientation()),
                );
            }
            grid_items.insert(
                building_position,
                GridItem::Building(b, offset_direction.opposite()),
            );
            break;
        }
    }

    grid_items
}

fn buildings_for(root_item: &Item) -> Vec<Building> {
    let counts: BTreeMap<_, _> = building_counts_for(root_item)
        .into_iter()
        .map(|(k, v)| (k, v.ceil() as usize))
        .collect();
    counts
        .into_iter()
        .flat_map(|(item, count)| {
            (0..count).map(move |_| {
                if item.components.is_empty() {
                    Building::Spawner {
                        item: item.clone(),
                        timer: RefCell::new(0.0),
                    }
                } else if item == root_item {
                    // points' buildings are actually submitters
                    Building::Submitter {
                        item: item.clone(),
                        contents: RefCell::new(BTreeMap::new()),
                    }
                } else {
                    Building::Crafter {
                        item: item.clone(),
                        contents: RefCell::new(BTreeMap::new()),
                        timer: RefCell::new(0.0),
                    }
                }
            })
        })
        .collect()
}

fn building_counts_for(item: &Item) -> BTreeMap<&Item, f64> {
    let mut buildings = BTreeMap::new();
    buildings.insert(item, item.time);

    for (component, count) in &item.components {
        let component_buildings = building_counts_for(component);
        for (subcomponent, subcomponent_count) in &component_buildings {
            *buildings.entry(subcomponent).or_default() += subcomponent_count * (*count as f64);
        }
    }

    buildings
}
