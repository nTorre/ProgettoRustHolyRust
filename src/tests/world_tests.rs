mod coordinate_test {
    use crate::world::coordinates::Coordinate;

    #[test]
    fn test_coordinate_new() {
        let coordinate = Coordinate::new(1, 2);

        assert_eq!(coordinate.get_row(), 1);
        assert_eq!(coordinate.get_col(), 2);
    }
}

#[cfg(test)]
mod environmental_conditions_tests {
    use crate::world::environmental_conditions::{DayTime, EnvironmentalConditions, TimeOfDay, WeatherType};

    #[test]
    fn test_environmental_conditions_new() {
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
            WeatherType::TrentinoSnow,
        ];
        let environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 240, 12).unwrap();

        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "12:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);
    }

    #[test]
    fn found_error_with_time_progression() {
        let weather_forecast = vec![WeatherType::Sunny];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 120, 12).unwrap();

        environmental_conditions.tick();
        assert_eq!(environmental_conditions.get_time_of_day_string(), "14:00".to_owned());

        environmental_conditions.tick();
        assert_eq!(environmental_conditions.get_time_of_day_string(), "16:00".to_owned());
    }

    #[test]
    fn found_error_with_time_progression_2() {
        let weather_forecast = vec![WeatherType::Sunny];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 120, 10).unwrap();

        environmental_conditions.tick();
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
    }

    #[test]
    fn test_environmental_conditions_tick() {
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
            WeatherType::TrentinoSnow,
        ];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 60, 12).unwrap();

        (0..2).into_iter().for_each(|_| {
            environmental_conditions.tick();
        });
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "14:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);

        (0..2).into_iter().for_each(|_| {
            environmental_conditions.tick();
        });
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "16:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);

        (0..2).into_iter().for_each(|_| {
            environmental_conditions.tick();
        });
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "18:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);

        (0..6).into_iter().for_each(|_| {
            environmental_conditions.tick();
        });
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Night);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "00:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Rainy);
    }

    #[test]
    fn test_environmental_conditions_next_day() {
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
        ];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 30, 12).unwrap();
        environmental_conditions.next_day();
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Rainy);
    }

    #[test]
    fn test_advance() {
        let mut time = TimeOfDay { hour: 0, minute: 0 };

        assert_eq!(time.advance(120), false);
        assert_eq!(time.hour, 2);
        assert_eq!(time.minute, 0);
    }
}

#[cfg(test)]
mod world_struct_tests {
    use crate::world::{
        environmental_conditions::{EnvironmentalConditions, WeatherType},
        tile::{Tile, TileType},
        World,
    };

    #[test]
    fn test_world_new_and_advance_time() {
        let map = vec![vec![Tile {
            tile_type: TileType::Grass,
            content: crate::world::tile::Content::None,
            elevation: 0,
        }]];
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
            WeatherType::TrentinoSnow,
        ];
        let environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 60, 12).unwrap();
        let mut world = World {
            map: map.clone(),
            dimension: 1,
            discoverable: 1 / 10 + 1,
            environmental_conditions,
            score_counter: Default::default(),
        };
        // let mut world = World::new(map.clone(), environmental_conditions, 1.);

        assert_eq!(world.dimension, map.len());
        (0..12).into_iter().for_each(|_| {
            world.advance_time();
        });

        assert_eq!(
            world.environmental_conditions.get_time_of_day_string(),
            "00:00".to_owned()
        );
        assert_eq!(
            world.environmental_conditions.get_weather_condition(),
            WeatherType::Rainy
        );

        (0..25).into_iter().for_each(|_| {
            world.advance_time();
        });

        assert_eq!(
            world.environmental_conditions.get_time_of_day_string(),
            "01:00".to_owned()
        );
        assert_eq!(
            world.environmental_conditions.get_weather_condition(),
            WeatherType::Foggy
        );
    }
}

#[cfg(test)]
mod world_generator_tests {
    use crate::world::tile::Content::Water;
    use crate::world::{
        tile::{Content, Tile, TileType},
        world_generator::{check_world, get_content_percentage, get_tiletype_percentage},
    };

    #[test]
    fn test_check_world_valid() {
        let world = vec![
            vec![
                Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                },
                Tile {
                    tile_type: TileType::DeepWater,
                    content: Water(1),
                    elevation: 0,
                },
                Tile {
                    tile_type: TileType::ShallowWater,
                    content: Water(1),
                    elevation: 0,
                },
            ];
            3
        ];

        let result = check_world(&world);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_check_world_invalid() {
        let world = vec![vec![Tile {
            tile_type: TileType::Grass,
            content: Content::Rock(3),
            elevation: 0,
        }]];

        let result = check_world(&world);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_get_tiletype_percentage() {
        let world = vec![vec![
            Tile {
                tile_type: TileType::Grass,
                content: Content::None,
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Rock(5),
                elevation: 0,
            },
            Tile {
                tile_type: TileType::ShallowWater,
                content: Content::Rock(5),
                elevation: 0,
            },
        ]];

        let result = get_tiletype_percentage(&world);
        assert_eq!(result.get(&TileType::Grass), Some(&(2_f64 / 3_f64)));
        assert_eq!(result.get(&TileType::ShallowWater), Some(&(1_f64 / 3_f64)));
        assert_eq!(result.get(&TileType::DeepWater), None);
    }

    #[test]
    fn test_get_content_percentage() {
        let world = vec![vec![
            Tile {
                tile_type: TileType::Grass,
                content: Content::Rock(3),
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Bin(0..2),
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Tree(1),
                elevation: 0,
            },
        ]];
        println!("{:?}", world);
        let result = get_content_percentage(&world);
        println!("{:?}", result);
        assert_eq!(result.get(&Content::Bin(0..2).to_default()), Some(&(1_f64 / 3_f64))); //example: using to default function
        assert_eq!(result.get(&Content::Rock(0)), Some(&(1_f64 / 3_f64)));
        assert_eq!(result.get(&Content::Tree(0)), Some(&(1_f64 / 3_f64)));
    }
}

#[cfg(test)]
mod score_tests {
    use std::cell::RefCell;

    use crate::world::score::*;
    use crate::world::tile::*;

    // use crate::energy::Energy;
    // use crate::interface::Direction::*;
    // use crate::interface::{debug, destroy, go, put, Direction};
    // use crate::runner::backpack::BackPack;
    // use crate::runner::{run, Robot, Runnable};
    // use crate::utils::add_to_backpack;
    // use crate::world::coordinates::Coordinate;
    // use crate::world::environmental_conditions::EnvironmentalConditions;
    // use crate::world::environmental_conditions::WeatherType::{Rainy, Sunny};
    // use crate::world::tile::Content::*;
    // use crate::world::tile::TileType::*;
    // use crate::world::worldgenerator::Generator;
    // use crate::world::World;
    // use rand::Rng;
    // use std::char::MAX;
    // use strum::IntoEnumIterator;

    const MAX_SCORE: f32 = 20.;

    #[test]
    fn test_add_score_flat() {
        let score_counter = ScoreCounter::new(
            MAX_SCORE,
            &vec![vec![Tile {
                tile_type: TileType::Grass,
                content: Content::Rock(3),
                elevation: 0,
            }]],
            None,
        );
        let result = score_counter.add_score_flat(10.);
        println!("{:?}", result);
        assert_eq!(score_counter.get_score(), 10.);
    }

    #[test]
    fn test_get_score() {
        let mut score_counter = ScoreCounter::new(
            MAX_SCORE,
            &vec![vec![Tile {
                tile_type: TileType::Grass,
                content: Content::Rock(3),
                elevation: 0,
            }]],
            None,
        );
        score_counter.score = RefCell::new(10.).into();
        assert_eq!(score_counter.get_score(), 10.);
    }

    #[test]
    fn test_init() {
        let world = vec![vec![
            Tile {
                tile_type: TileType::Grass,
                content: Content::Rock(3),
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Bin(0..2),
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Tree(1),
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Garbage(1),
                elevation: 0,
            },
        ]];
        let score_table = ScoreCounter::init_score_table(&world, MAX_SCORE, None);
        assert_eq!(
            score_table.get(&Content::Rock(0).to_default()),
            Some(1. * MAX_SCORE / ((3 * 1 + 1 * 3 + 1 * 2 + 2 * 10) as f32)).as_ref()
        );
    }
}
// Commented out as it is for debug purpose!
//Implementing Generator for add_score_destroy and add_score_put tests
//     struct WorldGenerator {
//         size: usize,
//     }
//     impl WorldGenerator {
//         fn init(size: usize) -> Self {
//             WorldGenerator { size }
//         }
//     }
//     impl Generator for WorldGenerator {
//         fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32) {
//             let map = vec![
//                 vec![
//                     Tile {
//                         tile_type: TileType::Grass,
//                         content: Content::Rock(3),
//                         elevation: 0,
//                     },
//                     Tile {
//                         tile_type: TileType::DeepWater,
//                         content: Content::Bin(0..2),
//                         elevation: 0,
//                     },
//                 ],
//                 vec![
//                     Tile {
//                         tile_type: TileType::Sand,
//                         content: Content::Garbage(1),
//                         elevation: 0,
//                     },
//                     Tile {
//                         tile_type: TileType::Grass,
//                         content: Content::Tree(2),
//                         elevation: 0,
//                     },
//                 ],
//             ];
//             let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12);
//             (map, (0, 0), environmental_conditions, MAX_SCORE)
//         }
//     }
//
// #[test]
// fn test_add_score_destroy() {
//     struct RobotThatDestroys(Robot);
//     impl Runnable for RobotThatDestroys {
//         fn process_tick(&mut self, world: &mut World) {
//             // other

//             for _ in 0..1 {
//                 let (tmp, a, b) = debug(self, world);
//                 for e in tmp.iter() {
//                     for f in e.iter() {
//                         match f.tile_type {
//                             | DeepWater => {
//                                 print!("DW");
//                             }
//                             | ShallowWater => {
//                                 print!("SW");
//                             }
//                             | Sand => {
//                                 print!("Sa");
//                             }
//                             | Grass => {
//                                 print!("Gr");
//                             }
//                             | Street => {
//                                 print!("St");
//                             }
//                             | Hill => {
//                                 print!("Hi");
//                             }
//                             | Mountain => {
//                                 print!("Mt");
//                             }
//                             | Snow => {
//                                 print!("Sn");
//                             }
//                             | Lava => {
//                                 print!("La");
//                             }
//                         }
//                         match &f.content {
//                             | Rock(quantity) => print!("->Ro {}", quantity),
//                             | Tree(quantity) => print!("->Tr {}", quantity),
//                             | Garbage(quantity) => print!("->Gr {}", quantity),
//                             | Fire => print!("->Fi -"),
//                             | Coin(quantity) => print!("->Co {}", quantity),
//                             | Bin(range) => print!("->Bi {}-{}", range.start, range.end),
//                             | Crate(range) => print!("->Cr {}-{}", range.start, range.end),
//                             | Bank(range) => print!("->Ba {}-{}", range.start, range.end),
//                             | Water(quantity) => print!("->Wa {}", quantity),
//                             | Content::None => print!("->No -"),
//                             | Content::Fish(quantity) => print!("->Fh {}", quantity),
//                             | Content::Market(quantity) => print!("->Mk {}", quantity),
//                             | Content::Building => print!("->Bui -"),
//                             | Content::Bush(quantity) => print!("->Bu {}", quantity),
//                             | Content::JollyBlock(quantity) => print!("->Jo {}", quantity),
//                             | Content::Scarecrow => print!("->Sc -"),

//                         }
//                         print!("\t| ");
//                     }
//                     println!();
//                 }
//                 println!("{:?}, {:?}", a, b);
//                 // match ris {
//                 //     | Ok(values) => println!("Ok"),
//                 //     | Err(e) => println!("{:?}", e),
//                 // }
//             }
//             println!("Destroying {:?}", destroy(self, world, Down));
//             println!("{:?}", world.score_counter);
//         }

//         fn get_energy_mut(&mut self) -> &mut Energy {
//             &mut self.0.energy
//         }
//         fn get_energy(&self) -> &Energy {
//             &self.0.energy
//         }
//          fn handle_event(&mut self, event: Event) {
//              println!("{:?}", event)
//          }
//         fn get_backpack(&self) -> &BackPack {
//             &self.0.backpack
//         }
//         fn get_backpack_mut(&mut self) -> &mut BackPack {
//             &mut self.0.backpack
//         }
//         fn get_coordinate(&self) -> &Coordinate {
//             &self.0.coordinate
//         }
//         fn get_coordinate_mut(&mut self) -> &mut Coordinate {
//             &mut self.0.coordinate
//         }
//     }
//     let mut r = RobotThatDestroys(Robot::new());
//     let mut generator = WorldGenerator::init(10);
//     println!("{:?}", run(&mut r, &mut generator));
// }

//     #[test]
//     fn test_add_score_put() {
//         struct RobotThatPuts(Robot);
//         impl Runnable for RobotThatPuts {
//             fn process_tick(&mut self, world: &mut World) {
//                 // other

//                 for _ in 0..1 {
//                     let (tmp, a, b) = debug(self, world);
//                     for e in tmp.iter() {
//                         for f in e.iter() {
//                             match f.tile_type {
//                                 | DeepWater => {
//                                     print!("DW");
//                                 }
//                                 | ShallowWater => {
//                                     print!("SW");
//                                 }
//                                 | Sand => {
//                                     print!("Sa");
//                                 }
//                                 | Grass => {
//                                     print!("Gr");
//                                 }
//                                 | Street => {
//                                     print!("St");
//                                 }
//                                 | Hill => {
//                                     print!("Hi");
//                                 }
//                                 | Mountain => {
//                                     print!("Mt");
//                                 }
//                                 | Snow => {
//                                     print!("Sn");
//                                 }
//                                 | Lava => {
//                                     print!("La");
//                                 }
//                             }
//                             match &f.content {
//                                 | Rock(quantity) => print!("->Ro {}", quantity),
//                                 | Tree(quantity) => print!("->Tr {}", quantity),
//                                 | Garbage(quantity) => print!("->Gr {}", quantity),
//                                 | Fire => print!("->Fi -"),
//                                 | Coin(quantity) => print!("->Co {}", quantity),
//                                 | Bin(range) => print!("->Bi {}-{}", range.start, range.end),
//                                 | Crate(range) => print!("->Cr {}-{}", range.start, range.end),
//                                 | Bank(range) => print!("->Ba {}-{}", range.start, range.end),
//                                 | Water(quantity) => print!("->Wa {}", quantity),
//                                 | Content::None => print!("->No -"),
//                                 | Content::Fish(quantity) => print!("->Fh {}", quantity),
//                                 | Content::Market(quantity) => print!("->Mk {}", quantity),
//                                 | Content::Building => print!("->Bui -"),
//                                 | Content::Bush(quantity) => print!("->Bu {}", quantity),
//                                 | Content::JollyBlock(quantity) => print!("->Jo {}", quantity),
//                                 | Content::Scarecrow => print!("->Sc -"),
//                             }
//                             print!("\t| ");
//                         }
//                         println!();
//                     }
//                     println!("{:?}, {:?}", a, b);
//                     // match ris {
//                     //     | Ok(values) => println!("Ok"),
//                     //     | Err(e) => println!("{:?}", e),
//                     // }
//                 }
//                 let _ = add_to_backpack(self, Content::Garbage(0), 2);
//                 println!(
//                     "Putting {:?}",
//                     put(self, world, Content::Garbage(0), 2, Direction::Right)
//                 );
//                 println!("{:?}", world.score_counter);
//             }

//             fn get_energy_mut(&mut self) -> &mut Energy {
//                 &mut self.0.energy
//             }
//             fn get_energy(&self) -> &Energy {
//                 &self.0.energy
//             }

//             fn get_backpack(&self) -> &BackPack {
//                 &self.0.backpack
//             }
//             fn get_backpack_mut(&mut self) -> &mut BackPack {
//                 &mut self.0.backpack
//             }
//             fn get_coordinate(&self) -> &Coordinate {
//                 &self.0.coordinate
//             }
//             fn get_coordinate_mut(&mut self) -> &mut Coordinate {
//                 &mut self.0.coordinate
//             }
//         }
//         let mut r = RobotThatPuts(Robot::new());
//         let mut generator = WorldGenerator::init(10);
//         println!("{:?}", run(&mut r, &mut generator));
//     }
// }
