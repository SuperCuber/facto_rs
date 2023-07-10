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
                draw_rail(draw, *direction);
            }
            GridItem::Rail(orientation) => {
                let (dir1, dir2) = match orientation {
                    Orientation::Horizontal => (Direction::West, Direction::East),
                    Orientation::Vertical => (Direction::North, Direction::South),
                };
                draw_rail(draw, dir1);
                draw_rail(draw, dir2);
            }
            GridItem::Intersection(intersection_type) => {
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
        // draw.rect()
        //     .no_fill()
        //     .w_h(CELL_SIZE, CELL_SIZE)
        //     .stroke_color(RED)
        //     .stroke_weight(1.0);

        match self {
            GridItem::Building(b, direction) => draw_building(draw, b, *direction),
            GridItem::Intersection(i_type) => draw_intersection(draw, i_type),
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
        Building::Spawner {
            item,
            timer,
            spawn_timer,
        } => {
            let timer = timer.borrow();
            let spawn_timer = spawn_timer.borrow();

            let arc = Arc {
                center: (building_frame.x(), building_frame.y()).into(),
                radii: (BUILDING_SIZE / 2.0, BUILDING_SIZE / 2.0).into(),
                start_angle: Angle::radians(0.0),
                sweep_angle: lerp(
                    *timer as f32,
                    0.0,
                    item.time as f32,
                    Angle::zero(),
                    Angle::two_pi(),
                )
                .unwrap_or(Angle::two_pi())
                    * 1.03, // fix visually idk why
                x_rotation: Angle::radians(0.0),
            };

            // Expand
            let extra_size = lerp(
                *spawn_timer,
                0.0,
                ITEM_SPAWN_ANIMATION_TIME - ITEM_SPAWN_ANIMATION_TIME_SHRINK,
                0.0,
                LOADING_BAR_WEIGHT,
            )
            .unwrap_or(0.0);
            // Shrink
            let extra_size = extra_size
                + lerp(
                    *spawn_timer,
                    ITEM_SPAWN_ANIMATION_TIME - ITEM_SPAWN_ANIMATION_TIME_SHRINK,
                    ITEM_SPAWN_ANIMATION_TIME,
                    LOADING_BAR_WEIGHT,
                    0.0,
                )
                .unwrap_or(0.0);

            draw.path()
                .stroke()
                .stroke_weight(2.0 * (LOADING_BAR_WEIGHT as f32))
                .color(LOADING_BAR_COLOR)
                .points(arc.flattened(0.1).map(|p| Vec2::from((p.x, p.y))));

            draw.ellipse()
                .color(soften(item.color))
                .xy(building_frame.xy())
                .wh(building_frame.pad(-(extra_size as f32)).wh());
        }

        Building::Crafter {
            item,
            contents,
            timer,
            spawn_timer,
        } => {
            let timer = timer.borrow();
            let spawn_timer = spawn_timer.borrow();

            draw.rect()
                .xy(building_frame.xy())
                .wh(building_frame.wh())
                .color(soften(item.color));
            draw_loading_square_frame(
                &draw.xy(building_frame.xy()),
                item.color,
                animation_completion(*timer, item.time, ITEM_SPAWN_ANIMATION_TIME),
                BUILDING_SIZE,
                5.0 * SIZE_UNIT,
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

fn draw_intersection(draw: &Draw, _intersection_type: &IntersectionType) {
    let cell_frame = Rect::from_w_h(CELL_SIZE, CELL_SIZE);
    draw.rect()
        .wh(cell_frame.pad((SLOT_LENGTH as f32) * CELL_SIZE).wh())
        .stroke_weight(SIZE_UNIT)
        .stroke_color(BLACK)
        .color(DARKGRAY);
}

impl Train {
    pub fn draw(&self, draw: &Draw) {
        let position = &self.path[self.position];
        let direction = self.heading();
        let draw_rotated = draw.xy((*position).into()).rotate(direction.into());
        let cell_frame = Rect::from_w_h(CELL_SIZE, CELL_SIZE);

        draw_rotated
            .rect()
            .x(cell_frame.mid_left().x
                + CELL_SIZE * (self.sub_position - TRAIN_LENGTH / 2.0) as f32)
            .y(-BUILDING_SIZE / 6.0)
            .color(self.item.color)
            .w_h(
                CELL_SIZE * (TRAIN_LENGTH as f32),
                CELL_SIZE * (TRAIN_LENGTH as f32) / 2.0,
            );
    }
}

pub fn draw_recipes(draw: &Draw, window: Rect, items: &[Item]) {
    #[allow(clippy::iter_count)]
    let item_count = items
        .iter()
        // .filter(|i| !i.components.is_empty())
        .count();
    let max_components = items
        .iter()
        .map(|i| i.components.values().sum::<usize>())
        .max()
        .unwrap();
    let recipe_frame_contents_size =
        Vec2::new(((max_components * 2) + 1) as f32, item_count as f32)
            * Vec2::new(ITEM_RECIPE_SIZE, RECIPE_ROW_HEIGHT);

    for (line, item) in items
        .iter()
        // .filter(|i| !i.components.is_empty())
        .enumerate()
    {
        let row_frame = Rect::from_w_h(recipe_frame_contents_size.x, RECIPE_ROW_HEIGHT)
            .align_top_of(window)
            .align_right_of(window)
            .shift_y(-RECIPE_ROW_HEIGHT * line as f32)
            .pad(RECIPE_ROW_HEIGHT / 10.0);

        let result_frame = Rect::from_w_h(ITEM_RECIPE_SIZE, ITEM_RECIPE_SIZE)
            .align_middle_y_of(row_frame)
            .align_left_of(row_frame)
            .shift_x(-ITEM_RECIPE_SIZE / 3.0);

        draw.rect()
            .xy(result_frame.xy())
            .wh(result_frame.wh())
            .color(soften(item.color))
            .stroke(item.color);

        // Point
        if line + 1 == item_count {
            draw.text("+1")
                .xy(result_frame.xy())
                .wh(result_frame.wh())
                .align_text_middle_y()
                .font_size((ITEM_RECIPE_SIZE * 0.65) as u32)
                .color(BLACK);
        }

        let equals_frame = result_frame
            .shift_x(ITEM_RECIPE_SIZE)
            .pad(ITEM_RECIPE_SIZE / 6.0);
        draw.line()
            .y(-ITEM_RECIPE_SIZE / 6.0)
            .points(equals_frame.mid_left(), equals_frame.mid_right())
            .color(WHITE);
        draw.line()
            .y(ITEM_RECIPE_SIZE / 6.0)
            .points(equals_frame.mid_left(), equals_frame.mid_right())
            .color(WHITE);

        let mut component_frame = result_frame;
        let mut is_first = true;
        for (component, &count) in &item.components {
            for _ in 0..count {
                component_frame = component_frame.shift_x(ITEM_RECIPE_SIZE);
                if !is_first {
                    let plus_frame = component_frame.pad(ITEM_RECIPE_SIZE / 6.0);
                    draw.line()
                        .points(plus_frame.mid_left(), plus_frame.mid_right())
                        .color(WHITE);
                    draw.line()
                        .points(plus_frame.mid_top(), plus_frame.mid_bottom())
                        .color(WHITE);
                }
                is_first = false;

                component_frame = component_frame.shift_x(ITEM_RECIPE_SIZE);

                draw.rect()
                    .xy(component_frame.xy())
                    .wh(component_frame.wh())
                    .color(soften(component.color))
                    .stroke(component.color);
            }
        }

        // Spawned item
        if item.components.is_empty() {
            component_frame = component_frame.shift_x(ITEM_RECIPE_SIZE * 2.0);
            draw.ellipse()
                .xy(component_frame.xy())
                .wh(component_frame.wh())
                .no_fill()
                .stroke(WHITE)
                .stroke_weight(2.0 * SIZE_UNIT);
            draw.line()
                .points(
                    component_frame.xy(),
                    component_frame.xy()
                        + (component_frame.mid_right() - component_frame.xy()).rotate(1.0) * 0.75,
                )
                .color(WHITE)
                .stroke_weight(2.0 * SIZE_UNIT);
        }
    }
}

pub fn draw_score(draw: &Draw, screen: Rect, model: &Model) {
    let score_frame = Rect::from_w_h(200.0, 100.0).bottom_right_of(screen.pad(100.0));
    draw.text(&format!("{}", model.score))
        .xy(score_frame.xy())
        .wh(score_frame.wh())
        .font_size(72)
        .align_text_bottom()
        .right_justify();
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
        let line_end = end.min(completion);
        let t = (line_end - start) / line_duration;
        let to = from * (1.0 - t) + to * t;
        draw.line()
            .points(from, to)
            .color(color)
            .stroke_weight(weight);
    }
}

fn lerp<R, V>(val: R, range_start: R, range_end: R, val_start: V, val_end: V) -> Option<V>
where
    R: std::cmp::PartialOrd + std::ops::Sub<Output = R> + std::ops::Div<Output = R> + Copy,
    V: std::ops::Sub<Output = V> + std::ops::Mul<R, Output = V> + std::ops::Add<Output = V> + Copy,
{
    if val < range_start || range_end < val {
        return None;
    }

    let range_size = range_end - range_start;
    let completion = (val - range_start) / range_size;
    let val_diff = val_end - val_start;

    Some(val_start + (val_diff * completion))
}
