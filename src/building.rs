use std::{cell::RefCell, collections::BTreeMap};

use nannou::prelude::*;

use crate::model::*;

impl Building {
    pub fn update(&mut self, update: &Update, grid_items: &BTreeMap<Position, RefCell<GridItem>>) {
        match self {
            Building::Spawner {
                item,
                ref mut timer,
            } => {
                if *timer > item.spawning_time {
                    *timer = 0.0;
                } else {
                    *timer += update.since_last.secs();
                }
            }
            Building::Crafter {
                item,
                contents: _,
                ref mut timer,
            } => {
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
