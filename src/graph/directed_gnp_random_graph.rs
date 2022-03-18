use super::MapGraph;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use rand::distributions::{Distribution, Uniform};

/// Return a :math:`G_{np}` directed random graph, also known as an
/// Erdős-Rényi graph or a binomial graph.
///
/// For number of nodes :math:`n` and probability :math:`p`, the :math:`G_{n,p}`
/// graph algorithm creates :math:`n` nodes, and for all the :math:`n (n - 1)` possible edges,
/// each edge is created independently with probability :math:`p`.
/// In general, for any probability :math:`p`, the expected number of edges returned
/// is :math:`m = p n (n - 1)`. If :math:`p = 0` or :math:`p = 1`, the returned
/// graph is not random and will always be an empty or a complete graph respectively.
/// An empty graph has zero edges and a complete directed graph has :math:`n (n - 1)` edges.
/// The run time is :math:`O(n + m)` where :math:`m` is the expected number of edges mentioned above.
/// When :math:`p = 0`, run time always reduces to :math:`O(n)`, as the lower bound.
/// When :math:`p = 1`, run time always goes to :math:`O(n + n (n - 1))`, as the upper bound.
/// For other probabilities, this algorithm [1]_ runs in :math:`O(n + m)` time.
///
/// For :math:`0 < p < 1`, the algorithm is based on the implementation of the networkx function
/// ``fast_gnp_random_graph`` [2]_
///
/// :param int num_nodes: The number of nodes to create in the graph
/// :param float probability: The probability of creating an edge between two nodes
/// :param int seed: An optional seed to use for the random number generator
///
/// :return: A PyDiGraph object
/// :rtype: PyDiGraph
///
/// .. [1] Vladimir Batagelj and Ulrik Brandes,
///    "Efficient generation of large random networks",
///    Phys. Rev. E, 71, 036113, 2005.
/// .. [2] https://github.com/networkx/networkx/blob/networkx-2.4/networkx/generators/random_graphs.py#L49-L120
pub fn new(num_nodes: isize, probability: f64) -> MapGraph {
  if num_nodes <= 0 {
    println!("num_nodes must be > 0");
  }
  // let mut rng: Pcg64 = match seed {
  //   Some(seed) => Pcg64::seed_from_u64(seed),
  //   None => Pcg64::from_entropy(),
  // };

  let mut rng = rand::thread_rng();
  let mut nodes: Vec<NodeIndex> = Vec::new();
  let mut inner_graph = StableGraph::<usize, usize>::default();

  for x in 0..num_nodes {
    let node = inner_graph.add_node(x as usize);
    nodes.push(node);
  }

  if !(0.0..=1.0).contains(&probability) {
    println!("Probability out of range, must be 0 <= p <= 1");
  }

  if probability > 0.0 {
    if (probability - 1.0).abs() < std::f64::EPSILON {
      for u in 0..num_nodes {
        for v in 0..num_nodes {
          if u != v {
            // exclude self-loops
            let u_index = NodeIndex::new(u as usize);
            let v_index = NodeIndex::new(v as usize);
            inner_graph.add_edge(u_index, v_index, 0);
          }
        }
      }
    } else {
      let mut v: isize = 0;
      let mut w: isize = -1;
      let lp: f64 = (1.0 - probability).ln();

      let between = Uniform::new(0.0, 1.0);
      while v < num_nodes {
        let random: f64 = between.sample(&mut rng);
        let lr: f64 = (1.0 - random).ln();
        let ratio: isize = (lr / lp) as isize;
        w = w + 1 + ratio;
        // avoid self loops
        if v == w {
          w += 1;
        }
        while v < num_nodes && num_nodes <= w {
          w -= v;
          v += 1;
          // avoid self loops
          if v == w {
            w -= v;
            v += 1;
          }
        }
        if v < num_nodes {
          let v_index = NodeIndex::new(v as usize);
          let w_index = NodeIndex::new(w as usize);
          inner_graph.add_edge(v_index, w_index, 0);
        }
      }
    }
  }

  // Reset node weights to be sequentially heavier
  for (i, nw) in inner_graph.node_weights_mut().enumerate() {
    *nw = i + 1;
  }

  MapGraph { graph: inner_graph, nodes }
}
