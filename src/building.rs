use std::{collections::VecDeque, ops::Deref};

use nannou::prelude::*;

use crate::{model::*, train::calculate_path};

impl Building {
    pub fn update(
        &self,
        position: &Position,
        update: &Update,
        grid_items: &GridItems,
        trains: &mut VecDeque<Train>,
    ) {
        match self {
            Building::Spawner { item, timer } => {
                let mut timer = timer.borrow_mut();
                if *timer > item.time && !Building::train_full(*position, trains) {
                    if let Some(target) = find_train_target(item, grid_items, trains) {
                        *timer = 0.0;
                        trains.push_back(Train {
                            item: item.clone(),
                            path: calculate_path(*position, target, grid_items),
                            position: 0,
                            sub_position: 0.5,
                        });
                    }
                } else {
                    *timer += update.since_last.secs();
                }
            }
            Building::Crafter {
                item,
                contents,
                timer,
            } => {
                let mut timer = timer.borrow_mut();
                if *timer > item.time && !Building::train_full(*position, trains) {
                    if let Some(target) = find_train_target(item, grid_items, trains) {
                        *timer = 0.0;
                        trains.push_back(Train {
                            item: item.clone(),
                            path: calculate_path(*position, target, grid_items),
                            position: 0,
                            sub_position: 0.5,
                        });
                    }
                } else if *timer == 0.0 && item.components.as_ref() == Some(contents.borrow().deref()) {
                    // Only start if we have contents, consuming them in the process
                    contents.borrow_mut().clear();
                    *timer += update.since_last.secs();
                } else if *timer > 0.0 {
                    *timer += update.since_last.secs();
                }
            }
            Building::Submitter { item, contents } => {
                if item.components.as_ref() == Some(contents.borrow().deref()) {
                    contents.borrow_mut().clear();
                    // TODO: point
                }
            }
        }
    }

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
                    .as_ref()
                    .and_then(|c| c.get(target_item))
                    .copied()
                    .unwrap_or_default();
                let existing_count = contents
                    .borrow()
                    .get(target_item)
                    .copied()
                    .unwrap_or_default();
                let incoming_trains = trains
                    .iter()
                    .filter(|t| t.path.last().unwrap() == self_position && &t.item == target_item)
                    .count();

                existing_count + incoming_trains < desired_count
            }
        }
    }

    fn train_full(position: Position, trains: &VecDeque<Train>) -> bool {
        trains.iter().any(|t| t.path[t.position] == position)
    }
}

// === Utils ===

fn find_train_target(
    item: &Item,
    grid_items: &GridItems,
    trains: &VecDeque<Train>,
) -> Option<Position> {
    for (pos, grid_item) in grid_items {
        if let GridItem::Building(b, _) = grid_item {
            if b.requires(item, pos, trains) {
                return Some(*pos);
            }
        }
    }
    None
}
