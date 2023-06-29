use std::collections::VecDeque;

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
            .get_mut(&self.target)
            .expect("train target does not exist")
            .contents()
            .expect("train target has no inventory");

        *contents.entry(self.item.clone()).or_default() += 1;
        false
    }
}
