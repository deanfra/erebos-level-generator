use super::GraphResult;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoEdgeReferences;

/// Generate an undirected binomial tree of order n recursively.
///
/// :param int order: Order of the binomial tree. The maximum allowed value
///     for order on the platform your running on. If it's a 64bit platform
///     the max value is 59 and on 32bit systems the max value is 29. Any order
///     value above these will raise a ``OverflowError``.
/// :param list weights: A list of node weights. If the number of weights is
///     less than 2**order extra nodes with with None will be appended.
/// :param bool multigraph: When set to False the output
///     :class:`~retworkx.PyGraph` object will not be not be a multigraph and
///     won't  allow parallel edges to be added. Instead
///     calls which would create a parallel edge will update the existing edge.
///
/// :returns: A binomial tree with 2^n vertices and 2^n - 1 edges.
/// :rtype: PyGraph
/// :raises IndexError: If the length of ``weights`` is greater that 2^n
/// :raises OverflowError: If the input order exceeds the maximum value for the
///     current platform.
///
/// .. jupyter-execute::
///
///   import retworkx.generators
///   from retworkx.visualization import mpl_draw
///
///   graph = retworkx.generators.binomial_tree_graph(4)
///   mpl_draw(graph)
///

pub fn new(order: u32, bidirectional: bool) -> GraphResult {
  let num_nodes = usize::pow(2, order);
  let num_edges = usize::pow(2, order) - 1;
  let mut graph = StableGraph::<usize, usize>::with_capacity(num_nodes, num_edges);
  let mut nodes: Vec<NodeIndex> = Vec::new();

  for i in 0..num_nodes {
    nodes.push(graph.add_node(i))
  }

  let mut n = 1;
  let zero_index = NodeIndex::new(0);

  for _ in 0..order {
    let edges: Vec<(NodeIndex, NodeIndex)> = graph.edge_references().map(|e| (e.source(), e.target())).collect();

    for (source, target) in edges {
      let source_index = NodeIndex::new(source.index() + n);
      let target_index = NodeIndex::new(target.index() + n);

      if graph.find_edge(source_index, target_index).is_none() {
        graph.add_edge(source_index, target_index, 0);
      }

      if bidirectional && graph.find_edge(target_index, source_index).is_none() {
        graph.add_edge(target_index, source_index, 0);
      }
    }
    let n_index = NodeIndex::new(n);

    if graph.find_edge(zero_index, n_index).is_none() {
      graph.add_edge(zero_index, n_index, 0);
    }

    if bidirectional && graph.find_edge(n_index, zero_index).is_none() {
      graph.add_edge(n_index, zero_index, 0);
    }

    n *= 2;
  }

  (graph, nodes)
}
