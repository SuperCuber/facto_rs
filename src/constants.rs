// Grid
pub const SIZE_UNIT: f32 = 1.0;
pub const CELL_SIZE: f32 = SIZE_UNIT * 100.0;
pub const BUILDING_SIZE: f32 = CELL_SIZE / 3.0 * 2.0;

pub const INVENTORY_ITEM_SQUARE_SIDE: usize = 3;

pub const SLOT_LENGTH: f64 = 0.3;
pub const TRAIN_BOUNDARY_1: f64 = SLOT_LENGTH;
pub const TRAIN_BOUNDARY_2: f64 = 1.0 - SLOT_LENGTH;

pub const TRAIN_LENGTH: f64 = 0.2;

// Generation
pub const MIN_ITEMS: usize = 6;
pub const MAX_ITEMS: usize = 10;
pub const MAX_SPAWNABLE_ITEMS: usize = 3;
pub const MAX_COMPONENTS: usize = 5;
pub const MIN_ITEM_TIME: f64 = 1.0;
pub const MAX_ITEM_TIME: f64 = 5.0;
pub const CELLS_PER_BUILDING: f32 = 20.0;
