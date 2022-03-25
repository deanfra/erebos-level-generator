use super::{room_template_definitions as definitions, DoorsXY, RoomType, XY};
use rand::Rng;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct RoomTemplate {
  /// The XY coords for each possible door
  pub h: i32,
  pub w: i32,
  pub min_doors: u32,
  pub possible_doors_xy: DoorsXY,
  pub room_type: RoomType,
  pub tiles: Vec<u8>,
  pub valid_combinations: HashMap<usize, Vec<RoomCombination>>,
}

///
pub type RoomCombination = (XY, u8, XY, u8, XY);
/// Hashmap of each room template
type RoomTemplateMap = HashMap<usize, RoomTemplate>;
/// Params: Tiles with room places, x, y coordinates of the room
type TemporaryCanvas = (Vec<u8>, i32, i32);

pub struct RoomTemplates {
  pub rooms: RoomTemplateMap,
}

impl RoomTemplates {
  pub fn new() -> RoomTemplates {
    let mut rooms: RoomTemplateMap = HashMap::new();

    for (num, template) in Vec::from([
      definitions::bent_l(),
      definitions::bent_r(),
      definitions::big_square(),
      definitions::boss_room(),
      definitions::cross(),
      definitions::jar(),
      definitions::l_bottom_left_shape(),
      definitions::l_bottom_right_shape(),
      definitions::l_top_left_shape(),
      definitions::l_top_right_shape(),
      definitions::long_shape(),
      definitions::rectangle(),
      definitions::small_square(),
      definitions::start_room(),
      definitions::tall(),
      definitions::wide(),
    ])
    .iter()
    .enumerate()
    {
      rooms.insert(num, template.clone());
    }

    calculate_combinations(&mut rooms);

    RoomTemplates { rooms }
  }

  pub fn of_type(&self, room_type: RoomType) -> Vec<(&usize, &RoomTemplate)> {
    self
      .rooms
      .iter()
      .filter(|(_, r)| r.room_type == room_type)
      .collect::<Vec<(&usize, &RoomTemplate)>>()
      .clone()
  }

  pub fn _random(&self) -> RoomTemplate {
    let mut rng = rand::thread_rng();
    let rooms_len = self.rooms.len();
    let rand_room = rng.gen_range(1..rooms_len);
    self.rooms.get(&rand_room).unwrap().clone()
  }
}

pub fn get(idx: &usize, rooms: &mut RoomTemplates) -> RoomTemplate {
  rooms.rooms.get(idx).unwrap().clone()
}

/// precalculate valid XY positions of all door combinations, these get randomly used when crawling through the original level graph
fn calculate_combinations(templates: &mut RoomTemplateMap) {
  let templates_b = templates.clone();
  // For each template
  for (_, template_a) in templates.iter_mut() {
    // Try all templates
    for (tb_i, template_b) in templates_b.iter() {
      // Create a placeholder array
      if !template_a.valid_combinations.contains_key(tb_i) {
        template_a.valid_combinations.insert(*tb_i, Vec::new());
      }

      // For each door template A
      for (door_a_type, door_xys) in template_a.possible_doors_xy.iter() {
        // For each door position on each face
        for door_a_xy in door_xys {
          // Get the corresponding door (eg, left door for right door)
          let door_b_type = connecting_door(*door_a_type);
          // Try each corresponding door of template B
          if let Some(doors_xy) = template_b.possible_doors_xy.get(&door_b_type) {
            for door_b_xy in doors_xy {
              // Align both templates and doors on a canvas and test their compatibility
              if let Some(template_b_xy) = can_place_room(door_a_xy, door_b_xy, template_a, template_b) {
                // println!("^ #{}/{} x:{} y:{}", ta_i, tb_i, template_b_x, template_b_y);
                let combination = (template_b_xy, *door_a_type, *door_a_xy, door_b_type, *door_b_xy);

                if let Some(combo) = template_a.valid_combinations.get_mut(tb_i) {
                  combo.push(combination);
                }
              }
            }
          };
        }
      }
    }
    // println!("{}: {:?}", ta_i, template_a.valid_combinations);
  }
}

/// Place the room into the centre of the canvas
fn canvas_with_room(room: &RoomTemplate, canvas_w: i32, canvas_h: i32) -> TemporaryCanvas {
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
fn can_place_room(xy_a: &XY, xy_b: &XY, template_a: &RoomTemplate, template_b: &RoomTemplate) -> Option<XY> {
  let canvas_width = template_a.w + (template_b.w * 2) + 1;
  let canvas_height = template_a.h + (template_b.h * 2) + 1;
  let (mut canvas_tiles, room_x, room_y) = canvas_with_room(&template_a, canvas_width, canvas_height);
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
  for tile in &template_b.tiles {
    // if this tile overlaps
    // thread 'main' panicked at 'index out of bounds
    if *tile != 0 && canvas_tiles[idx] == 8 {
      canvas_tiles[idx] = 9; // 9 = clash
      can_place = false;
    } else if *tile != 0 {
      canvas_tiles[idx] = *tile;
    }

    // end of the row, move down one and back to the left
    if room_x == template_b.w {
      room_x = 1;
      idx += (canvas_width - template_b.w + 1) as usize;
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
