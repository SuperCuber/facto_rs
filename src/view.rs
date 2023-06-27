use nannou::{lyon::geom::Angle, prelude::*};

use crate::{constants::*, model::*};

impl GridItem {
    pub fn draw(&self, pos: Position, draw: &Draw) {
        match self {
            GridItem::Building(b) => b.draw(pos, draw),
            GridItem::Rail(_, _) => todo!(),
            GridItem::Intersection(_, _) => todo!(),
        }
    }
}

impl Building {
    pub fn draw(&self, pos: Position, draw: &Draw) {
        match self {
            Building::Spawner { item, timer } => {
                const RADIUS: f32 = CELL_SIZE / 6.0;

                draw.ellipse().color(soften(item.color)).radius(RADIUS);

                let arc = nannou::lyon::geom::arc::Arc {
                    center: (0.0, 0.0).into(),
                    radii: (RADIUS, RADIUS).into(),
                    start_angle: Angle::radians(0.0),
                    sweep_angle: Angle::two_pi() * (timer / SPAWNER_TIMER_SEC) as f32,
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
                const SIZE: f32 = CELL_SIZE / 3.0;
                draw.rect().color(soften(item.color)).w(SIZE).h(SIZE);
                draw_loading_square_frame(draw, item.color, (timer / SPAWNER_TIMER_SEC) as f32, SIZE, 2.0 * SIZE_UNIT);
            }
            Building::Submitter { contents } => todo!(),
        }
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
        draw.line().points(from, to).color(color).stroke_weight(weight);
    }
}
