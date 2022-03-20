mod graph;
use graph::{MapGraph, NeighbourMap};
mod debug;
mod map;
use map::{
  room::{RoomType, Rooms},
  room_templates::get_template,
  Map,
};
mod common;
use petgraph::matrix_graph::NodeIndex;
use rand::prelude::{SliceRandom, ThreadRng};
use std::time::Instant;

// TODO: Weighted room randomness
// https://docs.rs/rand/0.5.0/rand/distributions/struct.Weighted.html
fn main() {
  let time_benchmark = Instant::now();
  let map_graph = graph::random_graph();
  let mut map = map::Map::new(100, 100);
  let mut rng = rand::thread_rng();
  let mut templates = Rooms::new();
  let first_node = map_graph.nodes.get(0).unwrap();
  let neighbour_map = graph::create_neighbour_map(&map_graph);

  try_node_recursive(first_node, &map_graph, &neighbour_map, &mut map, &mut templates, &mut rng);

  // debug
  // debug::print_er_diagram(&map_graph.graph, &map_graph.nodes);
  // debug::print_plantuml_map(&map_graph.graph, &map);
  // debug::print_plantuml_nodes(&map_graph.graph, &map_graph.nodes);
  debug::print_map(map.tiles, map.width);
  // debug::print_map_history(map.history, map.width);

  // benchmark
  let elapsed = time_benchmark.elapsed();
  println!("{}/{} rooms generated in: {:.2?}", map.rooms.len(), map_graph.nodes.len(), elapsed);
}

// Should live on Map
fn try_node_recursive(
  base_node: &NodeIndex<u32>,
  map_graph: &MapGraph,
  neighbour_map: &NeighbourMap,
  map: &mut Map,
  templates: &mut Rooms,
  rng: &mut ThreadRng,
) {
  let MapGraph { graph, nodes } = map_graph;
  let node_a_idx = base_node.index();

  // Should only create on first node
  let mut room_a = map::find_or_create_start_room(map, templates, &node_a_idx);
  let neighbours = neighbour_map.get(&base_node.index()).unwrap();

  let weights = graph.node_weights().collect::<Vec<&usize>>();
  let weight = weights.get(node_a_idx).unwrap();

  // Place first room in the middle of the map
  if nodes.first().unwrap().index() == node_a_idx {
    room_a.x = (map.width / 2) - (room_a.w / 2);
    room_a.y = (map.height / 2) - (room_a.h / 2);
    map.add_or_update_room(node_a_idx, room_a.clone());
  }

  // For each node
  for (neighbour_1, neighbour_2) in neighbours {
    // Neighbours can go both ways
    let a_to_b = neighbour_1.index() == node_a_idx;
    let node_b = if a_to_b { neighbour_2 } else { neighbour_1 };
    let node_b_idx = &node_b.index();

    // Stop at an existing node (TODO: try connect rooms?)
    let existing_node = map.rooms.contains_key(node_b_idx);
    let mut room_added = false;
    let mut template_idxs: Vec<usize> = room_a.valid_combinations.keys().cloned().collect();

    // randomise templates
    template_idxs.shuffle(rng);

    // Loop through randomised rooms until we find one that can fit into the map
    while !template_idxs.is_empty() && !room_added && !existing_node {
      let template_b_idx = template_idxs.pop().unwrap();

      // Each room has a precalculated coordinates
      let mut combination_coords = room_a.valid_combinations.get(&template_b_idx).unwrap().clone();
      combination_coords.shuffle(rng);

      for ((room_b_x, room_b_y), door_a_type, door_a_xy, door_b_type, door_b_xy) in combination_coords {
        // Get the B template of this room combination
        let mut room_b = get_template(&template_b_idx, templates);

        // Filter out rooms that are designed to have minimum doors (eg: T shaped rooms = 3 minimum doors)
        // NOTE: this has no guarantee, just a selection bias
        let has_min_doors = room_b.min_doors <= (graph.edges(*node_b).count() + 1) as u32;

        // Select Boss room based on node weight
        let node_b_weight = weights.get(*node_b_idx).unwrap();
        let room_is_correct_weight = if node_b_weight == weights.last().unwrap() {
          room_b.room_type == RoomType::Boss
        } else {
          room_b.room_type == RoomType::Normal
        };

        // 4 way offset
        let (x_offset, y_offset) = match door_b_type {
          2 => (0, -1),
          3 => (1, 0),
          4 => (0, 1),
          5 => (-1, 0),
          _ => (0, 0),
        };

        // move room b to a valid connecting position
        room_b.x = room_a.x - (room_b_x + x_offset);
        room_b.y = room_a.y - (room_b_y + y_offset);

        // Try and see if this valid position can fit on the map
        if room_is_correct_weight && has_min_doors && map.can_place_room(&mut room_b, door_b_type) {
          // Add door references to room
          // TODO: Traverse all existing rooms and attempt to optimise for more connections and more rooms?
          room_a.add_door(node_a_idx, *node_b_idx, door_a_type, door_a_xy, a_to_b);
          room_b.add_door(*node_b_idx, node_a_idx, door_b_type, door_b_xy, !a_to_b);

          // update rooms with the new door
          map.add_or_update_room(node_b_idx.clone(), room_b.clone());
          map.add_or_update_room(node_a_idx, room_a.clone());
          room_added = true;
          break;
        }
      }
    }

    // if this random room can be placed, try this room's connecting nodes
    if room_added {
      try_node_recursive(&node_b, map_graph, neighbour_map, map, templates, rng);
    }
  }
}
