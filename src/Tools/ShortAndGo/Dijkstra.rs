use std::collections::BinaryHeap;
use std::cmp::Ordering;

const INF: i32 = std::i32::MAX;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    index: usize,
    distance: i32,
}

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
            let neighbor_distance = distance[neighbor.index].unwrap_or(INF);

            if new_distance < neighbor_distance {
                distance[neighbor.index] = Some(new_distance);
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

        let result = PathResult {
            path,
            target_node: *target_node,
            total_cost: shortest_distances[*target_node].unwrap(),
        };

        results.push(result);
    }

    results
}

fn main() {
    let graph = vec![
        vec![Node { index: 1, distance: 1 }],       // node 0
        vec![
            Node { index: 1, distance: 1 },
            Node { index: 3, distance: 1 },
            Node { index: 4, distance: 1 },
        ],                                         // node 1
        vec![],                                     // node 2
        vec![
            Node { index: 5, distance: 1 },
            Node { index: 7, distance: 2 },
            Node { index: 8, distance: 0 },
        ],                                         // node 3
        vec![Node { index: 5, distance: 2 }],       // node 4
        vec![Node { index: 6, distance: 3 }],       // node 5
        vec![],                                     // node 6
        vec![Node { index: 9, distance: 1 }],       // node 7
        vec![Node { index: 9, distance: 1 }],       // node 8
        vec![],                                     // node 9
    ];

    let start_node = 0;
    let target_nodes = vec![6,9];

    let results = find_shortest_paths(&graph, start_node, &target_nodes);

    for result in results {
        if let Some(path) = result.path {
            println!("Shortest path from node {} to node {}: {:?}", start_node, result.target_node, path);
            println!("Total cost: {}", result.total_cost);
        } else {
            println!("No path from node {} to node {}", start_node, result.target_node);
        }
    }
}