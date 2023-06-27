use nannou::{
    lyon::{geom::Angle, lyon_tessellation::Orientation},
    prelude::*,
};

use crate::{constants::*, model::*};

impl GridItem {
    pub fn draw(&self, draw: &Draw) {
        match self {
            GridItem::Building(b, direction) => draw_building(draw, b, direction),
            GridItem::Rail(orientation, size) => draw_rail(draw, orientation, size),
            GridItem::Intersection(_, _) => todo!(),
        }
    }
}

pub fn draw_building(draw: &Draw, b: &Building, direction: &Direction) {
    let draw_rotated = draw.rotate(direction.into());
    const SIZE: f32 = CELL_SIZE / 3.0;
    let building_frame = Rect::from_w_h(SIZE, SIZE);
    let grid_frame = Rect::from_w_h(CELL_SIZE, CELL_SIZE);

    // Rails
    draw_rotated
        .y(building_frame.h() / 6.0)
        .line()
        .points(building_frame.mid_right() * 0.8, grid_frame.mid_right())
        .color(BLACK);
    draw_rotated
        .y(-building_frame.h() / 6.0)
        .line()
        .points(building_frame.mid_right() * 0.8, grid_frame.mid_right())
        .color(BLACK);

    match b {
        Building::Spawner { item, timer } => {
            draw.ellipse()
                .color(soften(item.color))
                .wh(building_frame.wh());

            let arc = nannou::lyon::geom::arc::Arc {
                center: (0.0, 0.0).into(),
                radii: (SIZE / 2.0, SIZE / 2.0).into(),
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
            draw.rect()
                .color(soften(item.color))
                .wh(building_frame.wh());
            draw_loading_square_frame(
                draw,
                item.color,
                (timer / item.crafting_time) as f32,
                SIZE,
                2.0 * SIZE_UNIT,
            );
        }
        Building::Submitter { contents } => todo!(),
    }
}

fn draw_rail(draw: &Draw, orientation: &Orientation, size: &RailSize) {}

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
