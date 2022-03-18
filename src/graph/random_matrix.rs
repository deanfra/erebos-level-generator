use super::MapGraph;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoEdgeReferences;
use rand::{prelude::SliceRandom, Rng};

const MAX_CONNECTIONS: usize = 2;

pub fn new() -> MapGraph {
  let mut rng = rand::thread_rng();
  let mut graph = StableGraph::<usize, usize>::new();

  let mut all_nodes: Vec<NodeIndex<u32>> = Vec::new();
  // Ensures any new created nodes are connected to some of the previous
  let mut connected_nodes: Vec<NodeIndex<u32>> = Vec::new();

  let node_count: usize = rng.gen_range(50..70);

  // Create a random amount of new nodes
  for i in 0..node_count {
    let node = graph.add_node(i);
    all_nodes.push(node);
  }

  // start connecting nodes
  connected_nodes.push(all_nodes[0]);

  for i in 0..node_count {
    // randomly select two viable nodes
    let random_nodes = get_random_nodes(&graph, &connected_nodes, &all_nodes);

    if let Some((node_1, node_2)) = random_nodes {
      if !connected_nodes.contains(&node_2) {
        connected_nodes.push(node_2)
      }

      graph.add_edge(node_1, node_2, i);
    }
  }

  // Reset node weights to be sequentially heavier
  for (i, nw) in graph.node_weights_mut().enumerate() {
    *nw = i + 1;
  }

  MapGraph { graph, nodes: all_nodes }
}

fn can_make_edge(
  graph: &StableGraph<usize, usize>,
  edges: Vec<(NodeIndex, NodeIndex)>,
  node_1: NodeIndex<u32>,
  node_2: NodeIndex<u32>,
) -> bool {
  let is_same = node_1 == node_2;

  let edge_exists = edges.contains(&(node_1, node_2)) || edges.contains(&(node_2, node_1));
  let below_max_connections = graph.edges(node_1).count() < MAX_CONNECTIONS && graph.edges(node_2).count() < MAX_CONNECTIONS;

  below_max_connections && !is_same && !edge_exists
}

/// Randomly attempts to make a connection with all currently connected nodes
fn get_random_nodes(
  graph: &StableGraph<usize, usize>,
  connected: &Vec<NodeIndex<u32>>,
  all_nodes: &Vec<NodeIndex<u32>>,
) -> Option<(NodeIndex<u32>, NodeIndex<u32>)> {
  let mut rng = rand::thread_rng();
  let mut remaining_nodes = connected.clone();
  let edges: Vec<(NodeIndex, NodeIndex)> = graph.edge_references().map(|e| (e.source(), e.target())).collect();

  while remaining_nodes.len() != 0 {
    let node_1 = remaining_nodes.choose(&mut rng).unwrap();
    let node_2 = all_nodes.choose(&mut rng).unwrap();

    if can_make_edge(&graph, edges.clone(), *node_1, *node_2) {
      return Some((*node_1, *node_2));
    } else {
      // remove node from remaining_nodes
      let index = remaining_nodes.iter().position(|x| *x == *node_1).unwrap();
      remaining_nodes.remove(index);
    }
  }

  None
}
