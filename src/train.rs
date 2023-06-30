use std::collections::{BTreeSet, VecDeque};

use nannou::prelude::*;

use crate::model::*;

impl Train {
    /// Returns true if train should be kept
    pub fn update(
        &mut self,
        _update: &Update,
        grid_items: &mut GridItems,
        _trains: &mut VecDeque<Train>,
    ) -> bool {
        let mut contents = grid_items
            .get_mut(self.path.last().unwrap())
            .expect("train target does not exist")
            .contents()
            .expect("train target has no inventory");

        *contents.entry(self.item.clone()).or_default() += 1;
        false
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
