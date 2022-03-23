use super::{room_templates::RoomTemplate, DoorsXY, RoomType};
use std::collections::HashMap;

/// Iterate through each tile and push the XY coords if its a door (2,2,2,5)
fn calculate_doors_xy(tiles: &Vec<u8>, width: i32) -> DoorsXY {
  let door_tiles: Vec<u8> = vec![2, 3, 4, 5];
  let mut doors_xy: DoorsXY = HashMap::new();
  let mut x = 1;
  let mut y = 1;

  for tile in tiles {
    // If this tile is a door
    if door_tiles.contains(tile) {
      if let Some(doors) = doors_xy.get_mut(tile) {
        doors.push((x, y));
      } else {
        doors_xy.insert(*tile, Vec::from([(x, y)]));
      }
    }

    // end of the row, move down one and back to the left
    if x == width {
      x = 1;
      y += 1;
    } else {
      x += 1;
    }
  }

  doors_xy
}

/// Takes any door reference on a room layout and converts it to a door number
fn calculate_door_tiles(tiles: Vec<u8>, width: i32) -> Vec<u8> {
  // 2 - possible north door
  // 3 - possible east door
  // 4 - possible south door
  // 5 - possible west door

  let mut tiles_with_doors: Vec<u8> = vec![];

  for (i, tile) in tiles.iter().enumerate() {
    let top_idx = i as i32 - width;
    let bottom_idx = i as i32 + width;
    let right_idx = i as i32 + 1;
    let left_idx = i as i32 - 1;

    if *tile == 2 {
      // North
      if !in_range(&tiles, top_idx) || tiles[top_idx as usize] == 0 {
        tiles_with_doors.push(2);
      // South
      } else if !in_range(&tiles, bottom_idx) || tiles[bottom_idx as usize] == 0 {
        tiles_with_doors.push(4);
      // West
      } else if !in_range(&tiles, left_idx) || ((i as i32) % width == 0) || tiles[left_idx as usize] == 0 {
        tiles_with_doors.push(5);
      // East
      } else if !in_range(&tiles, right_idx) || ((i as i32 + 1) % width == 0) || tiles[right_idx as usize] == 0 {
        tiles_with_doors.push(3);
      } else {
        tiles_with_doors.push(9);
      }
    } else {
      tiles_with_doors.push(*tile);
    }
  }
  tiles_with_doors
}

fn in_range<T>(arr: &Vec<T>, idx: i32) -> bool {
  idx >= 0 && idx < arr.len() as i32
}

/***
 * Template definitions
 **/

// 0 - exterior space
// 1 - wall
// 2 - door
// 8 - interior space

pub fn small_square() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 1, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    1, 2, 2, 2, 1,
  ];

  let width: i32 = 5;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn rectangle() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 2, 1, //
    2, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 2, //
    1, 2, 2, 2, 2, 1,
  ];

  let width: i32 = 6;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn big_square() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 1, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    1, 2, 2, 2, 1,
  ];

  let width: i32 = 5;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn bent_l() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 1, 0, //
    2, 8, 8, 8, 2, 0, //
    2, 8, 8, 8, 2, 0, //
    1, 1, 8, 8, 1, 1, //
    0, 2, 8, 8, 8, 2, //
    0, 2, 8, 8, 8, 2, //
    0, 1, 2, 2, 2, 1,
  ];

  let width: i32 = 6;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn bent_r() -> RoomTemplate {
  let template = vec![
    0, 1, 2, 2, 2, 1, //
    0, 2, 8, 8, 8, 2, //
    0, 2, 8, 8, 8, 2, //
    1, 1, 8, 8, 1, 1, //
    2, 8, 8, 8, 2, 0, //
    2, 8, 8, 8, 2, 0, //
    1, 2, 2, 2, 1, 0,
  ];

  let width: i32 = 6;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn l_top_left_shape() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 2, 2, 1, //
    2, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 2, //
    1, 2, 2, 1, 8, 8, 2, //
    0, 0, 0, 2, 8, 8, 2, //
    0, 0, 0, 2, 8, 8, 2, //
    0, 0, 0, 1, 2, 2, 1,
  ];

  let width: i32 = 7;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}
pub fn l_top_right_shape() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 2, 2, 1, //
    2, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 1, 2, 2, 1, //
    2, 8, 8, 2, 0, 0, 0, //
    2, 8, 8, 2, 0, 0, 0, //
    1, 2, 2, 1, 0, 0, 0,
  ];

  let width: i32 = 7;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn l_bottom_right_shape() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 1, 0, 0, 0, //
    2, 8, 8, 2, 0, 0, 0, //
    2, 8, 8, 2, 0, 0, 0, //
    2, 8, 8, 1, 2, 2, 1, //
    2, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 2, //
    1, 2, 2, 2, 2, 2, 1,
  ];

  let width: i32 = 7;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}
pub fn l_bottom_left_shape() -> RoomTemplate {
  let template = vec![
    0, 0, 0, 1, 2, 2, 1, //
    0, 0, 0, 2, 8, 8, 2, //
    0, 0, 0, 2, 8, 8, 2, //
    1, 2, 2, 1, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 2, //
    1, 2, 2, 2, 2, 2, 1,
  ];

  let width: i32 = 7;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn long_shape() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 2, 2, 2, 2, 1, //
    2, 8, 8, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 8, 8, 2, //
    1, 2, 2, 2, 2, 2, 2, 2, 1,
  ];

  let width: i32 = 9;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn cross() -> RoomTemplate {
  let template = vec![
    0, 1, 2, 1, 0, //
    1, 1, 8, 1, 1, //
    2, 8, 8, 8, 2, //
    1, 1, 8, 1, 1, //
    0, 1, 2, 1, 0, //
  ];

  let width: i32 = 5;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    min_doors: 2,
    ..Default::default()
  }
}

pub fn tall() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 1, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    2, 8, 8, 8, 2, //
    1, 2, 2, 2, 1,
  ];

  let width: i32 = 5;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn wide() -> RoomTemplate {
  let template = vec![
    1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, //
    2, 8, 8, 8, 8, 8, 8, 8, 8, 8, 2, //
    2, 8, 8, 8, 8, 8, 8, 8, 8, 8, 2, //
    1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1,
  ];

  let width: i32 = 11;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn jar() -> RoomTemplate {
  let template = vec![
    0, 1, 1, 2, 1, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    1, 1, 8, 8, 8, 1, 1, //
    2, 8, 8, 8, 8, 8, 2, //
    1, 1, 8, 8, 8, 1, 1, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 1, 2, 1, 1, 0,
  ];

  let width: i32 = 7;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    min_doors: 2,
    ..Default::default()
  }
}

pub fn start_room() -> RoomTemplate {
  let template = vec![
    0, 1, 2, 1, 2, 1, 2, 1, 0, //
    0, 2, 8, 8, 8, 8, 8, 2, 0, //
    1, 1, 1, 8, 8, 8, 1, 1, 1, //
    2, 8, 8, 8, 8, 8, 8, 8, 2, //
    1, 1, 2, 1, 1, 1, 2, 1, 1,
  ];
  let width: i32 = 9;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    room_type: RoomType::Start,
    ..Default::default()
  }
}

pub fn boss_room() -> RoomTemplate {
  let template = vec![
    0, 1, 2, 1, 2, 1, 2, 1, 0, //
    1, 1, 8, 1, 8, 1, 8, 1, 1, //
    2, 8, 8, 8, 8, 8, 8, 8, 2, //
    1, 1, 8, 8, 8, 8, 8, 1, 1, //
    2, 8, 8, 8, 8, 8, 8, 8, 2, //
    1, 2, 2, 1, 1, 1, 2, 2, 1,
  ];

  let width: i32 = 9;
  let tiles = calculate_door_tiles(template, width);
  let doors_xy = calculate_doors_xy(&tiles, width);

  RoomTemplate {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    room_type: RoomType::Boss,
    ..Default::default()
  }
}
