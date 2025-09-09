use {
    image::{Rgb, RgbImage},
    voltz_terrain_generator_rust::{
        builder::{
            crystal::build_crystal,
            island::{SurfaceType, build_mainland},
            spire::{SpireType, build_spire},
            starter_island::build_starter_islands,
        },
        config::{
            BeaconConfig, CrystalConfig, GrassConfig, MainlandConfig, NoiseConfig, SpireConfig,
            StarterIslandConfig, SupportConfig, TerrainConfig,
        },
        generator::{
            beacon::generate_beacon,
            crystal::generate_crystal,
            grass::generate_grass,
            mainland::generate_mainland,
            spire::generate_spire,
            starter_island::generate_starter_islands,
            starter_island_point::starter_island_point,
            support::{SupportColumnType, generate_supports},
            support_point::{generate_mainland_support_points, generate_starter_support_points},
        },
        {sources::terrain_randomizers::TerrainRandomizers, vector3::Vector3},
    },
};

fn main() {
    let terrain_config = TerrainConfig {
        seed: rand::random(),
        cell_size: 1,
        support: SupportConfig {
            height: 10,
            support_spacing: 7,
            ladder_chance: 0.3,
            well_chance: 0.3,
            circular_chance: 0.3,
            min_radius: 0,
            max_radius: 1,
        },
        mainland: MainlandConfig {
            size: 60,
            gaps: NoiseConfig {
                weight: 0.19,
                scale: 16.0,
            },
            depth: NoiseConfig {
                weight: 3.0,
                scale: 32.0,
            },
        },
        beacon: BeaconConfig {
            offset: (45 / 2),
            beacon_count: 7,
            control_beacon: true,
        },
        starter_island: StarterIslandConfig {
            island_count: 6,
            island_offset: (45.0 / 2.0),
            island_size: 6,
            radius_variation: NoiseConfig {
                weight: 4.0,
                scale: 12.0,
            },
        },
        crystal: CrystalConfig { spawn_rate: 0.05 },
        spire: SpireConfig {
            cities: NoiseConfig {
                scale: 32.0,
                weight: 0.5,
            },
            buildings: NoiseConfig {
                scale: 12.0,
                weight: 0.5,
            },
            alleys: NoiseConfig {
                scale: 3.0,
                weight: 0.5,
            },
            height: NoiseConfig {
                scale: 8.0,
                weight: 4.0,
            },
        },
        grass: GrassConfig {
            grass: NoiseConfig {
                weight: 0.5,
                scale: 16.0,
            },
        },
    };

    let mut terrain_randomizers = TerrainRandomizers::from_seed(rand::random());

    // Generate island terrain
    let mainland_grid = generate_mainland(&terrain_config, &terrain_randomizers);

    // Generate spires
    let spire_grid = generate_spire(&terrain_config, &mut terrain_randomizers, &mainland_grid);

    // Generate crystal
    let crystal_grid = generate_crystal(
        &terrain_config,
        &mut terrain_randomizers,
        &mainland_grid,
        &spire_grid,
    );

    // Generate beacons
    let beacon_grid = generate_beacon(&terrain_config);

    // Generate grass
    let grass_grid = generate_grass(&terrain_config, &terrain_randomizers, &mainland_grid);

    // Starter island point
    let starter_island_point_grid = starter_island_point(&terrain_config);

    // Starter island grids
    let starter_island_grids = generate_starter_islands(
        &terrain_config,
        &terrain_randomizers,
        &starter_island_point_grid,
    );

    // Island support points
    let mainland_support_points = generate_mainland_support_points(&terrain_config, &mainland_grid);

    // Starter support points
    let starter_support_points =
        generate_starter_support_points(&terrain_config, &starter_island_grids);

    // Mainland supports
    let supports = generate_supports(
        &terrain_config,
        &mut terrain_randomizers,
        &mainland_support_points,
        &starter_support_points,
    );

    println!("Generating completed");

    //      Code for builders

    // Mainland builder
    let mainland_build = build_mainland(&mainland_grid, &grass_grid);

    // Crystal builder
    let crystal_build = build_crystal(&crystal_grid);

    // Spire builder
    let spire_build = build_spire(&spire_grid, &grass_grid);

    // Starter island builder
    let starter_island_build = build_starter_islands(&starter_island_grids);

    println!("Building completed");

    // Image dimentions
    let (width, height) = (200, 200);

    // For generators test
    let image = RgbImage::from_fn(width, height, |x, y| {
        // Get pixel
        let map_coordinate =
            Vector3::new(x as i32 - width as i32 / 2, 0, y as i32 - height as i32 / 2);

        // Render
        // for (&support_center, support) in &supports {
        //     for (&support_offset, support_column_type) in support {
        //         if support_center + support_offset == map_coordinate {
        //             return match support_column_type {
        //                 SupportColumnType::Fill => Rgb([200, 200, 200]),
        //                 SupportColumnType::Ladder => Rgb([150, 150, 150]),
        //                 SupportColumnType::Well => Rgb([100, 100, 100]),
        //             };
        //         }
        //     }
        // }

        // if beacon_grid.contains(&map_coordinate) {
        //     return Rgb([170, 175, 117]);
        // }

        // if crystal_grid.contains(&map_coordinate) {
        //     return Rgb([0, 150, 200]);
        // }

        // if let Some(height) = spire_grid.get(&map_coordinate) {
        //     let bias = *height as u8 * 10;
        //     return Rgb([100 + bias, 50 + bias, 0 + bias]);
        // }

        // if grass_grid.contains(&map_coordinate) {
        //     return Rgb([0, 200, 70]);
        // }

        if let Some(depth) = mainland_grid.get(&map_coordinate) {
            return match depth {
                0 => Rgb([65, 65, 65]),
                1 => Rgb([50, 50, 50]),
                2 => Rgb([42, 42, 42]),
                _ => Rgb([255, 0, 255]), // debug color
            };
        };

        for (&island_center, island) in &starter_island_grids {
            for (&island_offset, depth) in island {
                if island_center + island_offset == map_coordinate {
                    return match depth {
                        0 => Rgb([65, 65, 65]),
                        1 => Rgb([50, 50, 50]),
                        2 => Rgb([42, 42, 42]),
                        _ => Rgb([255, 0, 255]), // debug color
                    };
                }
            }
        }

        Rgb([255, 255, 255])
    });

    // Save test image
    image.save("test.png").unwrap();

    // Test for builders
    let depth_layers = terrain_config.mainland.depth.weight.round() as i32;
    let max_height = terrain_config.spire.height.weight.round() as i32;

    for map_depth in -(depth_layers - 1)..max_height {
        let builder_test_image = RgbImage::from_fn(width, height, |x, y| {
            // Get pixel
            let map_coordinates = Vector3::new(
                x as i32 - width as i32 / 2,
                map_depth,
                y as i32 - width as i32 / 2,
            );

            if crystal_build.contains(&map_coordinates) {
                return Rgb([0, 150, 200]);
            }

            if let Some(spire_type) = spire_build.get(&map_coordinates) {
                return match spire_type {
                    SpireType::Fill => Rgb([100, 50, 0]),
                    SpireType::Grass => Rgb([50, 200, 70]),
                    SpireType::Top => Rgb([200, 100, 0]),
                };
            }

            for (&island_center, island) in &starter_island_build {
                for &island_offset in island {
                    if island_center + island_offset == map_coordinates {
                        return Rgb([65, 65, 65]);
                    }
                }
            }

            if let Some(surface_type) = mainland_build.get(&map_coordinates) {
                return match surface_type {
                    SurfaceType::Grass => Rgb([0, 200, 70]),
                    SurfaceType::Fill => Rgb([65, 65, 65]),
                };
            }

            Rgb([255, 255, 255])
        });

        let image_path = format!("builder_{}.png", map_depth);

        builder_test_image.save(image_path).unwrap();
    }
}
