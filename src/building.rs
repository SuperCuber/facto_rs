use std::{collections::VecDeque, ops::Deref};

use nannou::prelude::*;

use crate::model::*;

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
                if *timer > item.spawning_time {
                    if let Some(target) = find_train_target(item, grid_items, trains) {
                        *timer = 0.0;
                        trains.push_back(Train {
                            item: item.clone(),
                            position: *position,
                            sub_position: Vec2::ZERO,
                            target,
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
                if *timer > item.crafting_time {
                    if let Some(target) = find_train_target(item, grid_items, trains) {
                        *timer = 0.0;
                        trains.push_back(Train {
                            item: item.clone(),
                            position: *position,
                            sub_position: Vec2::ZERO,
                            target,
                        });
                    }
                } else if *timer == 0.0 && &item.components == contents.borrow().deref() {
                    // Only start if we have contents, consuming them in the process
                    contents.borrow_mut().clear();
                    *timer += update.since_last.secs();
                } else if *timer > 0.0 {
                    *timer += update.since_last.secs();
                }
            }
            Building::Submitter { .. } => todo!(),
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
                    .get(target_item)
                    .copied()
                    .unwrap_or_default();
                let existing_count = contents
                    .borrow()
                    .get(target_item)
                    .copied()
                    .unwrap_or_default();
                let incoming_trains = trains.iter().filter(|t| t.target == *self_position).count();

                existing_count + incoming_trains < desired_count
            }
        }
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
