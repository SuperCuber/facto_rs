use nannou::prelude::*;

use crate::{constants::*, model::*};

impl Building {
    pub fn update(&mut self, update: &Update) {
        match self {
            Building::Spawner {
                item,
                ref mut timer,
            } => {
                if *timer > SPAWNER_TIMER_SEC + 0.5 {
                    *timer = 0.0;
                } else {
                    *timer += update.since_last.secs();
                }
            }
            Building::Crafter {
                item,
                contents,
                ref mut timer,
            } => {
                if *timer > SPAWNER_TIMER_SEC + 0.5 {
                    *timer = 0.0;
                } else {
                    *timer += update.since_last.secs();
                }
            }
            Building::Submitter { .. } => todo!(),
        }
    }
}
