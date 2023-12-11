use std::collections::HashMap;

use rand::Rng;
use strum::IntoEnumIterator;

use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::Tools;
use robotics_lib::interface::{craft, debug, destroy, go, look_at_sky, teleport, Direction};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{
    Bank, Bin, Building, Bush, Coin, Crate, Fire, Fish, Garbage, JollyBlock, Market, Rock, Scarecrow, Tree, Water,
};
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport,
};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;

fn main() {
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
                        | 9 => Teleport(false),
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
                        | 10 => Fish(3),
                        | 11 => Market(20),
                        | 12 => Building,
                        | 13 => Bush(2),
                        | 14 => JollyBlock(2),
                        | 15 => Scarecrow,
                        | _ => Content::None,
                    };
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
            for _ in 0..1 {
                let (tmp, a, b) = debug(self, world);
                let environmental_conditions = look_at_sky(world);
                println!(
                    "Daytime: {:?}, Time:{:?}, Weather: {:?}\n",
                    environmental_conditions.get_time_of_day(),
                    environmental_conditions.get_time_of_day_string(),
                    environmental_conditions.get_weather_condition()
                );
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
                            | TileType::Wall => {
                                print!("Wl");
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
                            | Fish(quantity) => print!("->Fh {}", quantity),
                            | Market(quantity) => print!("->Mk {}", quantity),
                            | Building => print!("->Bui -"),
                            | Bush(quantity) => print!("->Bu {}", quantity),
                            | JollyBlock(quantity) => print!("->Jo {}", quantity),
                            | Scarecrow => print!("->Sc -"),
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
            println!("HERE {:?}", destroy(self, world, Direction::Down));
            let _ = go(self, world, Direction::Down);
            println!("CRAFT: {:?}", craft(self, Content::Garbage(0)));
            println!("\n\nBACKPACK: {:?}\n\n", self.get_backpack());
            println!("HERE {:?}", teleport(self, world, (1, 1)));
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
    let mut generator = WorldGenerator::init(4);
    let run = Runner::new(Box::new(r), &mut generator);

    //Known bug: 'check_world' inside 'Runner::new()' fails every time
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}
