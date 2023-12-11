pub mod my_robot {

    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;
    use robotics_lib::interface::{craft, debug, destroy, Direction, go, look_at_sky, one_direction_view, teleport};
    use robotics_lib::runner::{Robot, Runnable};
    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::tile::Content::{Bank, Bin, Coin, Crate, Fish, Garbage, Market, Rock, Tree, Water, Fire, Bush, JollyBlock};
    use robotics_lib::world::tile::{Content, TileType};

    use robotics_lib::world::World;

    use robotics_lib::world::tile::TileType::{Teleport, DeepWater, ShallowWater, Sand, Grass, Street, Hill, Mountain, Snow, Lava, Wall};



    pub struct MyRobot(pub Robot);


    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            /*for _ in 0..1 {
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
                            | robotics_lib::world::tile::Content::Building => print!("->Bui -"),
                            | Bush(quantity) => print!("->Bu {}", quantity),
                            | JollyBlock(quantity) => print!("->Jo {}", quantity),
                            | robotics_lib::world::tile::Content::Scarecrow => print!("->Sc -"),
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
            println!("HERE {:?}", teleport(self, world, (1, 1)));*/

            let var = one_direction_view(self, &world, Direction::Down, 10);
            println!("{:?}", var);
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


}