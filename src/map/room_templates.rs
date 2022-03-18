use super::{DoorsXY, Room, RoomType, Rooms};
use std::collections::HashMap;

/// Iterate through each tile and push the XY coords if its a door (2,3,4,5)
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

/***
 * Template definitions
 **/

// 0 - space
// 1 - wall
// 2 - possible north door
// 3 - possible east door
// 4 - possible south door
// 5 - possible west door

pub fn small_square() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 1, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    1, 4, 4, 4, 1,
  ];

  let width: i32 = 5;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn rectangle() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 2, 1, //
    5, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 3, //
    1, 4, 4, 4, 4, 1,
  ];

  let width: i32 = 6;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn big_square() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 1, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    1, 4, 4, 4, 1,
  ];

  let width: i32 = 5;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn bent_l() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 1, 0, //
    5, 8, 8, 8, 3, 0, //
    5, 8, 8, 8, 3, 0, //
    1, 1, 8, 8, 1, 1, //
    0, 5, 8, 8, 8, 3, //
    0, 5, 8, 8, 8, 3, //
    0, 1, 4, 4, 4, 1,
  ];

  let width: i32 = 6;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn bent_r() -> Room {
  let tiles = vec![
    0, 1, 2, 2, 2, 1, //
    0, 5, 8, 8, 8, 3, //
    0, 5, 8, 8, 8, 3, //
    1, 1, 8, 8, 1, 1, //
    5, 8, 8, 8, 3, 0, //
    5, 8, 8, 8, 3, 0, //
    1, 4, 4, 4, 1, 0,
  ];

  let width: i32 = 6;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn l_top_left_shape() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 2, 2, 1, //
    5, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 3, //
    1, 4, 4, 1, 8, 8, 3, //
    0, 0, 0, 5, 8, 8, 3, //
    0, 0, 0, 5, 8, 8, 3, //
    0, 0, 0, 1, 4, 4, 1,
  ];

  let width: i32 = 7;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}
pub fn l_top_right_shape() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 2, 2, 1, //
    5, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 1, 4, 4, 1, //
    5, 8, 8, 3, 0, 0, 0, //
    5, 8, 8, 3, 0, 0, 0, //
    1, 4, 4, 1, 0, 0, 0,
  ];

  let width: i32 = 7;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn l_bottom_right_shape() -> Room {
  let tiles = vec![
    1, 2, 2, 1, 0, 0, 0, //
    5, 8, 8, 3, 0, 0, 0, //
    5, 8, 8, 3, 0, 0, 0, //
    5, 8, 8, 1, 2, 2, 1, //
    5, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 3, //
    1, 4, 4, 4, 4, 4, 1,
  ];

  let width: i32 = 7;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}
pub fn l_bottom_left_shape() -> Room {
  let tiles = vec![
    0, 0, 0, 1, 2, 2, 1, //
    0, 0, 0, 5, 8, 8, 3, //
    0, 0, 0, 5, 8, 8, 3, //
    1, 2, 2, 1, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 3, //
    1, 4, 4, 4, 4, 4, 1,
  ];

  let width: i32 = 7;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn long_shape() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 2, 2, 2, 2, 1, //
    5, 8, 8, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 8, 8, 3, //
    1, 4, 4, 4, 4, 4, 4, 4, 1,
  ];

  let width: i32 = 9;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn cross() -> Room {
  let tiles = vec![
    0, 1, 2, 1, 0, //
    1, 1, 8, 1, 1, //
    5, 8, 8, 8, 3, //
    1, 1, 8, 1, 1, //
    0, 1, 4, 1, 0, //
  ];

  let width: i32 = 5;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    min_doors: 3,
    ..Default::default()
  }
}

pub fn tall() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 1, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    5, 8, 8, 8, 3, //
    1, 4, 4, 4, 1,
  ];

  let width: i32 = 5;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn wide() -> Room {
  let tiles = vec![
    1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, //
    5, 8, 8, 8, 8, 8, 8, 8, 8, 8, 3, //
    5, 8, 8, 8, 8, 8, 8, 8, 8, 8, 3, //
    1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1,
  ];

  let width: i32 = 11;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    ..Default::default()
  }
}

pub fn jar() -> Room {
  let tiles = vec![
    0, 1, 1, 2, 1, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    1, 1, 8, 8, 8, 1, 1, //
    5, 8, 8, 8, 8, 8, 3, //
    1, 1, 8, 8, 8, 1, 1, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 8, 8, 8, 1, 0, //
    0, 1, 1, 4, 1, 1, 0,
  ];

  let width: i32 = 7;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    min_doors: 3,
    ..Default::default()
  }
}

pub fn start_room() -> Room {
  let tiles = vec![
    0, 1, 2, 1, 2, 1, 2, 1, 0, //
    0, 5, 8, 8, 8, 8, 8, 3, 0, //
    1, 1, 1, 8, 8, 8, 1, 1, 1, //
    5, 8, 8, 8, 8, 8, 8, 8, 3, //
    1, 1, 4, 1, 1, 1, 4, 1, 1,
  ];

  let width: i32 = 9;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    room_type: RoomType::Start,
    ..Default::default()
  }
}

pub fn boss_room() -> Room {
  let tiles = vec![
    0, 1, 2, 1, 2, 1, 2, 1, 0, //
    1, 1, 8, 1, 8, 1, 8, 1, 1, //
    5, 8, 8, 8, 8, 8, 8, 8, 3, //
    1, 1, 8, 8, 8, 8, 8, 1, 1, //
    5, 8, 8, 8, 8, 8, 8, 8, 3, //
    1, 4, 4, 1, 1, 1, 4, 4, 1,
  ];

  let width: i32 = 9;
  let doors_xy = calculate_doors_xy(&tiles, width);

  Room {
    h: tiles.len() as i32 / width,
    w: width,
    tiles,
    possible_doors_xy: doors_xy,
    room_type: RoomType::Boss,
    ..Default::default()
  }
}

pub fn get_template(idx: &usize, rooms: &mut Rooms) -> Room {
  rooms.rooms.get(idx).unwrap().clone()
}
