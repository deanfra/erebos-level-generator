mod common;
mod crawler;
mod debug;
mod graph;
mod map;
use map::room_templates;
use std::time::Instant;

fn main() {
  let time_benchmark = Instant::now();
  let map_graph = graph::random_graph();
  let mut map = map::Map::new(100, 100);
  let mut rng = rand::thread_rng();
  let mut templates = room_templates::RoomTemplates::new();
  let first_node = map_graph.nodes.get(0).unwrap();

  crawler::try_node_recursive(first_node, &map_graph, &mut map, &mut templates, &mut rng);

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
