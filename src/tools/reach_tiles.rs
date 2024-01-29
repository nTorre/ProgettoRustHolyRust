use crate::interface::Tools;
use std::vec::Vec;
use crate::world::tile::Content;
use crate::world::tile::{Tile, TileType};
use std::usize;use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::fmt;

const INF: i32 = i32::MAX;

pub struct ReachTiles {

}

impl Tools for ReachTiles {

}

impl ReachTiles {
    /// Searches for target nodes (either TileType or Content) in a matrix of tiles and finds the shortest paths from the start node to the target nodes.
    ///
    /// # Arguments
    ///
    /// * `matrix_tile` - A matrix of tiles represented as a vector of vectors.
    /// * `tile_type_or_content` - An enum value indicating whether to search by tile type or tile content.
    /// * `start_node` - The index of the start node.
    ///
    /// # Returns
    ///
    /// A vector of `PathResult` structs, each representing a target node, its shortest path, and total cost.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use robotics_lib::energy::Energy;
    /// use robotics_lib::event::events::Event;
    /// use robotics_lib::interface::Tools;
    /// use robotics_lib::runner::{Robot, Runnable, Runner};
    /// use robotics_lib::runner::backpack::BackPack;
    /// use robotics_lib::world::coordinates::Coordinate;
    /// use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
    /// use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
    /// use robotics_lib::world::tile::{Content, Tile};
    /// use robotics_lib::world::tile::TileType::Grass;
    /// use robotics_lib::world::World;
    /// use robotics_lib::world::world_generator::Generator;
    /// use crate::tools::reach_tiles::ReachTiles;
    /// use crate::tools::reach_tiles::TileTypeOrContent;
    /// use robotics_lib::world::tile::TileType;
    ///
    /// fn main() {
    ///     struct MyRobot(Robot);
    ///     struct WorldGenerator {
    ///         size: usize,
    ///     }
    ///     impl WorldGenerator {
    ///         fn init(size: usize) -> Self {
    ///             WorldGenerator { size }
    ///         }
    ///     }
    ///     impl Generator for WorldGenerator {
    ///         fn gen(
    ///             &mut self,
    ///         ) -> (
    ///             Vec<Vec<Tile>>,
    ///             (usize, usize),
    ///             EnvironmentalConditions,
    ///             f32,
    ///             Option<HashMap<Content, f32>>,
    ///         ) {
    ///             let mut map: Vec<Vec<Tile>> = Vec::new();
    ///             // Initialize the map with default tiles
    ///             for _ in 0..self.size {
    ///                 let mut row: Vec<Tile> = Vec::new();
    ///                 for _ in 0..self.size {
    ///                     let tile_type = Grass;
    ///                     let content = Content::None;
    ///                     row.push(Tile {
    ///                         tile_type,
    ///                         content,
    ///                         elevation: 0,
    ///                     });
    ///                 }
    ///                 map.push(row);
    ///             }
    ///             let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();
    ///
    ///             let max_score = rand::random::<f32>();
    ///
    ///             (map, (0, 0), environmental_conditions, max_score, None)
    ///         }
    ///     }
    ///     impl Runnable for MyRobot {
    ///         fn process_tick(&mut self, world: &mut World) {
    ///             // Creazione di una matrice di Tile per test
    ///             let matrix_tile: Vec<Vec<Tile>> = vec![
    ///                 vec![
    ///                     Tile {
    ///                         tile_type: TileType::Grass,
    ///                         content: Content::Building,
    ///                         elevation: 0,
    ///                     },
    ///                     Tile {
    ///                         tile_type: TileType::Hill,
    ///                         content: Content::Building,
    ///                         elevation: 3,
    ///                     },
    ///                 ],
    ///                 vec![
    ///                     Tile {
    ///                         tile_type: TileType::DeepWater,
    ///                         content: Content::Building,
    ///                         elevation: 0,
    ///                     },
    ///                     Tile {
    ///                         tile_type: TileType::Sand,
    ///                         content: Content::Building,
    ///                         elevation: 2,
    ///                     },
    ///                 ],
    ///                 vec![
    ///                     Tile {
    ///                         tile_type: TileType::Hill,
    ///                         content: Content::Fire,
    ///                         elevation: 0,
    ///                     },
    ///                     Tile {
    ///                         tile_type: TileType::Sand,
    ///                         content: Content::Fire,
    ///                         elevation: 2,
    ///                     },
    ///                 ],
    ///             ];
    ///
    ///             let start_node = 0;
    ///             ReachTiles::reach_tiles(matrix_tile, TileTypeOrContent::TileType(TileType::Hill), start_node);
    ///         }
    ///
    ///         fn handle_event(&mut self, event: Event) {
    ///             println!();
    ///             println!("{:?}", event);
    ///             println!();
    ///         }
    ///
    ///         fn get_energy(&self) -> &Energy {
    ///             &self.0.energy
    ///         }
    ///         fn get_energy_mut(&mut self) -> &mut Energy {
    ///             &mut self.0.energy
    ///         }
    ///
    ///         fn get_coordinate(&self) -> &Coordinate {
    ///             &self.0.coordinate
    ///         }
    ///         fn get_coordinate_mut(&mut self) -> &mut Coordinate {
    ///             &mut self.0.coordinate
    ///         }
    ///
    ///         fn get_backpack(&self) -> &BackPack {
    ///             &self.0.backpack
    ///         }
    ///         fn get_backpack_mut(&mut self) -> &mut BackPack {
    ///             &mut self.0.backpack
    ///         }
    ///     }
    ///
    ///     let r = MyRobot(Robot::new());
    ///     struct Tool;
    ///     impl Tools for Tool {}
    ///     let mut generator = WorldGenerator::init(100);
    ///     let run = Runner::new(Box::new(r), &mut generator);
    ///
    ///     //Known bug: 'check_world' inside 'Runner::new()' fails every time
    ///     match run {
    ///         | Ok(mut r) => {
    ///             let _ = r.game_tick();
    ///         }
    ///         | Err(e) => println!("{:?}", e),
    ///     }
    /// }
    /// ```
    ///
    pub fn reach_tiles(matrix_tile: Vec<Vec<Tile>>, tile_type_or_content: TileTypeOrContent, start_node: usize) -> Vec<PathResult>{
        let (matrix_node, target_nodes) = change_matrix(matrix_tile, tile_type_or_content);

        print_result(&matrix_node, start_node, &target_nodes);

        return find_shortest_paths(&matrix_node, start_node, &target_nodes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    index: usize,
    distance: usize,
    direction: Direction,
}

impl Node {
    fn new(index: usize, weight: usize, direction: Direction) -> Node {
        Node {
            index: index,
            distance: weight,
            direction: direction,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PathResult {
    path: Option<Vec<Direction>>,
    target_node: usize,
    total_cost: i32,
}

pub enum TileTypeOrContent {
    TileType(TileType),
    Content(Content)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

/// Determines the cost of walking on a Tile.
///
/// # Arguments
///
/// * `tile - A reference to a Tile struct.
///
/// # Returns
///
/// The cost of the tile as a usize (if the tile is non walkable it returns INF)
fn get_cost (tile: &Tile) -> usize {
    match &tile.tile_type {
        TileType::DeepWater | TileType::Lava | TileType::Wall => INF as usize,
        _ => tile.tile_type.properties().cost()
    }
}

/// Checks if a Tile is walkable based on its TileType.
///
/// # Arguments
///
/// * `tile` - A reference to a Tile struct.
///
/// # Returns
///
/// A boolean that determines whether the tile is walkable or not.
fn is_walkable(tile: &Tile) -> bool {
    match &tile.tile_type {
        TileType::DeepWater | TileType::Lava | TileType::Wall => {return false;}
        _ => true
    }
}

/// Returns the cost of moving to a Tile with higher elevation.
///
/// # Arguments
///
/// * `tile_arrive` - A reference to a Tile struct that indicates the arriving point .
/// * `tile_start` - A reference to a Tile struct that indicates the starting point.
///
/// # Returns
///
/// A usize which indicates the cost of traveling from the starting tile to the arrival tile based on the elevation difference
fn get_cost_elevation (tile_arrive: &Tile, tile_start: &Tile) -> usize {
    if tile_arrive.elevation <= tile_start.elevation {
        return 0;
    }
    (tile_arrive.elevation - tile_start.elevation).pow(2)
}



/// Retrieves the neighbouring nodes of a given tile in a matrix of Tile.
///
/// # Arguments
///
/// * `matrix_tile` - A reference to a matrix of Tile.
/// * `x` - The x-coordinate of the tile.
/// * `y` - The y-coordinate of the tile.
/// * `index` - The index of the tile.
/// * `tile` - A reference to a Tile struct.
///
/// # Returns
///
/// Returns a vector of the neighbouring nodes of the tile given as a parameter
fn get_neighbours(
    matrix_tile: &Vec<Vec<Tile>>,
    x: usize,
    y: usize,
    _index: usize,
    tile: &Tile,
) -> Vec<Node> {
    let rows = matrix_tile.len();
    let cols = matrix_tile[0].len();
    let mut vec = Vec::new();

    // Using an offset to determine the neighbours of index
    let offsets = [
        (0, 1, Direction::Right),
        (1, 0, Direction::Down),
        (0, -1, Direction::Left),
        (-1, 0, Direction::Up),
    ];

    for (offset_x, offset_y, direction) in offsets.iter() {
        // if the new_x or new_y is negative then we skip the cycle
        if (*offset_x == -1 && x == 0) || (*offset_y == -1 && y == 0) {
            continue;
        }

        let new_x = (x as i32 + offset_x) as usize;
        let new_y = (y as i32 + offset_y) as usize;

        // Checking if the new position is not out of bounds
        if new_x < rows && new_y < cols {
            let neighbour_tile = &matrix_tile[new_x][new_y];

            // if the tile is walkable we add a cost and push that to the vector
            if is_walkable(neighbour_tile) {
                let mut cost = get_cost(neighbour_tile);

                if neighbour_tile.elevation != 0 {
                    cost += get_cost_elevation(neighbour_tile, tile);
                }

                // The new index is equal to new_x * cols (which gives us the starting index of the relative row) + new_y
                let new_index = new_x * cols + new_y;
                vec.push(Node::new(new_index, cost, direction.clone()));
            }
        }
    }

    vec
}

/// Transforms a matrix of TileType (or Content) into a tuple of matrix of Node and Vector of target nodes, where:
///
/// * `matrix_tile` - The original matrix represented as a Vector of Vector of Tile.
/// * `tile_or_content` - Either a TileType or a Content
///
/// For example given a matrix of TileType:
///
/// |        |        |
/// |------------|------------|
/// | Sand       | DeepWater  |
/// | Hill       | Grass      |
/// |            |            |
/// returns a tuple of matrix of Nodes, where each node's labels (index) are assigned row by row starting from the top left corner, that is:
/// * `Sand` => 0
/// * `DeepWater` => 1
/// * `Hill` => 2
/// * `Grass` => 3
/// and a vector of target nodes that are either TileType or Content.
///
fn change_matrix(matrix_tile: Vec<Vec<Tile>>, tile_or_content: TileTypeOrContent) -> (Vec<Vec<Node>>, Vec<usize>) {
    let rows = matrix_tile.len();
    let cols = matrix_tile[0].len();
    let mut matrix_node = vec![vec![]; rows * cols];
    let mut label_node = 0;

    let mut target_nodes = vec![];

    for (x,rows) in matrix_tile.iter().enumerate() {
        for (y,tile) in rows.iter().enumerate() {
            let is_walkable = is_walkable(tile);
            match &tile_or_content {
                TileTypeOrContent::TileType(tile_type) => {
                    if tile.tile_type == tile_type.clone() && is_walkable{
                        target_nodes.push(label_node);
                    }
                },
                TileTypeOrContent::Content(my_content) => {
                    if tile.content == my_content.clone() && is_walkable{
                        target_nodes.push(label_node);
                    }
                }
            };
            if is_walkable {
                let neighbours = get_neighbours(&matrix_tile,x,y,label_node, &tile);
                for i in neighbours {
                    matrix_node[label_node].push(i);
                }
            }
            label_node += 1;
        }
    }
    (matrix_node,target_nodes)
}

/// Performs Dijkstra's algorithm to find the shortest paths from the start node to all other nodes in the graph.
///
/// # Arguments
///
/// * `graph` - The graph represented as a vector of vectors of Node.
/// * `start` - The index of the starting node.
///
/// # Returns
///
/// A tuple containing a vector of the shortest distances from the start node to all the nodes and a vector of Option<usize>
/// representing the predecessors of each node in the shortest path.
fn dijkstra(graph: &Vec<Vec<Node>>, start: usize) -> (Vec<Option<i32>>, Vec<Option<usize>>) {
    let mut distance: Vec<Option<i32>> = vec![None; graph.len()];
    let mut predecessor: Vec<Option<usize>> = vec![None; graph.len()];
    let mut visited: Vec<bool> = vec![false; graph.len()];

    distance[start] = Some(0);
    let mut heap = BinaryHeap::new();
    heap.push(Node { index: start, distance: 0, direction: Direction::Up });

    while let Some(Node { index, distance: dist, .. }) = heap.pop() {
        if visited[index] {
            continue;
        }
        visited[index] = true;

        for neighbor in &graph[index] {
            let new_distance = dist + neighbor.distance;
            let neighbor_distance: usize = distance[neighbor.index].unwrap_or(INF) as usize;

            if new_distance < neighbor_distance {
                distance[neighbor.index] = Some(new_distance as i32);
                predecessor[neighbor.index] = Some(index);
                heap.push(Node { index: neighbor.index, distance: new_distance, direction: Direction::Up });
            }
        }
    }

    (distance, predecessor)
}

/// Reconstructs the shortest path from the start node to the target node using the predecessors vector.
///
/// # Arguments
///
/// * `predecessor` - A vector containing the predecessors of each node in the path.
/// * `target` - The target node.
/// * `graph` - The graph represented as a vector of vector of Node.
///
/// # Returns
///
/// - A vector of Direction containing the directions to follow in order to reach the target node.
fn reconstruct_shortest_path(predecessors: Vec<Option<usize>>, target_node: usize, graph: &Vec<Vec<Node>>) -> Vec<Direction> {
    let mut path = Vec::new();
    let mut current = target_node;

    // Check if target_node is within bounds
    if current >= graph.len() {
        // Return empty path if out of bounds
        return path;
    }

    // Reconstruct the path by following the predecessors
    while let Some(prev) = predecessors[current] {
        // Check if current and prev indices are within bounds
        if current < graph.len() && prev < graph.len() {
            // Find the direction from prev to current
            for node in &graph[prev] {
                if node.index == current {
                    path.push(node.direction);
                    break;
                }
            }
        } else {
            // Either current or prev is out of bounds, return empty path
            return Vec::new();
        }

        current = prev;
    }

    path.reverse();
    path
}

/// Finds the shortest paths from a start node to multiple target nodes in a graph represented as a matrix.
///
/// This function uses Dijkstra's algorithm to calculate the shortest distances and then it reconstructs the paths through the function reconstruct_shortest_path
///
/// # Arguments
///
/// * `graph` - The graph represented as a vector of vectors of Node.
/// * `start` - The index of the starting node.
/// * `target_nodes` - A vector of indices representing the target nodes.
///
/// # Returns
///
/// A vector of `PathResult` structs, each one representing a target node, its respective shortest path, and total cost.
fn find_shortest_paths(graph: &Vec<Vec<Node>>, start: usize, target_nodes: &Vec<usize>) -> Vec<PathResult> {
    let (shortest_distances, predecessors) = dijkstra(graph, start);
    let mut results = Vec::new();

    for target_node in target_nodes {
        let path = reconstruct_shortest_path(predecessors.clone(), *target_node, graph);

        let tmp = match &shortest_distances[*target_node] {
            None => 0,
            Some(t) => *t,
        };

        let result = PathResult {
            path: Some(path),
            target_node: *target_node,
            total_cost: tmp,
        };

        results.push(result);
    }

    results
}


/// Prints the matrix of Node.
///
/// # Arguments
///
/// * `matrix_node` - The matrix of Node represented as a vector of vectors of Node.
/// * `start_node` - The index of the starting node.
/// * `target_nodes` - A vector of indices representing the target nodes.
///
fn print_result(matrix_node: &Vec<Vec<Node>>, start_node: usize, target_nodes: &Vec<usize>) {
    // Printing the matrix of Node
    println!("Matrix of Nodes:");
    for (i,v) in matrix_node.iter().enumerate() {
        println!("Node  {}: {:?}", i, v);
    }

    let results = find_shortest_paths(&matrix_node, start_node, &target_nodes);

    for result in results {
        if let Some(path) = result.path {
            println!(
                "\nShortest path from node {} to node {}:\n {}",
                start_node,
                result.target_node,
                path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" -> "));
            println!("Total cost: {}", result.total_cost);
        } else {
            println!("\nNo path from node {} to node {}", start_node, result.target_node);
        }
    }
}


