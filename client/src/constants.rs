pub const CUBE_SIZE: f32 = 1.0;
pub const GRAVITY: f32 = -9.8 * 4.0;

pub const TEXTURE_SIZE: u32 = 16;

pub const INTERACTION_DISTANCE: f32 = 7.;
pub const BASE_ROUGHNESS: f32 = 0.6;
pub const BASE_SPECULAR_HIGHLIGHT: f32 = 0.;

pub const DEFAULT_CHUNK_RENDER_DISTANCE_RADIUS: u32 = 1;

pub const CELESTIAL_SIZE: f32 = 10.;
pub const CELESTIAL_DISTANCE: f32 = 50.; // Low value for testing ; will be increased later
pub const DAY_DURATION: f32 = 60.; // Length of a full day/night cycle, in seconds

pub const MAX_INVENTORY_SLOTS: u32 = 4 * 9;
pub const MAX_HOTBAR_SLOTS: u32 = 9;

pub const HOTBAR_CELL_SIZE: f32 = 50.;
pub const HOTBAR_PADDING: f32 = 5.;
pub const HOTBAR_BORDER: f32 = 5.;

pub const SAVE_PATH: &str = "saves/";
pub const SERVER_LIST_SAVE_NAME: &str = "servers.ron";
pub const BINDS_PATH: &str = "keybindings.ron";

pub const GRASS_COLOR: [f32; 4] = [0.1, 1.0, 0.3, 1.0];
