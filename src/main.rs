mod wgenerator;
mod my_robot;

use std::collections::HashMap;

use rand::Rng;
use strum::IntoEnumIterator;


use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{one_direction_view, Tools};
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

use wgenerator::wgenerator::WorldGenerator;
use my_robot::my_robot::MyRobot;


fn main() {


    let mut r = MyRobot(Robot::new()); // maybe it hasn't to be mutable
    let mut generator = WorldGenerator::init(10);
    let (map, _, environmental_conditions, max_score, test) = generator.gen();
    let run = Runner::new(Box::new(r), &mut generator);
    //Known bug: 'check_world' inside 'Runner::new()' fails every time
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }

}






