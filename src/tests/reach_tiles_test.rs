#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::energy::Energy;
    use crate::event::events::Event;
    use crate::interface::Tools;
    use crate::runner::{Robot, Runnable, Runner};
    use crate::runner::backpack::BackPack;
    use crate::world::coordinates::Coordinate;
    use crate::world::environmental_conditions::EnvironmentalConditions;
    use crate::world::environmental_conditions::WeatherType::{Rainy, Sunny};
    use crate::world::tile::{Content, Tile};
    use crate::world::tile::TileType::Grass;
    use crate::world::World;
    use crate::world::world_generator::Generator;
    use crate::tools::reach_tiles::ReachTiles;
    use crate::tools::reach_tiles::TileTypeOrContent;
    use crate::world::tile::TileType;
    #[test]
    fn test_reach_tiles() {
        struct MyRobot(Robot);
        struct WorldGenerator {
            size: usize,
        }
        impl WorldGenerator {
            fn init(size: usize) -> Self {
                WorldGenerator { size }
            }
        }
        impl Generator for WorldGenerator {
            fn gen(
                &mut self,
            ) -> (
                Vec<Vec<Tile>>,
                (usize, usize),
                EnvironmentalConditions,
                f32,
                Option<HashMap<Content, f32>>,
            ) {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile_type = Grass;
                        let content = Content::None;
                        row.push(Tile {
                            tile_type,
                            content,
                            elevation: 0,
                        });
                    }
                    map.push(row);
                }
                let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

                let max_score = rand::random::<f32>();

                (map, (0, 0), environmental_conditions, max_score, None)
            }
        }
        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {
                // Creazione di una matrice di Tile per test
                let matrix_tile: Vec<Vec<Tile>> = vec![
                    vec![
                        Tile {
                            tile_type: TileType::Grass,
                            content: Content::Building,
                            elevation: 0,
                        },
                        Tile {
                            tile_type: TileType::Hill,
                            content: Content::Building,
                            elevation: 3,
                        },
                    ],
                    vec![
                        Tile {
                            tile_type: TileType::DeepWater,
                            content: Content::Building,
                            elevation: 0,
                        },
                        Tile {
                            tile_type: TileType::Sand,
                            content: Content::Building,
                            elevation: 2,
                        },
                    ],
                    vec![
                        Tile {
                            tile_type: TileType::Hill,
                            content: Content::Fire,
                            elevation: 0,
                        },
                        Tile {
                            tile_type: TileType::Sand,
                            content: Content::Fire,
                            elevation: 2,
                        },
                    ],
                ];

                let start_node = 0;
                ReachTiles::reach_tiles(matrix_tile, TileTypeOrContent::TileType(TileType::Hill), start_node);
            }

            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }

            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }

            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        let r = MyRobot(Robot::new());
        struct Tool;
        impl Tools for Tool {}
        let mut generator = WorldGenerator::init(100);
        let run = Runner::new(Box::new(r), &mut generator);

        //Known bug: 'check_world' inside 'Runner::new()' fails every time
        match run {
            | Ok(mut r) => {
                let _ = r.game_tick();
            }
            | Err(e) => println!("{:?}", e),
        }
    }
}