use super::*;
use crate::tools::dijkstra::*;
use crate::tools::search_and_go::*;
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

    search_and_go(matrix_tile, crate::tools::search_and_go::TileTypeOrContent::TileType(TileType::Hill), start_node);
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

    let (matrix_node, target_nodes) = change_matrix(matrix_tile, crate::tools::dijkstra::TileTypeOrContent::TileType(TileType::Hill));

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