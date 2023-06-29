#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::BTreeMap;

use constants::CELL_SIZE;
use nannou::prelude::*;

mod building;
mod constants;
mod generate;
mod intersection;
mod model;
mod view;

use model::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app.new_window().maximized(true).view(view).event(event).build().unwrap();
    let (grid, items) = generate::generate();
    Model {
        window,
        grid,
        items,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    for (pos, grid_item) in &mut model.grid.grid_items {
        grid_item.update(&update);
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let translation = center_grid_translation(app.window_rect(), &model.grid);
    let draw = draw.xy(translation);

    draw.background().color(GREY);

    for (pos, grid_item) in &model.grid.grid_items {
        let pos = *pos;
        grid_item.draw(&draw.xy(pos.into()));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn center_grid_translation(rect: Rect, grid: &Grid) -> Vec2 {
    let max_x = grid.grid_items.keys().map(|p| p.0).max().unwrap();
    let max_y = grid.grid_items.keys().map(|p| p.1).max().unwrap();
    let grid_size = std::cmp::max(max_x, max_y) as f32 * CELL_SIZE;

    let grid_rect = Rect::from_xy_wh(rect.xy(), (grid_size, grid_size).into());
    grid_rect.bottom_left()
}
