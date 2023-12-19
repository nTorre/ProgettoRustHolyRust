use super::*;
use crate::tools::dijkstra::*;

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

    // Utilizzo della funzione change_matrix per ottenere una matrice di Node
    let (matrix_node, target_nodes) = change_matrix(matrix_tile, TileTypeOrContent::TileType(TileType::Hill));

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
    //let target_nodes = vec![5];

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


/*PIETRO VERONA*/

#[test]
fn build_path_test() {
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
            Tile {
                tile_type: TileType::Hill,
                content: Content::Building,
                elevation: 3,
            }
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
            Tile {
                tile_type: TileType::Hill,
                content: Content::Building,
                elevation: 3,
            }
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
            Tile {
                tile_type: TileType::Hill,
                content: Content::Building,
                elevation: 3,
            }
        ],
    ];

    let coordinates = get_coordinates(&matrix_tile);

    let (matrix_node, target_nodes) = change_matrix(matrix_tile, TileTypeOrContent::TileType(TileType::Hill));

    for (i,v) in matrix_node.iter().enumerate() {
        println!("Node {}: {:?}",i,v);
    }

    println!("/////////////////////////////////////");

    let start_node = 0;
    let mut target_nodes:Vec<usize> = vec![5, 8, 6];
    target_nodes = find_connected_targets(&matrix_node, start_node, &target_nodes);

    let results = match build_path(&matrix_node, start_node, target_nodes, &coordinates) {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("Errore durante la costruzione del percorso: {}", err);
            return ();

        }
    };

    for row in results {
        for direction in row {
            match direction {
                Direction::Up => print!("Up "),
                Direction::Down => print!("Down "),
                Direction::Left => print!("Left "),
                Direction::Right => print!("Right "),
            }
        }
        println!();
    }
}