use std::collections::{BTreeSet, VecDeque};

use nannou::prelude::*;

use crate::{constants::TRAIN_LENGTH, model::*};

impl Train {
    /// Returns true if train should be kept
    pub fn update(
        &mut self,
        update: &Update,
        grid_items: &mut GridItems,
        trains: &mut VecDeque<Train>,
    ) -> bool {
        if self
            .next_requirements(grid_items)
            .iter()
            .any(|s| s.taken(trains))
        {
            dbg!(self.next_requirements(grid_items));
            // don't move
            return true;
        }

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
    }

    pub fn heading(&self) -> Direction {
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

    pub fn next_turn(&self) -> Option<Direction> {
        let position = self.path[self.position];
        let Some(next_position) = self.path.get(self.position + 1) else {return None};

        Some(position.direction_towards(*next_position).unwrap())
    }

    fn next_requirements(&self, grid_items: &GridItems) -> Vec<TrainSlot> {
        let current_slot = self.current_slot();
        let is_intersection = matches!(
            grid_items.get(&current_slot.position),
            Some(GridItem::Intersection(..))
        );
        let next_turn = self.next_turn();

        match current_slot.part {
            SlotPart::Input(..) if is_intersection => vec![
                TrainSlot {
                    position: current_slot.position,
                    part: SlotPart::Middle,
                },
                TrainSlot {
                    position: current_slot.position,
                    part: SlotPart::Output(next_turn.expect("intersection to lead somewhere")),
                },
            ],
            SlotPart::Input(..) if next_turn.is_some() => vec![TrainSlot {
                position: current_slot.position,
                part: SlotPart::Output(next_turn.unwrap()),
            }],
            SlotPart::Input(..) => vec![], // no next turn
            SlotPart::Middle => vec![],
            SlotPart::Output(..) => vec![TrainSlot {
                position: current_slot.position + next_turn.expect("output to lead somewhere"),
                part: SlotPart::Input(next_turn.expect("output to lead somewhere").opposite()),
            }],
        }
    }

    fn current_slot(&self) -> TrainSlot {
        let position = self.path[self.position];

        let part = if (0.0..TRAIN_LENGTH).contains(&self.sub_position) {
            SlotPart::Input(self.heading().opposite())
        } else if (TRAIN_LENGTH..(1.0 - TRAIN_LENGTH)).contains(&self.sub_position) {
            SlotPart::Middle
        } else if ((1.0 - TRAIN_LENGTH)..=1.0).contains(&self.sub_position) {
            SlotPart::Output(self.next_turn().unwrap())
        } else {
            unreachable!("sub_position not between 0 and 1")
        };

        TrainSlot { position, part }
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

    unreachable!("no valid path")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrainSlot {
    position: Position,
    part: SlotPart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotPart {
    /// The input part of a building/intersection
    Input(Direction),
    /// The middle part of an intersection
    Middle,
    /// The output part of a building/intersection
    Output(Direction),
}

impl TrainSlot {
    fn taken(&self, trains: &VecDeque<Train>) -> bool {
        dbg!(trains.iter().find(|t| t.current_slot() == *self)).is_some()
    }
}
