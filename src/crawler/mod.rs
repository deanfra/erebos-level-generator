use super::{graph, map};
use map::{
  room::{Room, RoomType},
  room_templates::{get_template, RoomTemplates},
};
use petgraph::matrix_graph::NodeIndex;
use rand::prelude::{SliceRandom, ThreadRng};

pub fn try_node_recursive(
  base_node: &NodeIndex<u32>,
  map_graph: &graph::MapGraph,
  neighbour_map: &graph::NeighbourMap,
  map: &mut map::Map,
  templates: &mut RoomTemplates,
  rng: &mut ThreadRng,
) {
  let graph::MapGraph { graph, nodes } = map_graph;
  let node_a_idx = base_node.index();

  // Should only create on first node
  let mut room_a = map::find_or_create_start_room(map, templates, &node_a_idx);
  let neighbours = neighbour_map.get(&base_node.index()).unwrap();

  let weights = graph.node_weights().collect::<Vec<&usize>>();
  let _weight = weights.get(node_a_idx).unwrap();

  // Place first room in the middle of the map
  if nodes.first().unwrap().index() == node_a_idx {
    room_a.x = (map.width / 2) - (room_a.template.w / 2);
    room_a.y = (map.height / 2) - (room_a.template.h / 2);
    map::add_or_update_room(map, node_a_idx, room_a.clone());
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
    let mut template_idxs: Vec<usize> = room_a.template.valid_combinations.keys().cloned().collect();

    // randomise templates
    template_idxs.shuffle(rng);

    // Loop through randomised rooms until we find one that can fit into the map
    while !template_idxs.is_empty() && !room_added && !existing_node {
      let template_b_idx = template_idxs.pop().unwrap();

      // Each room has a precalculated coordinates
      let mut room_combinations = room_a.template.valid_combinations.get(&template_b_idx).unwrap().clone();
      room_combinations.shuffle(rng);

      for ((room_b_x, room_b_y), door_a_type, door_a_xy, door_b_type, door_b_xy) in room_combinations {
        // Get the B template of this room combination
        let template = get_template(&template_b_idx, templates);
        let mut room_b = Room::new(template);

        // Filter out rooms that are designed to have minimum doors (eg: T shaped rooms = 3 minimum doors)
        // NOTE: this has no guarantee, just a selection bias
        let has_min_doors = room_b.template.min_doors <= (graph.edges(*node_b).count() + 1) as u32;

        // Select Boss room based on node weight
        let node_b_weight = weights.get(*node_b_idx).unwrap();
        let room_is_correct_weight = if node_b_weight == weights.last().unwrap() {
          room_b.template.room_type == RoomType::Boss
        } else {
          room_b.template.room_type == RoomType::Normal
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
        if room_is_correct_weight && has_min_doors && map::can_place_room(map, &mut room_b, door_b_type) {
          // Add door references to room
          // TODO: Traverse all existing rooms and attempt to optimise for more connections and more rooms?
          room_a.add_door(node_a_idx, *node_b_idx, door_a_type, door_a_xy, a_to_b);
          room_b.add_door(*node_b_idx, node_a_idx, door_b_type, door_b_xy, !a_to_b);

          // update rooms with the new door
          map::add_or_update_room(map, node_b_idx.clone(), room_b.clone());
          map::add_or_update_room(map, node_a_idx, room_a.clone());
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
