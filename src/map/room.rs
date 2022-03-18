use super::{room_templates, xy_idx, XY};
use rand::Rng;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Room {
  pub x: i32,
  pub y: i32,
  pub w: i32,
  pub h: i32,
  pub min_doors: u32,
  pub room_type: RoomType,
  /// The XY coords for each used door
  pub doors_xy: DoorsXY,
  /// The XY coords for each possible door
  pub possible_doors_xy: DoorsXY,
  pub tiles: Vec<u8>,
  pub valid_combinations: HashMap<usize, Vec<(XY, u8, XY, u8, XY)>>,
}

impl Room {
  pub fn add_door(&mut self, door_type: u8, xy: XY) {
    if let Some(doors) = self.doors_xy.get_mut(&door_type) {
      doors.push(xy);
    } else {
      self.doors_xy.insert(door_type, Vec::from([xy]));
    }

    let idx = xy_idx(xy.0 - 1, xy.1 - 1, self.w);
    // Assign a connected door index
    self.tiles[idx] = 7;
  }
}

#[derive(Clone, PartialEq)]
pub enum RoomType {
  Start,
  Normal,
  Boss,
}

impl Default for RoomType {
  fn default() -> RoomType {
    RoomType::Normal
  }
}

type RoomMap = HashMap<usize, Room>;
pub type DoorsXY = HashMap<u8, Vec<XY>>;
/// Params: Tiles with room places, x, y coordinates of the room
type TemporaryCanvas = (Vec<u8>, i32, i32);

pub struct Rooms {
  pub rooms: RoomMap,
}

impl Rooms {
  pub fn new() -> Rooms {
    let mut rooms: RoomMap = HashMap::new();

    for (num, template) in Vec::from([
      room_templates::bent_l(),
      room_templates::bent_r(),
      room_templates::big_square(),
      room_templates::boss_room(),
      room_templates::cross(),
      room_templates::jar(),
      room_templates::l_bottom_left_shape(),
      room_templates::l_bottom_right_shape(),
      room_templates::l_top_left_shape(),
      room_templates::l_top_right_shape(),
      room_templates::long_shape(),
      room_templates::rectangle(),
      room_templates::small_square(),
      room_templates::start_room(),
      room_templates::tall(),
      room_templates::wide(),
    ])
    .iter()
    .enumerate()
    {
      rooms.insert(num, template.clone());
    }

    calculate_combinations(&mut rooms);

    Rooms { rooms }
  }

  pub fn of_type(&self, room_type: RoomType) -> Vec<(&usize, &Room)> {
    self
      .rooms
      .iter()
      .filter(|(_, r)| r.room_type == room_type)
      .collect::<Vec<(&usize, &Room)>>()
      .clone()
  }

  pub fn random(&self) -> Room {
    let mut rng = rand::thread_rng();
    let rooms_len = self.rooms.len();
    let rand_room = rng.gen_range(1..rooms_len);
    self.rooms.get(&rand_room).unwrap().clone()
  }
}

/// precalculate valid XY positions of all door combinations, these get randomly used when crawling through the original level graph
fn calculate_combinations(rooms: &mut RoomMap) {
  let rooms_b = rooms.clone();
  // For each room
  for (_, room_a) in rooms.iter_mut() {
    // Try all rooms
    for (tb_i, room_b) in rooms_b.iter() {
      // For each door room A
      for (door_a_type, door_xys) in room_a.possible_doors_xy.iter() {
        // For each door position on each face
        for door_a_xy in door_xys {
          // Get the corresponding door (eg, left door for right door)
          let door_b_type = connecting_door(*door_a_type);
          // Try each corresponding door of room B
          if let Some(doors_xy) = room_b.possible_doors_xy.get(&door_b_type) {
            for door_b_xy in doors_xy {
              // Align both rooms and doors on a canvas and test their compatibility
              if let Some(room_b_xy) = can_place_room(door_a_xy, door_b_xy, room_a, room_b) {
                // println!("^ #{}/{} x:{} y:{}", ta_i, tb_i, room_b_x, room_b_y);
                let combination = (room_b_xy, *door_a_type, *door_a_xy, door_b_type, *door_b_xy);

                if let Some(combo) = room_a.valid_combinations.get_mut(tb_i) {
                  combo.push(combination);
                } else {
                  // Push the valid position combination of room a and room b (relative to a)
                  room_a.valid_combinations.insert(*tb_i, Vec::from([combination]));
                }
              }
            }
          };
        }
      }
    }
    // println!("{}: {:?}", ta_i, room_a.valid_combinations);
  }
}

/// Place the room into the centre of the canvas
fn canvas_with_room(room: &Room, canvas_w: i32, canvas_h: i32) -> TemporaryCanvas {
  let mut tiles = vec![0; (canvas_w * canvas_h) as usize];
  let x = (canvas_w / 2) - (room.w / 2);
  let y = (canvas_h / 2) - (room.h / 2);
  let mut idx = (y * canvas_w + x) as usize;
  let mut room_x = 1;

  // place each tile in the canvas area
  for t in &room.tiles {
    tiles[idx] = *t;

    // end of the row, move down one and back to the left
    if room_x == room.w {
      room_x = 1;
      idx += (canvas_w - room.w + 1) as usize;
    } else {
      idx += 1;
      room_x += 1;
    }
  }

  (tiles, x, y)
}

fn connecting_door(tile: u8) -> u8 {
  match tile {
    2 => 4,
    4 => 2,
    3 => 5,
    5 => 3,
    _ => 2,
  }
}

/// Refactor: This is basically the same function as map::try_room
fn can_place_room(xy_a: &XY, xy_b: &XY, room_a: &Room, room_b: &Room) -> Option<XY> {
  let canvas_width = room_a.w + (room_b.w * 2) + 1;
  let canvas_height = room_a.h + (room_b.h * 2) + 1;
  let (mut canvas_tiles, room_x, room_y) = canvas_with_room(&room_a, canvas_width, canvas_height);
  let mut can_place = true;

  // Get canvas position of door a
  let door_a_x = room_x + xy_a.0;
  let door_a_y = room_y + xy_a.1;

  // Get canvas position of room b, aligning door a and b together
  // TODO: offset by one
  let new_room_canvas_x = door_a_x - &xy_b.0;
  let new_room_canvas_y = door_a_y - &xy_b.1;

  // Get relative position of room b to room a
  let new_room_relative_x = room_x - new_room_canvas_x;
  let new_room_relative_y = room_y - new_room_canvas_y;

  // ---------- Duplicated map crawling logic
  // index of where to begin placing the new room
  let mut idx = (new_room_canvas_y * canvas_width + new_room_canvas_x) as usize;
  let mut room_x = 1;

  // check each tile in the canvas area
  for tile in &room_b.tiles {
    // if this tile overlaps
    // thread 'main' panicked at 'index out of bounds
    if *tile != 0 && canvas_tiles[idx] == 8 {
      canvas_tiles[idx] = 9; // 9 = clash
      can_place = false;
    } else if *tile != 0 {
      canvas_tiles[idx] = *tile;
    }

    // end of the row, move down one and back to the left
    if room_x == room_b.w {
      room_x = 1;
      idx += (canvas_width - room_b.w + 1) as usize;
    } else {
      idx += 1;
      room_x += 1;
    }
  }
  // -------- Duplicated map crawling logic

  if can_place {
    // debug::print_map(canvas_tiles.clone(), canvas_width);
    Some((new_room_relative_x, new_room_relative_y))
  } else {
    None
  }
}
