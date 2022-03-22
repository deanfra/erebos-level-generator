use super::GraphResult;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoEdgeReferences;
use std::iter;

/// Generate an undirected barbell graph where two identical mesh graphs are
/// connected by a path.
///
/// If ``num_path_nodes`` (described below) is not specified then this is
/// equivalent to two mesh graphs joined together.
///
/// :param int num_mesh_nodes: The number of nodes to generate the mesh graphs
///     with. Node weights will be None if this is specified. If both
///     ``num_mesh_nodes`` and ``mesh_weights`` are set this will be ignored and
///     ``mesh_weights`` will be used.
/// :param int num_path_nodes: The number of nodes to generate the path
///     with. Node weights will be None if this is specified. If both
///     ``num_path_nodes`` and ``path_weights`` are set this will be ignored and
///     ``path_weights`` will be used.
/// :param bool multigraph: When set to False the output
///     :class:`~retworkx.PyGraph` object will not be not be a multigraph and
///     won't  allow parallel edges to be added. Instead
///     calls which would create a parallel edge will update the existing edge.
///
/// :returns: The generated barbell graph
/// :rtype: PyGraph
/// :raises IndexError: If ``num_mesh_nodes`` is not specified
///
/// .. jupyter-execute::
///
///   import retworkx.generators
///   from retworkx.visualization import mpl_draw
///
///   graph = retworkx.generators.barbell_graph(4, 2)
///   mpl_draw(graph)
///

pub fn new(num_mesh_nodes: usize, num_path_nodes: usize) -> GraphResult {
  let mut left_mesh = StableGraph::<usize, usize>::default();
  let mut mesh_nodes: Vec<NodeIndex> = (0..num_mesh_nodes).map(|w| left_mesh.add_node(w)).collect();
  let mut nodelen = mesh_nodes.len();

  for i in 0..nodelen - 1 {
    for j in i + 1..nodelen {
      left_mesh.add_edge(mesh_nodes[i], mesh_nodes[j], 0);
    }
  }

  let right_mesh = left_mesh.clone();

  let path_nodes: Vec<NodeIndex> = (0..num_path_nodes).map(|w| left_mesh.add_node(w)).collect();
  left_mesh.add_edge(NodeIndex::new(nodelen - 1), NodeIndex::new(nodelen), 0);

  nodelen += path_nodes.len();

  for (node_a, node_b) in pairwise(path_nodes.clone()) {
    match node_a {
      Some(node_a) => left_mesh.add_edge(node_a, node_b, 0),
      None => continue,
    };
  }

  for node in right_mesh.node_indices() {
    let new_node = &right_mesh[node];
    mesh_nodes.push(left_mesh.add_node(new_node.clone()));
  }

  left_mesh.add_edge(NodeIndex::new(nodelen - 1), NodeIndex::new(nodelen), 0);
  for edge in right_mesh.edge_references().rev() {
    let new_source = NodeIndex::new(nodelen + edge.source().index());
    let new_target = NodeIndex::new(nodelen + edge.target().index());
    let weight = edge.weight();
    left_mesh.add_edge(new_source, new_target, weight.clone());
  }

  // errors with .concat(), marked as unstable?
  let mut nodes: Vec<NodeIndex> = mesh_nodes.clone();
  path_nodes.iter().for_each(|n| {
    nodes.push(*n);
  });

  // Reset node weights to be sequentially heavier
  for (i, nw) in left_mesh.node_weights_mut().enumerate() {
    *nw = i;
  }

  (left_mesh, nodes)
}

pub fn pairwise<I>(right: I) -> impl Iterator<Item = (Option<I::Item>, I::Item)>
where
  I: IntoIterator + Clone,
{
  let left = iter::once(None).chain(right.clone().into_iter().map(Some));
  left.zip(right)
}
