use super::MapGraph;
use petgraph::stable_graph::{NodeIndex, StableGraph};

/// Generate a directed heavy hex graph. Fig. 2 of
/// https://arxiv.org/abs/1907.09528
/// An ASCII diagram of the graph is given by:
///
/// .. code-block:: text
///
///     ... D-S-D   D ...
///         |   |   |
///     ...-F   F-S-F ...
///         |   |   |
///     ... D   D   D ...
///         |   |   |
///     ... F-S-F   F-...
///         |   |   |
///         .........
///         |   |   |
///     ... D   D   D ...
///         |   |   |
///     ...-F   F-S-F ...
///         |   |   |
///     ... D   D   D ...
///         |   |   |
///     ... F-S-F   F-...
///         |   |   |
///         .........
///         |   |   |
///     ... D   D   D ...
///         |   |   |
///     ...-F   F-S-F ...
///         |   |   |
///     ... D   D   D ...
///         |   |   |
///     ... F-S-F   F-...
///         |   |   |
///     ... D   D-S-D ...
///
///
/// :param int d: distance of the code. If ``d`` is set to ``1`` a
///     :class:`~retworkx.PyDiGraph` with a single node will be returned.
/// :param bool multigraph: When set to False the output
///     :class:`~retworkx.PyGraph` object will not be not be a multigraph and
///     won't  allow parallel edges to be added. Instead
///     calls which would create a parallel edge will update the existing edge.
///
/// :returns: The generated heavy hex directed graph
/// :rtype: PyDiGraph
/// :raises IndexError: If d is even.
///
/// .. jupyter-execute::
///
///   import os
///   import tempfile
///
///   import pydot
///   from PIL import Image
///
///   import retworkx.generators
///
///   graph = retworkx.generators.heavy_hex_graph(3)
///   dot_str = graph.to_dot(
///       lambda node: dict(
///           color='black', fillcolor='lightblue', style='filled'))
///   dot = pydot.graph_from_dot_data(dot_str)[0]
///
///   with tempfile.TemporaryDirectory() as tmpdirname:
///       tmp_path = os.path.join(tmpdirname, 'dag.png')
///       dot.write_png(tmp_path)
///       image = Image.open(tmp_path)
///       os.remove(tmp_path)
///   image
///

pub fn new(distance: usize, bidirectional: bool) -> MapGraph {
  let mut graph = StableGraph::<usize, usize>::default();

  if distance % 2 == 0 {
    println!("Error: d must be odd");
  }

  if distance == 1 {
    let node = graph.add_node(0);
    let nodes = Vec::from([node]);
    return MapGraph { graph, nodes };
  }

  let num_data = distance * distance;
  let num_syndrome = (distance - 1) * (distance + 1) / 2;
  let num_flag = distance * (distance - 1);

  let nodes_data: Vec<NodeIndex> = (0..num_data).map(|w| graph.add_node(w * 2)).collect();
  let nodes_syndrome: Vec<NodeIndex> = (0..num_syndrome).map(|_| graph.add_node(0)).collect();
  let nodes_flag: Vec<NodeIndex> = (0..num_flag).map(|w| graph.add_node(w + 1 * 2)).collect();

  // connect data and flags
  for (i, flag_chunk) in nodes_flag.chunks(distance - 1).enumerate() {
    for (j, flag) in flag_chunk.iter().enumerate() {
      graph.add_edge(nodes_data[i * distance + j], *flag, 0);
      graph.add_edge(nodes_data[i * distance + j + 1], *flag, 0);
      if bidirectional {
        graph.add_edge(*flag, nodes_data[i * distance + j], 0);
        graph.add_edge(*flag, nodes_data[i * distance + j + 1], 0);
      }
    }
  }

  // connect data and syndromes
  for (i, syndrome_chunk) in nodes_syndrome.chunks((distance + 1) / 2).enumerate() {
    if i % 2 == 0 {
      graph.add_edge(nodes_data[i * distance], syndrome_chunk[0], 0);
      graph.add_edge(nodes_data[(i + 1) * distance], syndrome_chunk[0], 0);
      if bidirectional {
        graph.add_edge(syndrome_chunk[0], nodes_data[i * distance], 0);
        graph.add_edge(syndrome_chunk[0], nodes_data[(i + 1) * distance], 0);
      }
    } else if i % 2 == 1 {
      let edge_1 = i * distance + (distance - 1);
      let edge_2 = i * distance + (2 * distance - 1);
      if edge_1 < nodes_data.len() && edge_2 < nodes_data.len() {
        graph.add_edge(nodes_data[edge_1], syndrome_chunk[syndrome_chunk.len() - 1], 0);
        graph.add_edge(nodes_data[edge_2], syndrome_chunk[syndrome_chunk.len() - 1], 0);
      };

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
    }
  }

  // connect flag and syndromes
  for (i, syndrome_chunk) in nodes_syndrome.chunks((distance + 1) / 2).enumerate() {
    if i % 2 == 0 {
      for (j, syndrome) in syndrome_chunk.iter().enumerate() {
        if j != 0 {
          let edge_1 = i * (distance - 1) + 2 * (j - 1) + 1;
          let edge_2 = (i + 1) * (distance - 1) + 2 * (j - 1) + 1;

          if edge_1 < nodes_flag.len() && edge_2 < nodes_flag.len() {
            graph.add_edge(*syndrome, nodes_flag[edge_1], 0);
            graph.add_edge(*syndrome, nodes_flag[edge_2], 0);
            if bidirectional {
              graph.add_edge(nodes_flag[edge_1], *syndrome, 0);
              graph.add_edge(nodes_flag[edge_2], *syndrome, 0);
            }
          };
        }
      }
    } else if i % 2 == 1 {
      for (j, syndrome) in syndrome_chunk.iter().enumerate() {
        if j != syndrome_chunk.len() - 1 {
          let edge_1 = i * (distance - 1) + 2 * j;
          let edge_2 = (i + 1) * (distance - 1) + 2 * j;

          if edge_1 < nodes_flag.len() && edge_2 < nodes_flag.len() {
            graph.add_edge(*syndrome, nodes_flag[edge_1], 0);
            graph.add_edge(*syndrome, nodes_flag[edge_2], 0);
            if bidirectional {
              graph.add_edge(nodes_flag[edge_1], *syndrome, 0);
              graph.add_edge(nodes_flag[edge_2], *syndrome, 0);
            }
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

  MapGraph { graph, nodes }
}
