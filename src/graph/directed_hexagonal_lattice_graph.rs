use super::GraphResult;
use petgraph::stable_graph::{NodeIndex, StableGraph};

/// Generate an undirected hexagonal lattice graph.
///
/// :param int rows: The number of rows to generate the graph with.
/// :param int cols: The number of columns to generate the graph with.
/// :param bool multigraph: When set to False the output
///     :class:`~retworkx.PyGraph` object will not be not be a multigraph and
///     won't  allow parallel edges to be added. Instead
///     calls which would create a parallel edge will update the existing edge.
///
/// :returns: The generated hexagonal lattice graph.
///
/// :rtype: PyGraph
/// :raises TypeError: If either ``rows`` or ``cols`` are
///      not specified
///
/// .. jupyter-execute::
///
///   import retworkx.generators
///   from retworkx.visualization import mpl_draw
///
///   graph = retworkx.generators.hexagonal_lattice_graph(2, 2)
///   mpl_draw(graph)
///
pub fn new(rows: usize, cols: usize, bidirectional: bool) -> GraphResult {
  let mut graph = StableGraph::<usize, usize>::new();

  let mut rowlen = rows;
  let mut collen = cols;

  // Needs two times the number of nodes vertically
  rowlen = 2 * rowlen + 2;
  collen += 1;
  let num_nodes = rowlen * collen - 2;

  let nodes: Vec<NodeIndex> = (0..num_nodes).map(|w| graph.add_node(w)).collect();

  // Add column edges
  // first column
  for j in 0..(rowlen - 2) {
    graph.add_edge(nodes[j], nodes[j + 1], 0);
    if bidirectional {
      graph.add_edge(nodes[j + 1], nodes[j], 0);
    }
  }

  for i in 1..(collen - 1) {
    for j in 0..(rowlen - 1) {
      graph.add_edge(nodes[i * rowlen + j - 1], nodes[i * rowlen + j], 0);
      if bidirectional {
        graph.add_edge(nodes[i * rowlen + j], nodes[i * rowlen + j - 1], 0);
      }
    }
  }

  // last column
  for j in 0..(rowlen - 2) {
    graph.add_edge(nodes[(collen - 1) * rowlen + j - 1], nodes[(collen - 1) * rowlen + j], 0);
    if bidirectional {
      graph.add_edge(nodes[(collen - 1) * rowlen + j], nodes[(collen - 1) * rowlen + j - 1], 0);
    }
  }

  // Add row edges
  for j in (0..(rowlen - 1)).step_by(2) {
    graph.add_edge(nodes[j], nodes[j + rowlen - 1], 0);
    if bidirectional {
      graph.add_edge(nodes[j + rowlen - 1], nodes[j], 0);
    }
  }

  for i in 1..(collen - 2) {
    for j in 0..rowlen {
      if i % 2 == j % 2 {
        graph.add_edge(nodes[i * rowlen + j - 1], nodes[(i + 1) * rowlen + j - 1], 0);
        if bidirectional {
          graph.add_edge(nodes[(i + 1) * rowlen + j - 1], nodes[i * rowlen + j - 1], 0);
        }
      }
    }
  }

  if collen > 2 {
    for j in ((collen % 2)..rowlen).step_by(2) {
      graph.add_edge(
        nodes[(collen - 2) * rowlen + j - 1],
        nodes[(collen - 1) * rowlen + j - 1 - (collen % 2)],
        0,
      );
      if bidirectional {
        graph.add_edge(
          nodes[(collen - 1) * rowlen + j - 1 - (collen % 2)],
          nodes[(collen - 2) * rowlen + j - 1],
          0,
        );
      }
    }
  }

  // Reset node weights to be sequentially heavier
  for (i, nw) in graph.node_weights_mut().enumerate() {
    *nw = i + 1;
  }

  (graph, nodes)
}
