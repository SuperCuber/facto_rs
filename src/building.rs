use std::ops::Deref;

use nannou::prelude::*;

use crate::model::*;

impl Building {
    pub fn update(
        &self,
        position: &Position,
        update: &Update,
        grid_items: &GridItems,
        trains: &mut Vec<Train>,
    ) {
        match self {
            Building::Spawner { item, timer } => {
                let mut timer = timer.borrow_mut();
                if *timer > item.spawning_time {
                    if let Some(target) = find_train_target(item, grid_items, trains) {
                        *timer = 0.0;
                        trains.push(Train {
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
                        trains.push(Train {
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
}

// === Utils ===

fn find_train_target(item: &Item, grid_items: &GridItems, trains: &[Train]) -> Option<Position> {
    for (pos, grid_item) in grid_items {
        if let GridItem::Building(b, _) = grid_item {
            if b.requires(item, pos, trains) {
                return Some(*pos);
            }
        }
    }
    None
}
