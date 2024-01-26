use std::vec::Vec;
use crate::world::tile::Content;
use crate::world::tile::{Tile, TileType};
use std::usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    index: usize,
    distance: usize,
}

impl Node {
    fn new(index: usize, weight: usize) -> Node {
        Node { index: index, distance: weight}
    }
}

enum TileTypeOrContent {
    TileType(TileType),
    Content(Content)
}


/// Returns the cost of walking on the Tile.
///
/// If the Tile is not walkable, it returns a high number (100000)
fn get_cost (tile: &Tile) -> usize {
    match &tile.tile_type {
        TileType::DeepWater | TileType::Lava | TileType::Wall => 100000,
        _ => tile.tile_type.properties().cost()
    }
}
/// Returns a boolean corresponding to the attribute walk of Tile
fn is_wakable (tile: &Tile) -> bool {
    match &tile.tile_type {
        TileType::DeepWater | TileType::Lava | TileType::Wall => {return false;}
        _ => true
    }
}

/// Returns the cost of moving to a Tile with higher elevation
fn get_cost_elevation (tile_arrive: &Tile, tile_start: &Tile) -> usize {
    if tile_arrive.elevation <= tile_start.elevation {
        return 0;
    }
    (tile_arrive.elevation - tile_start.elevation).pow(2)
}


/// Returns a vector of Node made by the neighbours of the tile given as a parameter in the function if they are walkable
fn get_neighbours (matrix_tile: &Vec<Vec<Tile>>, x: usize, y: usize, value: usize, tile: &Tile) -> Vec<Node> {
    let rows = matrix_tile.len();
    let cols = matrix_tile[0].len();
    let mut vec = vec![];
    // Tile at bottom
    if (x as i32-1) >= 0 && (x as i32-1) < rows as i32 && (y) < cols {
        if is_wakable(&matrix_tile[x-1][y]) {
            if matrix_tile[x-1][y].elevation == 0 {
                vec.push(Node::new(value-cols,get_cost(&matrix_tile[x-1][y])));
            }
            else {
                vec.push(Node::new(value-cols,get_cost(&matrix_tile[x-1][y]) + get_cost_elevation(&matrix_tile[x-1][y],tile)));
            }
        }
    }
    // Tile at right
    if (x) < rows && (y+1) < cols {
        if is_wakable(&matrix_tile[x][y+1]) {
            if matrix_tile[x][y+1].elevation == 0 {
                vec.push(Node::new(value+1,get_cost(&matrix_tile[x][y+1])));
            }
            else {
                vec.push(Node::new(value+1,get_cost(&matrix_tile[x][y+1]) + get_cost_elevation(&matrix_tile[x][y+1],tile)));
            }
        }
    }
    // Tile at top
    if (x+1) < rows && (y) < cols {
        if is_wakable(&matrix_tile[x+1][y]) {
            if matrix_tile[x+1][y].elevation == 0 {
                vec.push(Node::new(value+cols,get_cost(&matrix_tile[x+1][y])));
            }
            else {
                vec.push(Node::new(value+cols,get_cost(&matrix_tile[x+1][y]) + get_cost_elevation(&matrix_tile[x+1][y],tile)));
            }
        }
    }
    // Tile at left
    if (x) < rows && (y as i32-1) >= 0 && (y as i32-1) < cols as i32 {
        if is_wakable(&matrix_tile[x][y-1]) {
            if matrix_tile[x][y-1].elevation == 0 {
                vec.push(Node::new(value-1,get_cost(&matrix_tile[x][y-1])));
            }
            else {
                vec.push(Node::new(value-1,get_cost(&matrix_tile[x][y-1]) + get_cost_elevation(&matrix_tile[x][y-1],tile)));
            }
        }
    }
    vec
}


/// Transforms a matrix of TileType (or Content) into a matrix of Node, where:
///
/// * `index` - Identify the label of the node
/// * `weight` - Contain the cost of walking on that Tile
///
/// For example given a matrix of TileType:
///
/// |        |        |
/// |------------|------------|
/// | Sand       | DeepWater  |
/// | Hill       | Grass      |
/// |            |            |
/// returns a matrix of Nodes where each node's labels (index) are assigned row by row starting from the top right corner, that is:
/// * `Sand` => 0
/// * `DeepWater` => 1
/// * `Hill` => 2
/// * `Grass` => 3
fn change_matrix(matrix_tile: Vec<Vec<Tile>>, tile_or_content: TileTypeOrContent) -> (Vec<Vec<Node>>, Vec<usize>) {
    let rows = matrix_tile.len();
    let cols = matrix_tile[0].len();
    let mut matrix_node = vec![vec![]; rows * cols];
    let mut label_node = 0;

    let mut target_nodes = vec![];

    for (x,rows) in matrix_tile.iter().enumerate() {
        for (y,tile) in rows.iter().enumerate() {
            let is_walkable = is_wakable(tile);
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

////////////////////////////////////////////////////

use std::collections::BinaryHeap;
use std::cmp::Ordering;

const INF: i32 = std::i32::MAX;

struct PathResult {
    path: Option<Vec<usize>>,
    target_node: usize,
    total_cost: i32,
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


/// Performs Dijkstra's algorithm to find the shortest paths from the start node to all other nodes in the graph.
///
/// # Arguments
///
/// * `graph` - The graph represented as a vector of vectors of Node.
/// * `start` - The index of the starting node.
///
/// # Returns
///
/// A tuple containing a vector of the shortest distances from the start node to all the nodes and a vector of Option<usize> representing the predecessors of each node in the shortest path.
fn dijkstra(graph: &Vec<Vec<Node>>, start: usize) -> (Vec<Option<i32>>, Vec<Option<usize>>) {
    let mut distance: Vec<Option<i32>> = vec![None; graph.len()];
    let mut predecessor: Vec<Option<usize>> = vec![None; graph.len()];
    let mut visited: Vec<bool> = vec![false; graph.len()];

    distance[start] = Some(0);
    let mut heap = BinaryHeap::new();
    heap.push(Node { index: start, distance: 0 });

    while let Some(Node { index, distance: dist }) = heap.pop() {
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
                heap.push(Node { index: neighbor.index, distance: new_distance });
            }
        }
    }

    (distance, predecessor)
}

/// Reconstructs the shortest path from the start node to the target node using the predecessors vector.
fn reconstruct_shortest_path(predecessor: Vec<Option<usize>>, target: usize) -> Option<Vec<usize>> {
    let mut path = Vec::new();
    let mut current = target;

    while let Some(prev) = predecessor[current] {
        path.push(current);
        current = prev;
    }

    path.push(current);
    path.reverse();

    if path.len() == 0 {
        return None;
    }


    if path == [target] {  // If the path only contains the target node, there is no valid path
        None
    } else {
        Some(path)
    }
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
        let path = reconstruct_shortest_path(predecessors.clone(), *target_node);

        let tmp =match &shortest_distances[*target_node] {
            None => {0}
            Some(t) => {*t}
        };

        let result = PathResult {
            path,
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
fn print_result(matrix_node: &Vec<Vec<Node>>, start_node: usize, target_nodes: &Vec<usize>) {
    // Printing the matrix of Node
    for (i,v) in matrix_node.iter().enumerate() {
        println!("Node {}: {:?}", i, v);
    }
    println!("/////////////////////////////////////");

    let results = find_shortest_paths(&matrix_node, start_node, &target_nodes);

    for result in results {
        if let Some(path) = result.path {
            println!("Shortest path from node {} to node {}: {:?}", start_node, result.target_node, path);
            println!("Total cost: {}", result.total_cost);
        } else {
            println!("No path from node {} to node {}", start_node, result.target_node);
        }
    }
}


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
fn search_and_go(matrix_tile: Vec<Vec<Tile>>, tile_type_or_content: TileTypeOrContent, start_node: usize) -> Vec<PathResult>{
    let (matrix_node, target_nodes) = change_matrix(matrix_tile, tile_type_or_content);

    print_result(&matrix_node, start_node, &target_nodes);

    return find_shortest_paths(&matrix_node, start_node, &target_nodes);
}


/////////////////////////////////////////////////////

#[test]
fn test_change_matrix() {
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

    search_and_go(matrix_tile, TileTypeOrContent::TileType(TileType::Hill), start_node);
}

