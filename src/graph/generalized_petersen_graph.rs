use super::GraphResult;
use petgraph::stable_graph::{NodeIndex, StableGraph};

/// Generate a generalized Petersen graph :math:`G(n, k)` with :math:`2n`
/// nodes and :math:`3n` edges. See Watkins [1]_ for more details.
///
/// .. note::
///   
///   The Petersen graph itself is denoted :math:`G(5, 2)`
///
/// :param int n: number of nodes in the internal star and external regular polygon.
/// :param int k: shift that changes the internal star graph.
/// :param bool multigraph: When set to False the output
///     :class:`~retworkx.PyGraph` object will not be not be a multigraph and
///     won't allow parallel edges to be added. Instead
///     calls which would create a parallel edge will update the existing edge.
///
/// :returns: The generated generalized Petersen graph.

pub fn new(num_nodes: usize, shift: usize) -> GraphResult {
  if num_nodes < 3 {
    println!("num_nodes must be at least 3");
  }

  if shift == 0 || 2 * shift >= num_nodes {
    println!("shift is invalid: it must be positive and less than n/2");
  }

  let mut graph = StableGraph::<usize, usize>::with_capacity(2 * num_nodes, 3 * num_nodes);

  let star_nodes: Vec<NodeIndex> = (0..num_nodes).map(|w| graph.add_node(w * 2)).collect();

  let polygon_nodes: Vec<NodeIndex> = (0..num_nodes).map(|w| graph.add_node(w + 1 * 2)).collect();

  for i in 0..num_nodes {
    graph.add_edge(star_nodes[i], star_nodes[(i + shift) % num_nodes], 0);
  }

  for i in 0..num_nodes {
    graph.add_edge(polygon_nodes[i], polygon_nodes[(i + 1) % num_nodes], 0);
  }

  for i in 0..num_nodes {
    graph.add_edge(polygon_nodes[i], star_nodes[i], 0);
  }

  // errors with .concat(), marked as unstable?
  let mut nodes: Vec<NodeIndex> = star_nodes.clone();
  polygon_nodes.iter().for_each(|pn| {
    nodes.push(*pn);
  });

  // Reset node weights to be sequentially heavier
  for (i, nw) in graph.node_weights_mut().enumerate() {
    *nw = i + 1;
  }

  (graph, nodes)
}
