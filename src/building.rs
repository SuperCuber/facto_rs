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
            Building::Spawner {
                item,
                timer,
            } => {
                let mut timer = timer.borrow_mut();
                if *timer > item.spawning_time {
                    if let Some(target) = find_train_target(grid_items) {
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
                contents: _,
                timer,
            } => {
                let mut timer = timer.borrow_mut();
                if *timer > item.crafting_time {
                    *timer = 0.0;
                } else {
                    *timer += update.since_last.secs();
                }
            }
            Building::Submitter { .. } => todo!(),
        }
    }
}

fn find_train_target(_grid_items: &GridItems) -> Option<Position> {
    todo!()
}
