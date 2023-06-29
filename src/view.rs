use nannou::{
    lyon::{geom::{Angle, Arc}, lyon_tessellation::Orientation},
    prelude::*,
};

use crate::{constants::*, model::*};

impl GridItem {
    pub fn draw(&self, draw: &Draw) {

        // draw.rect()
        //     .no_fill()
        //     .w_h(CELL_SIZE, CELL_SIZE)
        //     .stroke_color(RED)
        //     .stroke_weight(1.0);

        match self {
            GridItem::Building(b, direction) => draw_building(draw, b, direction),
            GridItem::Rail(orientation) => draw_rail(draw, orientation, false),
            GridItem::Intersection(i, i_type) => draw_intersection(draw, i, i_type),
        }
    }
}

pub fn draw_building(draw: &Draw, b: &Building, direction: &Direction) {
    let draw_rotated = draw.rotate(direction.into());
    let offset = -(CELL_SIZE - BUILDING_SIZE) / 4.0;
    let center = Vec2::new(offset, 1.0).rotate(direction.into());

    let building_frame = Rect::from_w_h(BUILDING_SIZE, BUILDING_SIZE);
    let cell_frame = Rect::from_w_h(CELL_SIZE, CELL_SIZE);

    draw_rail(&draw_rotated, &Orientation::Horizontal, true);

    match b {
        Building::Spawner { item, timer } => {
            draw_rotated
                .ellipse()
                .x(offset)
                .color(soften(item.color))
                .wh(building_frame.wh());

            let arc = Arc {
                center: (center.x, center.y).into(),
                radii: (BUILDING_SIZE / 2.0, BUILDING_SIZE / 2.0).into(),
                start_angle: Angle::radians(0.0),
                sweep_angle: Angle::two_pi() * (timer / item.spawning_time) as f32,
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
            draw_rotated
                .x(offset)
                .rect()
                .color(soften(item.color))
                .wh(building_frame.wh());
            draw_loading_square_frame(
                &draw.xy(center),
                item.color,
                (timer / item.crafting_time) as f32,
                BUILDING_SIZE,
                2.0 * SIZE_UNIT,
            );
        }
        Building::Submitter { contents } => todo!(),
    }
}

fn draw_rail(draw: &Draw, orientation: &Orientation, half: bool) {
    let cell_frame = Rect::from_w_h(CELL_SIZE, CELL_SIZE);
    let draw_rotated = match orientation {
        Orientation::Horizontal => draw.clone(),
        Orientation::Vertical => draw.rotate(PI / 2.0),
    };

    let start = if half {
        cell_frame.xy()
    } else {
        cell_frame.mid_left()
    };

    draw_rotated
        .y(BUILDING_SIZE / 6.0)
        .line()
        .points(start, cell_frame.mid_right())
        .color(BLACK);
    draw_rotated
        .y(-BUILDING_SIZE / 6.0)
        .line()
        .points(start, cell_frame.mid_right())
        .color(BLACK);
}

fn draw_intersection(
    draw: &Draw,
    intersection: &Intersection,
    intersection_type: &IntersectionType,
) {
    let draw_rotated = draw.rotate(if let IntersectionType::Triple(dir) = intersection_type {
        dir.into()
    } else {
        0.0
    });
    let cell_frame = Rect::from_w_h(CELL_SIZE, CELL_SIZE);
    let intersection_frame = Rect::from_w_h(BUILDING_SIZE, BUILDING_SIZE);
    let is_triple = intersection_type.is_triple();

    draw_rail(&draw_rotated, &Orientation::Vertical, false);
    draw_rail(&draw_rotated, &Orientation::Horizontal, is_triple);

    if is_triple {
        let point1 = -Vec2::X;
        let point2 = point1.rotate(PI * 2.0 / 3.0);
        let point3 = point2.rotate(PI * 2.0 / 3.0);
        draw_rotated
            .x(BUILDING_SIZE / 18.0)
            .scale(CELL_SIZE / 3.0)
            .tri()
            .points(point1, point2, point3)
            .color(WHITE);
    } else {
        draw_rotated
            .rect()
            .w_h(CELL_SIZE / 2.0, CELL_SIZE / 2.0)
            .rotate(PI / 4.0)
            .color(WHITE);
    }
}

// === Utils ===

fn soften(color: Srgb) -> Srgb {
    const C: f32 = 0.8;
    let mut color: Hsv = color.into();
    color.saturation *= C;
    color.into()
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
