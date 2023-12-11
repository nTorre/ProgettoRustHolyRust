use std::collections::HashMap;

use rand::Rng;
use strum::IntoEnumIterator;

use crate::energy::Energy;
use crate::energy::*;
use crate::event::events::Event;
use crate::interface::Direction::{Down, Right};
use crate::interface::{debug, destroy, go, put, Tools};
use crate::interface::{one_direction_view, Direction};
use crate::runner::backpack::BackPack;
use crate::runner::{Robot, Runnable, Runner};
use crate::utils::LibError::*;
use crate::utils::*;
use crate::world::coordinates::Coordinate;
use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use crate::world::tile::Content::{Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree, Water};
use crate::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport, Wall,
};
use crate::world::tile::{Content, Tile, TileType};
use crate::world::world_generator::Generator;
use crate::world::World;

mod backpack_test;
mod energy_tests;
mod interface_tests;
mod runner_test;
mod utils_test;
mod world_tests;

// Structs and function implementation shared in more than one test
struct TestRobot(Robot);

struct TestWorld {
    size: usize,
}

impl TestWorld {
    fn init(size: usize) -> Self {
        TestWorld { size }
    }
}

impl Generator for TestWorld {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        let _rng = rand::thread_rng();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // Initialize the map with default tiles
        for _ in 0..self.size {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..self.size {
                row.push(Tile {
                    tile_type: Sand,
                    content: Rock(1),
                    elevation: 0,
                });
            }
            map.push(row);
        }
        let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12);
        (map, (0, 0), environmental_conditions.unwrap(), 1.0, None)
    }
}

impl Runnable for TestRobot {
    fn process_tick(&mut self, _world: &mut crate::world::World) {
        println!("I am just the TestRobot, I won't change the world 🥲.")
    }

    fn handle_event(&mut self, event: Event) {
        println!("{:?}", event);
    }

    fn get_energy(&self) -> &crate::energy::Energy {
        &self.0.energy
    }

    fn get_energy_mut(&mut self) -> &mut crate::energy::Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &crate::world::coordinates::Coordinate {
        &self.0.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut crate::world::coordinates::Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &crate::runner::backpack::BackPack {
        &self.0.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut crate::runner::backpack::BackPack {
        &mut self.0.backpack
    }
}

fn generate_map_of_type_and_content(tile_type: TileType, content: Content, size: usize) -> Vec<Vec<Tile>> {
    vec![
        vec![
            Tile {
                tile_type,
                content,
                elevation: 0,
            };
            size
        ];
        size
    ]
}

fn generate_sunny_weather() -> EnvironmentalConditions {
    EnvironmentalConditions::new(&[Sunny], 15, 12).unwrap()
}

// fn gen_world(dimension: usize) -> World {
//     let mut rng = rand::thread_rng();
//     let mut map: Vec<Vec<Tile>> = Vec::new();
//     // Initialize the map with default tiles
//     for _ in 0..dimension {
//         let mut row: Vec<Tile> = Vec::new();
//         for _ in 0..dimension {
//             let i_tiletype = rng.gen_range(0..TileType::iter().len());
//             let i_content = rng.gen_range(0..Content::iter().len());
//             let tile_type = match i_tiletype {
//                 | 0 => DeepWater,
//                 | 1 => ShallowWater,
//                 | 2 => Sand,
//                 | 3 => Grass,
//                 | 4 => Street,
//                 | 5 => Hill,
//                 | 6 => Mountain,
//                 | 7 => Snow,
//                 | 8 => Lava,
//                 | _ => Grass,
//             };
//             let content = match i_content {
//                 | 0 => Rock(0),
//                 | 1 => Tree(2),
//                 | 2 => Garbage(2),
//                 | 3 => Fire,
//                 | 4 => Coin(2),
//                 | 5 => Bin(0..2),
//                 | 6 => Crate(1..2),
//                 | 7 => Bank(3..54),
//                 | 8 => Water(20),
//                 | 9 => Content::None,
//                 | 10 => Fish(3),
//                 | 11 => Market(20),
//                 | 12 => Content::Building,
//                 | 13 => Content::Bush(2),
//                 | 14 => Content::JollyBlock(20),
//                 | 15 => Content::Scarecrow,
//                 | _ => Content::None,
//             };
//             row.push(Tile {
//                 tile_type,
//                 content,
//                 elevation: 0,
//             });
//         }
//         map.push(row);
//     }
//     let environmental_conditions = EnvironmentalConditions::new(&vec![Sunny, Rainy], 15, 12);
//     let tools_allowed = vec![];
//     World {
//         map,
//         dimension,
//         environmental_conditions,
//         tools_allowed,
//         score_counter: Default::default(),
//     }
// }

// I commented this because of ISSUE #91
// pub(crate) fn view_interface_test(robot: &impl Runnable, world: &World) {
//     let tmp = robot_view(robot, world);

//     for row in tmp.iter() {
//         for elem in row.iter() {
//             print!("{:?}", elem)
//         }
//         println!();
//     }
// }

#[test]
#[ignore]
pub(crate) fn testing() {
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
            let mut rng = rand::thread_rng();
            let mut map: Vec<Vec<Tile>> = Vec::new();
            // Initialize the map with default tiles
            for _ in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for _ in 0..self.size {
                    let i_tiletype = rng.gen_range(0..TileType::iter().len());
                    let i_content = rng.gen_range(0..Content::iter().len());
                    let elevation = rng.gen_range(0..5);
                    let tile_type = match i_tiletype {
                        | 0 => DeepWater,
                        | 1 => ShallowWater,
                        | 2 => Sand,
                        | 3 => Grass,
                        | 4 => Street,
                        | 5 => Hill,
                        | 6 => Mountain,
                        | 7 => Snow,
                        | 8 => Lava,
                        | _ => Grass,
                    };
                    let content = match i_content {
                        | 0 => Rock(0),
                        | 1 => Tree(2),
                        | 2 => Garbage(2),
                        | 3 => Fire,
                        | 4 => Coin(2),
                        | 5 => Bin(2..3),
                        | 6 => Crate(2..3),
                        | 7 => Bank(3..54),
                        | 8 => Water(20),
                        | 9 => Content::None,
                        | 10 => Fish(3),
                        | 11 => Market(20),
                        | 12 => Content::Building,
                        | 13 => Content::Bush(2),
                        | 14 => Content::JollyBlock(20),
                        | 15 => Content::Scarecrow,
                        | _ => Content::None,
                    };

                    row.push(Tile {
                        tile_type,
                        content,
                        elevation,
                    });
                }
                map.push(row);
            }
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();
            (map, (0, 0), environmental_conditions, 1., None)
        }
    }
    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            // other

            for _ in 0..1 {
                let (tmp, a, b) = debug(self, world);
                for e in tmp.iter() {
                    for f in e.iter() {
                        match f.tile_type {
                            | DeepWater => {
                                print!("DW");
                            }
                            | ShallowWater => {
                                print!("SW");
                            }
                            | Sand => {
                                print!("Sa");
                            }
                            | Grass => {
                                print!("Gr");
                            }
                            | Street => {
                                print!("St");
                            }
                            | Hill => {
                                print!("Hi");
                            }
                            | Mountain => {
                                print!("Mt");
                            }
                            | Snow => {
                                print!("Sn");
                            }
                            | Lava => {
                                print!("La");
                            }
                            | Teleport(_) => {
                                print!("Tl");
                            }
                            | Wall => {
                                print!("Wa");
                            }
                        }
                        match &f.content {
                            | Rock(quantity) => print!("->Ro {}", quantity),
                            | Tree(quantity) => print!("->Tr {}", quantity),
                            | Garbage(quantity) => print!("->Gr {}", quantity),
                            | Fire => print!("->Fi -"),
                            | Coin(quantity) => print!("->Co {}", quantity),
                            | Bin(range) => print!("->Bi {}", range.start),
                            | Crate(range) => print!("->Cr {}", range.start),
                            | Bank(range) => print!("->Ba {}", range.start),
                            | Water(quantity) => print!("->Wa {}", quantity),
                            | Content::None => print!("->No -"),
                            | Content::Fish(quantity) => print!("->Fh {}", quantity),
                            | Content::Market(quantity) => print!("->Mk {}", quantity),
                            | Content::Building => print!("->Bui -"),
                            | Content::Bush(quantity) => print!("->Bu {}", quantity),
                            | Content::JollyBlock(quantity) => print!("->Jo {}", quantity),
                            | Content::Scarecrow => print!("->Sc -"),
                        }
                        print!("\t| ");
                    }
                    println!();
                }
                println!("{:?}, {:?}", a, b);
                // match ris {
                //     | Ok(values) => println!("Ok"),
                //     | Err(e) => println!("{:?}", e),
                // }
            }
            println!("HERE {:?}", destroy(self, world, crate::interface::Direction::Down))
        }

        fn handle_event(&mut self, event: Event) {
            println!("{:?}", event);
        }

        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }

        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
    }

    let r = MyRobot(Robot::new());
    let mut generator = WorldGenerator::init(10);

    struct Tool;
    impl Tools for Tool {}

    let mut run = Runner::new(Box::new(r), &mut generator).unwrap();
    match run.game_tick() {
        | Ok(_) => println!("Seccesful game tick"),
        | Err(e) => println!("{:?}", e),
    }
}

#[test]
pub fn test_issue24() {
    struct MyRobot(Robot);
    struct WorldGenerator;
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
            map.push(vec![
                Tile {
                    tile_type: Grass,
                    content: Content::None,
                    elevation: 0,
                },
                Tile {
                    tile_type: Sand,
                    content: Content::None,
                    elevation: 0,
                },
                Tile {
                    tile_type: DeepWater,
                    content: Water(1),
                    elevation: 0,
                },
            ]);
            map.push(vec![
                Tile {
                    tile_type: ShallowWater,
                    content: Water(1),
                    elevation: 0,
                },
                Tile {
                    tile_type: Grass,
                    content: Tree(2),
                    elevation: 0,
                },
                Tile {
                    tile_type: Sand,
                    content: Content::None,
                    elevation: 0,
                },
            ]);
            map.push(vec![
                Tile {
                    tile_type: ShallowWater,
                    content: Water(1),
                    elevation: 0,
                },
                Tile {
                    tile_type: Grass,
                    content: Content::None,
                    elevation: 0,
                },
                Tile {
                    tile_type: Street,
                    content: Content::None,
                    elevation: 0,
                },
            ]);
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();
            (map, (1, 1), environmental_conditions, 1., None)
        }
    }
    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            // other
            for _ in 0..1 {
                let (tmp, a, b) = debug(self, world);
                for elem in tmp.iter() {
                    for tile in elem.iter() {
                        match tile.tile_type {
                            | DeepWater => {
                                print!("DW");
                            }
                            | ShallowWater => {
                                print!("SW");
                            }
                            | Sand => {
                                print!("Sa");
                            }
                            | Grass => {
                                print!("Gr");
                            }
                            | Street => {
                                print!("St");
                            }
                            | Hill => {
                                print!("Hi");
                            }
                            | Mountain => {
                                print!("Mt");
                            }
                            | Snow => {
                                print!("Sn");
                            }
                            | Lava => {
                                print!("La");
                            }
                            | Teleport(_) => {
                                print!("Tl");
                            }
                            | Wall => {
                                print!("Wa");
                            }
                        }
                        match &tile.content {
                            | Rock(quantity) => print!("->Ro {}", quantity),
                            | Tree(quantity) => print!("->Tr {}", quantity),
                            | Garbage(quantity) => print!("->Gr {}", quantity),
                            | Fire => print!("->Fi -"),
                            | Coin(quantity) => print!("->Co {}", quantity),
                            | Bin(range) => print!("->Bi {}-{}", range.start, range.end),
                            | Crate(range) => print!("->Cr {}-{}", range.start, range.end),
                            | Bank(range) => print!("->Ba {}-{}", range.start, range.end),
                            | Water(quantity) => print!("->Wa {}", quantity),
                            | Content::None => print!("->No -"),
                            | Content::Fish(quantity) => print!("->Fh {}", quantity),
                            | Content::Market(quantity) => print!("->Mk {}", quantity),
                            | Content::Building => print!("->Bui -"),
                            | Content::Bush(quantity) => print!("->Bu {}", quantity),
                            | Content::JollyBlock(quantity) => print!("->Jo {}", quantity),
                            | Content::Scarecrow => print!("->Sc -"),
                        }
                        print!("\t| ");
                    }
                    println!();
                }
                println!("{:?}, {:?}", a, b);
            }

            println!("HERE {:?}", destroy(self, world, Right));
            let _ = put(self, world, Tree(0), 2, Down);
            let (tmp, _a, _b) = debug(self, world);
            for elem in tmp.iter() {
                for tile in elem.iter() {
                    match tile.tile_type {
                        | DeepWater => {
                            print!("DW");
                        }
                        | ShallowWater => {
                            print!("SW");
                        }
                        | Sand => {
                            print!("Sa");
                        }
                        | Grass => {
                            print!("Gr");
                        }
                        | Street => {
                            print!("St");
                        }
                        | Hill => {
                            print!("Hi");
                        }
                        | Mountain => {
                            print!("Mt");
                        }
                        | Snow => {
                            print!("Sn");
                        }
                        | Lava => {
                            print!("La");
                        }
                        | Teleport(_) => {
                            print!("Tl");
                        }
                        | Wall => {
                            print!("Wa");
                        }
                    }
                    match &tile.content {
                        | Rock(quantity) => print!("->Ro {}", quantity),
                        | Tree(quantity) => print!("->Tr {}", quantity),
                        | Garbage(quantity) => print!("->Gr {}", quantity),
                        | Fire => print!("->Fi -"),
                        | Coin(quantity) => print!("->Co {}", quantity),
                        | Bin(range) => print!("->Bi {}-{}", range.start, range.end),
                        | Crate(range) => print!("->Cr {}-{}", range.start, range.end),
                        | Bank(range) => print!("->Ba {}-{}", range.start, range.end),
                        | Water(quantity) => print!("->Wa {}", quantity),
                        | Content::None => print!("->No -"),
                        | Content::Fish(quantity) => print!("->Fh {}", quantity),
                        | Content::Market(quantity) => print!("->Mk {}", quantity),
                        | Content::Building => print!("->Bui -"),
                        | Content::Bush(quantity) => print!("->Bu {}", quantity),
                        | Content::JollyBlock(quantity) => print!("->Jo {}", quantity),
                        | Content::Scarecrow => print!("->Sc -"),
                    }
                    print!("\t| ");
                }
                println!();
            }
            let _ = go(self, world, Down);
            println!("{:?}", self.get_coordinate());
            let _ = go(self, world, Direction::Up);
            println!("{:?}", self.get_coordinate());
            println!("{:?}", one_direction_view(self, world, Direction::Right, 2))
        }

        fn handle_event(&mut self, event: Event) {
            println!("{:?}", event)
        }

        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }

        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
    }

    let r = MyRobot(Robot::new());
    let mut generator = WorldGenerator;

    struct Tool;

    impl Tools for Tool {}
    let mut run = Runner::new(Box::new(r), &mut generator).unwrap();
    match run.game_tick() {
        | Ok(_) => println!("Seccesful game tick"),
        | Err(e) => println!("{:?}", e),
    }
}
