use crate::vector3::Vector3;
use std::collections::HashMap;

// TODO: Finish config

// Cell Types
struct CellType(String);

// Replacement Config
struct ReplacementMatrix(HashMap<Vector3<i32>, CellType>);

struct ReplacemenetEntry {
    matrix: ReplacementMatrix,
    extent: Vector3<i32>,
    weight: i32,
}

struct ReplacementConfig(HashMap<CellType, ReplacemenetEntry>);

// SelectionConfig
struct SelectionEntry {
    // model: Model, TODO: replace with mock type
    weight: i32,
}

struct SelectionEntries(Vec<SelectionEntry>);

struct SelectionConfig(HashMap<CellType, SelectionEntries>);

// Noise Config
pub struct NoiseConfig {
    pub weight: f64,
    pub scale: f64,
}

// Generator Config
pub struct SupportConfig {
    pub height: i32,
    pub support_spacing: i32,
    pub ladder_chance: f64,
    pub well_chance: f64,
    pub circular_chance: f64,
    pub min_radius: i32,
    pub max_radius: i32,
}

pub struct MainlandConfig {
    pub size: i32,
    pub gaps: NoiseConfig,
    pub depth: NoiseConfig,
}

pub struct BeaconConfig {
    pub offset: i32,
    pub beacon_count: i32,
    pub control_beacon: bool,
}

pub struct StarterIslandConfig {
    pub island_count: i32,
    pub island_offset: f64,
    pub island_size: i32,
    pub radius_variation: NoiseConfig,
}

pub struct CrystalConfig {
    pub spawn_rate: f64,
}

pub struct SpireConfig {
    pub cities: NoiseConfig,
    pub buildings: NoiseConfig,
    pub alleys: NoiseConfig,
    pub height: NoiseConfig,
}

pub struct GrassConfig {
    pub grass: NoiseConfig,
}

// TerrainConfig
pub struct TerrainConfig {
    // General
    pub seed: u64,
    pub cell_size: i32,

    // Template
    //replacement: ReplacementConfig,
    //selection: SelectionConfig,

    // generation
    pub support: SupportConfig,
    pub mainland: MainlandConfig,
    pub beacon: BeaconConfig,
    pub starter_island: StarterIslandConfig,
    pub crystal: CrystalConfig,
    pub spire: SpireConfig,
    pub grass: GrassConfig,
}
