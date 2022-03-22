use super::GraphResult;
use petgraph::stable_graph::{NodeIndex, StableGraph};

/// Generate a directed heavy square graph. Fig. 6 of
/// https://arxiv.org/abs/1907.09528.
/// An ASCII diagram of the graph is given by:
///
/// .. code-block:: console
///
///     ...       S   ...
///        \     / \
///        ... D   D   D ...
///            |   |   |
///        ... F-S-F-S-F-...
///            |   |   |
///        ... D   D   D ...
///            |   |   |
///        ... F-S-F-S-F-...
///            |   |   |
///            .........
///            |   |   |
///        ... D   D   D ...
///             \ /     \
///        ...   S       ...
///
/// NOTE: This function generates the four-frequency variant of the heavy square code.
/// This function implements Fig 10.b left of the [paper](https://arxiv.org/abs/1907.09528).
/// This function doesn't support the variant Fig 10.b right.
///
/// :param int distance: distance of the code. If ``d`` is set to ``1`` a
///     :class:`~retworkx.PyDiGraph` with a single node will be returned.
/// :param bool multigraph: When set to False the output
///     :class:`~retworkx.PyDiGraph` object will not be not be a multigraph and
///     won't  allow parallel edges to be added. Instead
///     calls which would create a parallel edge will update the existing edge.
///
/// :returns: The generated directed heavy square graph
/// :raises IndexError: If d is even.
///
pub fn new(distance: usize, bidirectional: bool) -> GraphResult {
  let mut graph = StableGraph::<usize, usize>::default();

  if distance % 2 == 0 {
    println!("Error: d must be odd");
  }

  if distance == 1 {
    let node = graph.add_node(0);
    let nodes = Vec::from([node]);
    return (graph, nodes);
  }

  let num_data = distance * distance;
  let num_syndrome = distance * (distance - 1);
  let num_flag = distance * (distance - 1);

  let nodes_data: Vec<NodeIndex> = (0..num_data).map(|w| graph.add_node(w * 2)).collect();
  let nodes_syndrome: Vec<NodeIndex> = (0..num_syndrome).map(|_| graph.add_node(0)).collect();
  let nodes_flag: Vec<NodeIndex> = (0..num_flag).map(|w| graph.add_node(w + 1 * 2)).collect();

  // connect data and flags
  for (i, flag_chunk) in nodes_flag.chunks(distance - 1).enumerate() {
    for (j, flag) in flag_chunk.iter().enumerate() {
      graph.add_edge(nodes_data[i * distance + j], *flag, 0);
      graph.add_edge(*flag, nodes_data[i * distance + j + 1], 0);
      if bidirectional {
        graph.add_edge(*flag, nodes_data[i * distance + j], 0);
        graph.add_edge(nodes_data[i * distance + j + 1], *flag, 0);
      }
    }
  }

  // connect data and syndromes
  for (i, syndrome_chunk) in nodes_syndrome.chunks(distance).enumerate() {
    if i % 2 == 0 {
      graph.add_edge(
        nodes_data[i * distance + (distance - 1)],
        syndrome_chunk[syndrome_chunk.len() - 1],
        0,
      );
      graph.add_edge(
        nodes_data[i * distance + (2 * distance - 1)],
        syndrome_chunk[syndrome_chunk.len() - 1],
        0,
      );
      if bidirectional {
        graph.add_edge(
          syndrome_chunk[syndrome_chunk.len() - 1],
          nodes_data[i * distance + (distance - 1)],
          0,
        );
        graph.add_edge(
          syndrome_chunk[syndrome_chunk.len() - 1],
          nodes_data[i * distance + (2 * distance - 1)],
          0,
        );
      }
    } else if i % 2 == 1 {
      graph.add_edge(nodes_data[i * distance], syndrome_chunk[0], 0);
      graph.add_edge(nodes_data[(i + 1) * distance], syndrome_chunk[0], 0);
      if bidirectional {
        graph.add_edge(syndrome_chunk[0], nodes_data[i * distance], 0);
        graph.add_edge(syndrome_chunk[0], nodes_data[(i + 1) * distance], 0);
      }
    }
  }

  // connect flag and syndromes
  for (i, syndrome_chunk) in nodes_syndrome.chunks(distance).enumerate() {
    if i % 2 == 0 {
      for (j, syndrome) in syndrome_chunk.iter().enumerate() {
        if j != syndrome_chunk.len() - 1 {
          graph.add_edge(*syndrome, nodes_flag[i * (distance - 1) + j], 0);
          graph.add_edge(*syndrome, nodes_flag[(i + 1) * (distance - 1) + j], 0);
          if bidirectional {
            graph.add_edge(nodes_flag[i * (distance - 1) + j], *syndrome, 0);
            graph.add_edge(nodes_flag[(i + 1) * (distance - 1) + j], *syndrome, 0);
          }
        }
      }
    } else if i % 2 == 1 {
      for (j, syndrome) in syndrome_chunk.iter().enumerate() {
        if j != 0 {
          graph.add_edge(*syndrome, nodes_flag[i * (distance - 1) + j - 1], 0);
          graph.add_edge(*syndrome, nodes_flag[(i + 1) * (distance - 1) + j - 1], 0);
          if bidirectional {
            graph.add_edge(nodes_flag[i * (distance - 1) + j - 1], *syndrome, 0);
            graph.add_edge(nodes_flag[(i + 1) * (distance - 1) + j - 1], *syndrome, 0);
          }
        }
      }
    }
  }

  // errors with .concat(), marked as unstable?
  let mut nodes: Vec<NodeIndex> = nodes_data.clone();
  nodes_syndrome.iter().for_each(|ns| {
    nodes.push(*ns);
  });
  nodes_flag.iter().for_each(|nf| {
    nodes.push(*nf);
  });

  // Reset node weights to be sequentially heavier
  for (i, nw) in graph.node_weights_mut().enumerate() {
    *nw = i + 1;
  }

  (graph, nodes)
}
