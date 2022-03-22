use super::{room_templates::RoomTemplate, xy_idx, XY};
use std::collections::HashMap;

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

#[derive(Default, Clone)]
pub struct Room {
  pub x: i32,
  pub y: i32,
  pub template: RoomTemplate,
  /// The XY coords for each used door
  pub door_connections: DoorConnections,
  /// The XY coords for each possible door
  pub tiles: Vec<u8>,
}

impl Room {
  pub fn new(template: RoomTemplate) -> Room {
    Room {
      tiles: template.tiles.clone(),
      template,
      ..Default::default()
    }
  }

  pub fn add_door(&mut self, node_a_idx: usize, node_b_idx: usize, door_type: u8, xy: XY, dir: bool) {
    let connection = DoorConnection {
      node_a_idx,
      node_b_idx,
      xy,
      direction: dir,
    };

    if let Some(doors) = self.door_connections.get_mut(&door_type) {
      doors.push(connection);
    } else {
      self.door_connections.insert(door_type, Vec::from([connection]));
    }

    let idx = xy_idx(xy.0 - 1, xy.1 - 1, self.template.w);

    // Assign a connected door index
    self.tiles[idx] = 7;
  }
}

/// A struct to represent how two rooms connect
#[derive(Clone)]
pub struct DoorConnection {
  /// The node index of the current room
  pub node_a_idx: usize,
  /// The node index of the target room
  pub node_b_idx: usize,
  /// XY Coordinates
  xy: XY,
  /// Direction of the connection - true = A->B | false = B->A
  pub direction: bool,
}

/// Hashmap of each door type and their connections
pub type DoorConnections = HashMap<u8, Vec<DoorConnection>>;
pub type DoorsXY = HashMap<u8, Vec<XY>>;
