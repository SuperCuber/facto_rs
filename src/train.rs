use std::collections::{BTreeSet, VecDeque};

use nannou::prelude::*;

use crate::model::*;

impl Train {
    /// Returns true if train should be kept
    pub fn update(
        &mut self,
        update: &Update,
        grid_items: &mut GridItems,
        _trains: &mut VecDeque<Train>,
    ) -> bool {
        // Move and then submit in the same tick so that we never have to draw an invalid state
        self.sub_position += update.since_last.secs();
        if self.sub_position >= 1.0 {
            self.sub_position = 0.0;
            self.position += 1;
        }

        if self.position + 1 == self.path.len() && self.sub_position >= 0.5 {
            let mut contents = grid_items
                .get_mut(self.path.last().unwrap())
                .expect("train target does not exist")
                .contents()
                .expect("train target has no inventory");

            *contents.entry(self.item.clone()).or_default() += 1;
            false
        } else {
            true
        }

        // match self.position {
        //     0 => {}
        //     x if x == self.path.len() => {}
        //     n => {}
        // }
    }

    pub fn calculate_direction(&self) -> Direction {
        let position = self.path[self.position];
        if self.sub_position < 0.5 {
            let previous_position = self
                .position
                .checked_sub(1)
                .and_then(|num| self.path.get(num))
                .expect("previous position exists in first half of grid");
            previous_position.direction_towards(position).unwrap()
        } else {
            let next_position = self
                .path
                .get(self.position + 1)
                .expect("next position exists in second half of grid");
            position.direction_towards(*next_position).unwrap()
        }
    }
}

pub fn calculate_path(start: Position, target: Position, grid_items: &GridItems) -> Vec<Position> {
    let mut queue = VecDeque::new();
    let mut explored = BTreeSet::new();

    queue.push_back(vec![start]);
    explored.insert(start);

    while let Some(path) = queue.pop_front() {
        let last = path.last().unwrap();
        if last == &target {
            return path;
        }

        if let Some(grid_item) = grid_items.get(last) {
            for neighbor in grid_item.neighbors(*last) {
                if !explored.contains(&neighbor) {
                    explored.insert(neighbor);
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push_back(new_path);
                }
            }
        }
    }

    todo!()
}
