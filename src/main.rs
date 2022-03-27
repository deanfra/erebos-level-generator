mod common;
mod crawler;
mod debug;
mod graph;
mod map;
use graph::MapGraph;
use map::room_templates;
use std::time::Instant;

// #[derive(Default)]
pub struct Config {
  width: i32,
  height: i32,
}

fn main() {
  let config = Config { width: 100, height: 100 };

  let time_benchmark = Instant::now();
  let map_graph = graph::random_graph();
  let map = generate_map(&map_graph, config);
  // ---------- debug ------------
  // debug::print_er_diagram(&map_graph.graph, &map_graph.nodes);
  // debug::print_plantuml_map(&map_graph.graph, &map);
  // debug::print_plantuml_nodes(&map_graph.graph, &map_graph.nodes);
  debug::print_map(map.tiles, map.width);
  // debug::print_map_history(map.history, map.width);

  // ---------- benchmark ------------
  let elapsed = time_benchmark.elapsed();
  println!("{}/{} rooms generated in: {:.2?}", map.rooms.len(), map_graph.nodes.len(), elapsed);
}

pub fn generate_map(map_graph: &MapGraph, config: Config) -> map::Map {
  let mut map = map::Map::new(config.width, config.height);
  let mut rng = rand::thread_rng();
  let mut templates = room_templates::RoomTemplates::new();

  for node in map_graph.nodes.iter() {
    let mut chain = Vec::from([node.clone()]);
    crawler::try_node_recursive(node, &map_graph, &mut map, &mut templates, &mut chain, &mut rng);
  }
  map
}
