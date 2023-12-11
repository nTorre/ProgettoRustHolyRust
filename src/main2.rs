#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::{Hyperlink, Response, Vec2, Widget, Label, Pos2};
use egui::{vec2, Color32, Ui, WidgetInfo, widgets};
use egui::Rect;
use rand::prelude::*;

use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::interface::{debug, destroy, look_at_sky};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{run, Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{Bank, Bin, Coin, Crate, Fire, Garbage, Rock, Tree, Water};
use robotics_lib::world::tile::TileType::{DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::Generator;
use robotics_lib::world::World;
use strum::IntoEnumIterator;

extern crate noise;

use noise::{utils::*, Abs, Perlin, Seedable, Clamp};

const DIM: usize = 700;

fn main() -> Result<(), eframe::Error> {
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
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
            let mut rng = rand::thread_rng();
            let mut map: Vec<Vec<Tile>> = Vec::new();


            let perlin = Perlin::default().set_seed(rng.gen());
            let abs: Abs<f64, Perlin, 3> = Abs::new(perlin);
            print!("Ciao");
            let noise_map = &PlaneMapBuilder::new(abs).set_size(DIM, DIM).build();

            for i in 0..DIM{
                let mut row: Vec<Tile> = Vec::new();
                for j in 0..DIM{
                    let depth = noise_map.get_value(i, j);
                    if depth > 0.65{
                        row.push(Tile{tile_type: TileType::Snow, content: Content::None, elevation: 1});
                    } else if depth > 0.55{
                        row.push(Tile{tile_type: TileType::Mountain, content: Content::None, elevation: 1});
                    } else if depth > 0.3 {
                        row.push(Tile{tile_type: TileType::Hill, content: Content::None, elevation: 1});
                    } else if depth > 0.15{
                        row.push(Tile{tile_type: TileType::Grass, content: Content::None, elevation: 1});
                    } else if depth > 0.10{
                        row.push(Tile{tile_type: TileType::Sand, content: Content::None, elevation: 1});
                    } else if depth > 0.01 {
                        row.push(Tile { tile_type: TileType::ShallowWater, content: Content::None, elevation: 1 });
                    } else {
                        row.push(Tile{tile_type: TileType::DeepWater, content: Content::None, elevation: 1});
                    }
                    print!("{:?}", depth);
                }
                map.push(row);
            }

            /*
            let perlin = Perlin::default().set_seed(rng.gen());
            let clamp: Clamp<f64, Perlin, 3>  = Clamp::new(perlin).set_lower_bound(0.00020).set_upper_bound(2.0);
            let noise_map = &PlaneMapBuilder::new(clamp).build();

            for i in 0..DIM{
                let mut row: Vec<Tile> = Vec::new();
                for j in 0..DIM{
                    let depth = noise_map.get_value(i, j);
                    if depth > 0.8{
                        row.push(Tile{tile_type: TileType::Mountain, content: Content::None, elevation: 1});
                    } else if depth > 0.06 {
                        row.push(Tile{tile_type: TileType::Hill, content: Content::None, elevation: 1});
                    } else if depth > 0.005{
                        row.push(Tile{tile_type: TileType::Grass, content: Content::None, elevation: 1});
                    } else if depth > 0.0008{
                        row.push(Tile{tile_type: TileType::ShallowWater, content: Content::None, elevation: 1});
                    } else {
                        row.push(Tile{tile_type: TileType::DeepWater, content: Content::None, elevation: 1});
                    }
                    print!("{:?}", depth);
                }
                map.push(row);
            }*/

            let environmental_conditions = EnvironmentalConditions::new(&vec![Sunny, Rainy], 15, 12);
            (map, (0, 0), environmental_conditions)
        }
    }
    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            for _ in 0..1 {
                let (tmp, a, b) = debug(self, world);
                let environmental_conditions = look_at_sky(self, world);
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
            println!(
                "HERE {:?}",
                destroy(self, world, robotics_lib::interface::Direction::Down)
            );
        }

        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
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

    let mut r = MyRobot(Robot::new());
    let mut generator = WorldGenerator::init(30);
    let (map, (startx, starty), environmental_conditions) = generator.gen();

    let mut myapp = MyApp::default();

    let size:f32 = 0.9;
    let mut i: i32 = 0;

    let mut j: i32 = 0;

    for row in map{

        for tile in row{
            let mut color;
            match tile.tile_type {
                TileType::DeepWater => { color = Color32::DARK_BLUE },
                TileType::Grass => { color = Color32::GREEN },
                TileType::Hill => { color = Color32::DARK_GREEN },
                TileType::Lava => { color = Color32::RED }
                TileType::Mountain => { color = Color32::GRAY }
                TileType::Sand => { color = Color32::YELLOW }
                TileType::Snow => { color = Color32::WHITE }
                TileType::Street => { color = Color32::BLACK }
                TileType::ShallowWater => { color = Color32::BLUE }
            }
            let mut square = Square::new(
                size,
                size * (j as f32),
                size * (i as f32),
                color);
            myapp.add_square(square);
            j+=1;
        }
        j=0;
        i+=1;
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            Box::new(myapp)
        }),
    )

    //println!("{:?}", run(&mut r, &mut generator));
}

struct MyApp {
    name: String,
    age: u32,
    squares: Vec<Square>,
}

#[derive(Clone, Copy)]
struct Square{
    color: Color32,
    size: f32,
    x: f32,
    y: f32
}

impl Square{
    fn new(size: f32, x: f32, y: f32, color: Color32)->Self{
        Self{
            color,
            size,
            x,
            y
        }
    }
}

impl Widget for Square{
    fn ui(self, ui: &mut Ui) -> Response {

        let rect = Rect::from_min_size(Pos2{x: self.x, y: self.y}, Vec2{x: self.size, y: self.size});
        ui.painter().rect_filled(rect, 0.0, self.color);
        let response = ui.horizontal(|ui| {}).response;
        response
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            squares: vec![]
        }
    }
}

fn random() -> u8 {
    rand::thread_rng().gen_range(0..=255)
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // init world



        egui::CentralPanel::default().show(ctx, |ui| {
            for square in &self.squares {
                square.ui(ui);
            }
        });
    }
}


impl MyApp{
    fn add_square(&mut self, square: Square){
        self.squares.push(square);
    }
}

