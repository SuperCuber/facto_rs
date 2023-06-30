use nannou::{
    lyon::{
        geom::{Angle, Arc},
        lyon_tessellation::Orientation,
    },
    prelude::*,
};

use crate::{constants::*, model::*};

impl GridItem {
    pub fn draw_rail(&self, draw: &Draw) {
        match self {
            GridItem::Building(_, direction) => {
                draw_rail(&draw, *direction);
            }
            GridItem::Rail(orientation) => {
                let (dir1, dir2) = match orientation {
                    Orientation::Horizontal => (Direction::West, Direction::East),
                    Orientation::Vertical => (Direction::North, Direction::South),
                };
                draw_rail(draw, dir1);
                draw_rail(draw, dir2);
            }
            GridItem::Intersection(_, intersection_type) => {
                match *intersection_type {
                    IntersectionType::Corner(d) => {
                        draw_rail(draw, d);
                        draw_rail(draw, d.right());
                    }
                    IntersectionType::Triple(d) => {
                        draw_rail(draw, d);
                        draw_rail(draw, d.left());
                        draw_rail(draw, d.right());
                    }
                    IntersectionType::Quad => {
                        draw_rail(draw, Direction::East);
                        draw_rail(draw, Direction::West);
                        draw_rail(draw, Direction::North);
                        draw_rail(draw, Direction::South);
                    }
                };
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .no_fill()
            .w_h(CELL_SIZE, CELL_SIZE)
            .stroke_color(RED)
            .stroke_weight(1.0);

        match self {
            GridItem::Building(b, direction) => draw_building(draw, b, *direction),
            GridItem::Intersection(i, i_type) => draw_intersection(draw, &i.borrow(), i_type),
            GridItem::Rail(..) => {}
        }
    }
}

pub fn draw_building(draw: &Draw, b: &Building, direction: Direction) {
    let building_frame = {
        let offset = -(CELL_SIZE - BUILDING_SIZE) / 4.0;
        let center = Vec2::new(offset, 1.0).rotate(direction.into());
        Rect::from_xy_wh(center, (BUILDING_SIZE, BUILDING_SIZE).into())
    };

    match b {
        Building::Spawner { item, timer } => {
            let timer = timer.borrow();
            draw.ellipse()
                .color(soften(item.color))
                .xy(building_frame.xy())
                .wh(building_frame.wh());

            let arc = Arc {
                center: (building_frame.x(), building_frame.y()).into(),
                radii: (BUILDING_SIZE / 2.0, BUILDING_SIZE / 2.0).into(),
                start_angle: Angle::radians(0.0),
                sweep_angle: Angle::two_pi()
                    * animation_completion(*timer, item.spawning_time, 0.5)
                    // For some reason rendering is slightly off, this fixes
                    * 1.05,
                x_rotation: Angle::radians(0.0),
            };

            draw.path()
                .stroke()
                .stroke_weight(2.0 * SIZE_UNIT)
                .color(item.color)
                .points(arc.flattened(0.1).map(|p| Vec2::from((p.x, p.y))));
        }

        Building::Crafter {
            item,
            contents,
            timer,
        } => {
            let timer = timer.borrow();
            draw.rect()
                .xy(building_frame.xy())
                .wh(building_frame.wh())
                .color(soften(item.color));
            draw_loading_square_frame(
                &draw.xy(building_frame.xy()),
                item.color,
                animation_completion(*timer, item.crafting_time, 0.5),
                BUILDING_SIZE,
                2.0 * SIZE_UNIT,
            );

            let items_frame = building_frame.pad(5.0 * SIZE_UNIT);
            let mut position = (0, 0);
            let item_frame = Rect::from_wh(items_frame.wh() / INVENTORY_ITEM_SQUARE_SIDE as f32)
                .top_left_of(items_frame);
            for (item, &count) in contents.borrow().iter() {
                for _ in 0..count {
                    let position_px = Vec2::new(position.0 as f32, position.1 as f32);
                    let item_frame = item_frame.shift(item_frame.wh() * position_px);
                    draw.rect()
                        .xy(item_frame.xy())
                        .wh(item_frame.pad(2.0 * SIZE_UNIT).wh())
                        .stroke(item.color)
                        .stroke_weight(1.0 * SIZE_UNIT)
                        .color(soften(item.color));

                    position.0 += 1;
                    if position.0 >= INVENTORY_ITEM_SQUARE_SIDE {
                        position.1 -= 1;
                        position.0 = 0;
                    }
                }
            }
        }
        Building::Submitter { item, contents } => {
            let mut point = Vec2::X * BUILDING_SIZE / 3.0 * 2.0;
            let mut points = vec![];

            const SIDES: usize = 6;
            for _ in 0..SIDES {
                points.push(point);
                point = point.rotate(PI * 2.0 / (SIDES as f32));
            }

            draw.xy(building_frame.xy())
                .polygon()
                .points(points)
                .rotate(direction.into())
                .color(item.color);

            let items_frame = building_frame.pad(5.0 * SIZE_UNIT);
            let mut position = (0, 0);
            let item_frame = Rect::from_wh(items_frame.wh() / INVENTORY_ITEM_SQUARE_SIDE as f32)
                .top_left_of(items_frame);
            for (item, &count) in contents.borrow().iter() {
                for _ in 0..count {
                    let position_px = Vec2::new(position.0 as f32, position.1 as f32);
                    let item_frame = item_frame.shift(item_frame.wh() * position_px);
                    draw.rect()
                        .xy(item_frame.xy())
                        .wh(item_frame.pad(2.0 * SIZE_UNIT).wh())
                        .stroke(item.color)
                        .stroke_weight(1.0 * SIZE_UNIT)
                        .color(soften(item.color));

                    position.0 += 1;
                    if position.0 >= INVENTORY_ITEM_SQUARE_SIDE {
                        position.1 -= 1;
                        position.0 = 0;
                    }
                }
            }
        }
    }
}

fn draw_rail(draw: &Draw, direction: Direction) {
    let cell_frame = Rect::from_w_h(CELL_SIZE, CELL_SIZE);
    let draw_rotated = draw.rotate(direction.into());

    draw_rotated
        .y(BUILDING_SIZE / 6.0)
        .line()
        .weight(2.0 * SIZE_UNIT)
        .points(cell_frame.xy(), cell_frame.mid_right())
        .color(BLACK);
    draw_rotated
        .y(-BUILDING_SIZE / 6.0)
        .line()
        .weight(2.0 * SIZE_UNIT)
        .points(cell_frame.xy(), cell_frame.mid_right())
        .color(BLACK);
}

fn draw_intersection(
    draw: &Draw,
    _intersection: &Intersection,
    _intersection_type: &IntersectionType,
) {
    draw.rect()
        .w_h(BUILDING_SIZE / 2.0, BUILDING_SIZE / 2.0)
        .stroke_weight(SIZE_UNIT)
        .stroke_color(BLACK)
        .color(DARKGRAY);
}

impl Train {
    pub fn draw(&self, draw: &Draw) {
        let position = &self.path[self.position];
        let direction = self.calculate_direction();
        let draw_rotated = draw.xy((*position).into()).rotate(direction.into());
        draw_rotated
            .rect()
            .x(CELL_SIZE * (self.sub_position as f32) - (CELL_SIZE / 2.0))
            .y(-BUILDING_SIZE / 6.0)
            .color(self.item.color)
            .w_h(20.0 * SIZE_UNIT, 10.0 * SIZE_UNIT);
    }
}

// === Utils ===

fn soften(color: Srgb) -> Srgb {
    const C: f32 = 0.8;
    let mut color: Hsv = color.into();
    color.saturation *= C;
    color.into()
}

fn animation_completion(elapsed: f64, length: f64, end_lag: f64) -> f32 {
    let end = length - end_lag;
    debug_assert!(end > 0.0);

    if elapsed < end {
        (elapsed / end) as f32
    } else {
        1.0
    }
}

fn draw_loading_square_frame(draw: &Draw, color: Srgb, completion: f32, wh: f32, weight: f32) {
    let rect = Rect::from_w_h(wh, wh);
    let points = vec![
        (rect.mid_right(), 0.0),
        (rect.top_right(), 0.125),
        (rect.top_left(), 0.125 + 0.25),
        (rect.bottom_left(), 0.125 + 0.5),
        (rect.bottom_right(), 0.125 + 0.75),
        (rect.mid_right(), 1.0),
    ];
    for window in points.windows(2) {
        let (from, start) = window[0];
        let (to, end) = window[1];

        if completion < start {
            break;
        }

        let line_duration = end - start;
        let line_end = if end < completion { end } else { completion };
        let t = (line_end - start) / line_duration;
        let to = from * (1.0 - t) + to * t;
        draw.line()
            .points(from, to)
            .color(color)
            .stroke_weight(weight);
    }
}
