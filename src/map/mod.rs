use super::{
  common::{xy_idx, XY},
  debug,
};
pub mod room;
use room::{DoorsXY, Room, RoomType};
pub mod room_template_definitions;
use room_templates::RoomTemplates;
pub mod room_templates;
use std::collections::HashMap;

const DEBUG: bool = false;

pub struct Map {
  pub tiles: Vec<u8>,
  pub history: Vec<Vec<u8>>,
  pub rooms: HashMap<usize, Room>,
  pub width: i32,
  pub height: i32,
}

impl Map {
  pub fn new(width: i32, height: i32) -> Map {
    let tilecount = (width * height) as usize;

    Map {
      width,
      height,
      rooms: HashMap::from([]),
      tiles: vec![0; tilecount],
      history: vec![],
    }
  }
}

pub fn add_or_update_room(map: &mut Map, node_idx: usize, room: Room) {
  // start at x,y in the map
  let is_new_room = !map.rooms.contains_key(&node_idx);
  let mut idx = xy_idx(room.x, room.y, map.width);
  let mut x = 1;

  for tile in room.tiles.iter() {
    // if this tile overlaps
    if is_new_room && *tile != 0 && idx <= map.tiles.len() && map.tiles[idx] == 8 {
      map.tiles[idx] = 9; // 9 = clash
    } else if *tile != 0 && idx <= map.tiles.len() {
      map.tiles[idx] = *tile;
    }

    // end of the row, move down one and back to the left
    if x == room.template.w {
      x = 1;
      idx += (map.width - room.template.w + 1) as usize;
    } else {
      idx += 1;
      x += 1;
    }
  }

  // println!("Add room: node #{}", node_idx);
  // debug::print_map(map.tiles.clone(), map.width);
  map.history.push(map.tiles.clone());
  map.rooms.insert(node_idx, room);
}

pub fn can_place_room(map: &Map, room: &mut Room, door_type: u8) -> bool {
  // start at x,y in the map
  let mut idx = xy_idx(room.x, room.y, map.width);
  let mut x = 1;
  let mut can_place = true;
  let mut test_area = map.tiles.clone();

  for tile in room.tiles.iter() {
    let in_range = idx < map.tiles.len();
    let is_tile = *tile != 0;
    let tile_overlaps_another = in_range && is_tile && map.tiles[idx] != 0;
    let crosses_side_of_map = room_crosses_side_of_map(idx, *tile, door_type, map.width as usize, map.tiles.len());

    let has_conflicts = !in_range || tile_overlaps_another || crosses_side_of_map;

    // if this tile overlaps
    if has_conflicts {
      // Debug: print every tile conflict with a red mark + reason
      if DEBUG {
        let reason = match (!in_range, tile_overlaps_another, crosses_side_of_map) {
          (true, false, false) => "Out of range".to_string(),
          (false, true, false) => format!("Tile overlaps another: {} {}", *tile, map.tiles[idx]),
          (false, false, true) => "Room crosses the map edge".to_string(),
          _ => "".to_string(),
        };

        test_area[idx] = 9;
        println!("CONFLICT - i:{}, reason:{}", idx, reason);
        debug::print_map(test_area.clone(), map.width);
      }
      // end debug

      can_place = false;
      break;
    } else {
      test_area[idx] = *tile;
    }

    // end of the row, move down one and back to the left
    if x == room.template.w {
      x = 1;
      idx += (map.width - room.template.w + 1) as usize;
    } else {
      idx += 1;
      x += 1;
    }
  }

  if can_place && DEBUG {
    debug::print_map(test_area.clone(), map.width);
  }

  can_place
}

/// Check if the door or the background crosses the edge of the map
fn room_crosses_side_of_map(idx: usize, tile: u8, door_type: u8, width: usize, len: usize) -> bool {
  let mut sides = vec![];

  // 2 - north door
  // 3 - east door
  // 4 - south door
  // 5 - west door

  // east
  if (idx + 1) % width == 0 {
    sides.push(3);
  // west
  } else if (idx + 1) % width == 1 {
    sides.push(5);
  }

  // north
  if (idx + 1) < width {
    sides.push(2);
  };

  // south
  if (idx + 1) > len - width {
    sides.push(4);
  };

  if sides.len() > 0 {
    // Intended door crosses the map edge
    if sides.contains(&door_type) {
      return true;
    }
    // Room crosses the map edsge
    if tile == 8 {
      return true;
    }
    false
  } else {
    false
  }
}

pub fn find_or_create_start_room(map: &mut Map, templates: &mut RoomTemplates, idx: &usize) -> Room {
  let start_room_templates = templates.of_type(RoomType::Start);
  let (_, start_room_template) = start_room_templates.get(0).unwrap().clone();
  let start_room = Room::new(start_room_template.clone());

  let found_room = map.rooms.get(idx).unwrap_or(&start_room);
  found_room.clone()
}
