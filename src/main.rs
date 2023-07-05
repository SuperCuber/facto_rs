use constants::CELL_SIZE;
use nannou::prelude::*;

mod building;
mod constants;
mod generate;
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
        score: 0,
    }
}

fn update(_app: &App, model: &mut Model, mut update: Update) {
    update.since_last *= 1;
    for _ in 0..model.grid.trains.len() {
        let mut train = model.grid.trains.pop_front().unwrap();
        if train.update(&update, &mut model.grid.grid_items, &mut model.grid.trains) {
            model.grid.trains.push_back(train);
        }
    }

    for (pos, grid_item) in &model.grid.grid_items {
        grid_item.update(
            pos,
            &update,
            &model.grid.grid_items,
            &mut model.grid.trains,
            &mut model.score,
        );
    }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        Closed => std::process::exit(0),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let (translation, scale) = center_grid_translation_scale(app.window_rect(), &model.grid);
    let draw_grid = draw.xy(translation).scale(scale);

    draw_grid.background().color(GREY);

    for (pos, grid_item) in &model.grid.grid_items {
        let pos = *pos;
        grid_item.draw_rail(&draw_grid.xy(pos.into()));
    }

    for train in &model.grid.trains {
        train.draw(&draw_grid);
    }

    for (pos, grid_item) in &model.grid.grid_items {
        let pos = *pos;
        grid_item.draw(&draw_grid.xy(pos.into()));
    }

    view::draw_recipes(&draw, frame.rect(), &model.items);
    view::draw_score(&draw, frame.rect(), model);

    draw_grid.to_frame(app, &frame).unwrap();
}

fn center_grid_translation_scale(rect: Rect, grid: &Grid) -> (Vec2, f32) {
    let min_x = grid.grid_items.keys().map(|p| p.0).min().unwrap();
    let min_y = grid.grid_items.keys().map(|p| p.1).min().unwrap();
    let max_x = grid.grid_items.keys().map(|p| p.0).max().unwrap();
    let max_y = grid.grid_items.keys().map(|p| p.1).max().unwrap();
    let grid_size = std::cmp::max(max_x, max_y) - std::cmp::min(min_x, min_y);
    let grid_size_px = grid_size as f32 * CELL_SIZE;
    let grid_offset_px = Vec2::new(-min_x as f32, -min_y as f32) * CELL_SIZE;
    let grid_rect = Rect::from_xy_wh(
        rect.xy() + grid_offset_px,
        (grid_size_px, grid_size_px).into(),
    );

    let translation = grid_rect.bottom_left(); //.round() + Vec2::new(0.5, 0.5) // Make the lines sharp
    let min_dimension = if rect.w() < rect.h() {
        rect.w()
    } else {
        rect.h()
    };
    let new_cell_size = min_dimension / (grid_size + 1) as f32;

    (translation, new_cell_size / CELL_SIZE)
}
