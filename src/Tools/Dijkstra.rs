use std::vec::Vec;
use crate::world::tile::Content;
use crate::world::tile::{Tile, TileType};

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

use std::usize;

fn get_cost (tile: &Tile) -> usize {
    match &tile.tile_type {
        TileType::DeepWater | TileType::Lava | TileType::Wall => 100000,
        _ => tile.tile_type.properties().cost()
    }
}

fn is_wakable (tile: &Tile) ->bool {
    match &tile.tile_type {
        TileType::DeepWater | TileType::Lava | TileType::Wall => {return false;}
        _ => true
    }
}



fn get_neighbours (matrix_tile: &Vec<Vec<Tile>>, x: usize, y: usize, value: usize, tile: &Tile) -> Vec<Node> {
    let rows = matrix_tile.len();
    let cols = matrix_tile[0].len();
    let mut vec = vec![];

    if (x as i32-1) >= 0 && (x as i32-1) < rows as i32 && (y) < cols {
        if is_wakable(&matrix_tile[x-1][y]) {
            if matrix_tile[x-1][y].elevation == 0 {
                vec.push(Node::new(value-cols,get_cost(&matrix_tile[x-1][y])));
            }
            else {
                vec.push(Node::new(value-cols,get_cost(&matrix_tile[x-1][y]) + ((matrix_tile[x-1][y].elevation - tile.elevation) as i32).pow(2) as usize));
            }
        }
    }
    if (x) < rows && (y+1) < cols {
        if is_wakable(&matrix_tile[x][y+1]) {
            if matrix_tile[x][y+1].elevation == 0 {
                vec.push(Node::new(value+1,get_cost(&matrix_tile[x][y+1])));
            }
            else {
                vec.push(Node::new(value+1,get_cost(&matrix_tile[x][y+1]) + ((&matrix_tile[x][y+1].elevation - tile.elevation) as i32).pow(2) as usize));
            }
        }
    }
    if (x+1) < rows && (y) < cols {
        if is_wakable(&matrix_tile[x+1][y]) {
            if matrix_tile[x+1][y].elevation == 0 {
                vec.push(Node::new(value+cols,get_cost(&matrix_tile[x+1][y])));
            }
            else {
                vec.push(Node::new(value+cols,get_cost(&matrix_tile[x+1][y]) + ((matrix_tile[x+1][y].elevation - tile.elevation) as i32).pow(2) as usize));
            }
        }
    }
    if (x) < rows && (y as i32-1) >= 0 && (y as i32-1) < cols as i32 {
        if is_wakable(&matrix_tile[x][y-1]) {}
        if matrix_tile[x][y-1].elevation == 0 {
            vec.push(Node::new(value-1,get_cost(&matrix_tile[x][y-1])));
        }
        else {
            vec.push(Node::new(value-1,get_cost(&matrix_tile[x][y-1]) + ((matrix_tile[x][y-1].elevation - tile.elevation) as i32).pow(2) as usize));
        }
    }
    vec
}

fn change_matrix(matrix_tile: Vec<Vec<Tile>>) -> Vec<Vec<Node>> {
    let rows = matrix_tile.len();
    let cols = matrix_tile[0].len();
    let mut matrix_node = vec![vec![]; rows * cols];
    let mut label_node = 0;

    for (x,rows) in matrix_tile.iter().enumerate() {
        for (y,tile) in rows.iter().enumerate() {
            if is_wakable(tile) {
                let neighbours = get_neighbours(&matrix_tile,x,y,label_node, &tile);
                for i in neighbours {
                    matrix_node[label_node].push(i);
                }
            }
            label_node += 1;
        }
    }
    matrix_node
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



/////////////////////////////////////////////////////

#[test]
fn test_change_matrix() {
    // Creazione di una matrice di Tile per test
    let matrix_tile: Vec<Vec<Tile>> = vec![
        vec![
            Tile {
                tile_type: TileType::Grass,
                content: Content::Fire,
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Lava,
                content: Content::Fire,
                elevation: 0,
            },
        ],
        vec![
            Tile {
                tile_type: TileType::DeepWater,
                content: Content::Fire,
                elevation: 0,
            },
            Tile {
                tile_type: TileType::Sand,
                content: Content::Fire,
                elevation: 0,
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
                elevation: 0,
            },
        ],
    ];

    // Utilizzo della funzione change_matrix per ottenere una matrice di Node
    let matrix_node = change_matrix(matrix_tile);

    // let rows = matrix_tile.len();
    // let cols = matrix_tile[0].len();
    // let mut matrix_node = vec![vec![]; rows * 2];
    //
    // matrix_node[0].push(Node::new(1,3));
    // matrix_node[0].push(Node::new(2,5));
    // matrix_node[1].push(Node::new(0,1));
    // matrix_node[1].push(Node::new(3,usize::MAX));
    // matrix_node[2].push(Node::new(0,1));
    // matrix_node[2].push(Node::new(3,usize::MAX));
    // matrix_node[3].push(Node::new(2,5));
    // matrix_node[3].push(Node::new(1,3));

    // Stampa della matrice di Node
    for (i,v) in matrix_node.iter().enumerate() {
        println!("Node {}: {:?}",i,v);
    }

    println!("/////////////////////////////////////");

    let start_node = 0;
    let target_nodes = vec![5];

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
