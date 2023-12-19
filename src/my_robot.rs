pub mod my_robot {
    use std::fmt::{Debug, Formatter, write};
    use std::path::Display;
    use strum_macros::Display;
    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;
    use robotics_lib::interface::{craft, debug, destroy, Direction, go, look_at_sky, one_direction_view, teleport};
    use robotics_lib::runner::{Robot, Runnable};
    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::utils::LibError;
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::tile::Content::{Bank, Bin, Coin, Crate, Fish, Garbage, Market, Rock, Tree, Water, Fire, Bush, JollyBlock};
    use robotics_lib::world::tile::{Content, Tile, TileType};

    use robotics_lib::world::World;

    use robotics_lib::world::tile::TileType::{Teleport, DeepWater, ShallowWater, Sand, Grass, Street, Hill, Mountain, Snow, Lava, Wall};

    pub struct MyRobot(pub Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {

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
