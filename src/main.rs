#![allow(dead_code)]

use constants::CELL_SIZE;
use nannou::prelude::*;

mod building;
mod constants;
mod generate;
mod intersection;
mod model;
mod train;
mod view;

use model::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .maximized(true)
        .view(view)
        .event(event)
        .build()
        .unwrap();
    let (grid, items) = generate::generate();
    Model {
        window,
        grid,
        items,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    for _ in 0..model.grid.trains.len() {
        let mut train = model.grid.trains.pop_front().unwrap();
        if train.update(&update, &mut model.grid.grid_items, &mut model.grid.trains) {
            model.grid.trains.push_back(train);
        }
    }

    for (pos, grid_item) in &model.grid.grid_items {
        grid_item.update(pos, &update, &model.grid.grid_items, &mut model.grid.trains);
    }
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let translation = center_grid_translation(app.window_rect(), &model.grid);
    let draw = draw.xy(translation);

    draw.background().color(GREY);

    for train in &model.grid.trains {
        train.draw(&draw);
    }

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
    // Make the lines sharp
    grid_rect.bottom_left().round() + Vec2::new(0.5, 0.5)
}
