use crate::map::room::DoorConnection;

use super::map::Map;
use petgraph::{matrix_graph::NodeIndex, stable_graph::StableGraph};

#[allow(dead_code)]
pub fn print_map(tiles: Vec<u8>, width: i32) {
  let mut x = 0;
  let u_width = width as usize;

  // Filter out any empty map rows
  let cropped_tiles: Vec<u8> = (1..tiles.len() / u_width + 1)
    .filter_map(|i: usize| {
      let last = i * u_width;
      let first = last - u_width;
      let tile_sum: u8 = tiles[first..last].iter().sum();
      if tile_sum == 0 {
        None
      } else {
        Some(tiles[first..last].to_vec())
      }
    })
    .collect::<Vec<Vec<u8>>>()
    .concat();

  for tile in cropped_tiles {
    if tile == 0 {
      // SPACE: grey
      print!("  ");
    } else if tile == 9 {
      // CONFLICT: red
      print!("\x1B[31mx\x1B[39m ");
    } else if tile == 8 {
      // BG: black
      print!("\x1B[30m🀫\x1B[39m ");
    } else if tile == 7 {
      // DOOR: blue
      print!("\x1B[34m🀫\x1B[39m ");
    } else {
      // WALL: grey white
      print!("🀕 ");
    }
    // Move the coordinates
    x += 1;
    // end of the row, move down one and back to the left
    if (x + 1) > width {
      println!("");
      x = 0;
    }
  }
}

#[allow(dead_code)]
pub fn print_map_history(history: Vec<Vec<u8>>, width: i32) {
  for tiles in history {
    print_map(tiles, width);
  }
}

#[allow(dead_code)]
pub fn print_er_diagram(graph: &StableGraph<usize, usize>, nodes: &Vec<NodeIndex<u32>>) {
  println!("erDiagram");

  let weights = graph.node_weights().collect::<Vec<&usize>>();

  for (i, node) in nodes.iter().enumerate() {
    let weight = weights.get(i).unwrap();
    graph.neighbors(*node).for_each(|edge| {
      let weight_2 = weights.get(edge.index()).unwrap();
      println!("N{:?}-{} ||--|| N{:?}-{}: \"\"", node.index(), weight, edge.index(), weight_2);
    });
  }
}

#[allow(dead_code)]
pub fn print_plantuml_nodes(graph: &StableGraph<usize, usize>, nodes: &Vec<NodeIndex<u32>>) {
  println!("@startuml");
  println!("(*) --> (*)");

  let weights = graph.node_weights().collect::<Vec<&usize>>();

  for (i, node) in nodes.iter().enumerate() {
    let weight = weights.get(i).unwrap();

    graph.neighbors(*node).for_each(|edge| {
      let weight_2 = weights.get(edge.index()).unwrap();
      println!("\"N{:?}-W{}\" --> \"N{:?}-W{}\"", node.index(), weight, edge.index(), weight_2);
    });
  }
  println!("@enduml");
}

#[allow(dead_code)]
pub fn print_plantuml_map(graph: &StableGraph<usize, usize>, map: &Map) {
  print!("@startuml\n  !theme crt-green\n  (*) --> (*)\n");

  let weights = graph.node_weights().collect::<Vec<&usize>>();

  for (room_idx, room) in map.rooms.clone() {
    let weight = weights.get(room_idx).unwrap();

    for (_, connections) in room.door_connections {
      for connection in connections {
        let DoorConnection {
          node_a_idx,
          node_b_idx,
          direction,
          ..
        } = connection;

        let weight_2 = weights.get(node_b_idx).unwrap();

        if direction {
          println!("  \"N{:?}-W{}\" --> \"N{:?}-W{}\"", node_a_idx, weight, node_b_idx, *weight_2);
        }
      }
    }
  }

  println!("@enduml");
}

/*
'bold'      : ['\x1B[1m',  '\x1B[22m'],
'italic'    : ['\x1B[3m',  '\x1B[23m'],
'underline' : ['\x1B[4m',  '\x1B[24m'],
'inverse'   : ['\x1B[7m',  '\x1B[27m'],
'strikethrough' : ['\x1B[9m',  '\x1B[29m'],

//grayscale
'white'     : ['\x1B[37m', '\x1B[39m'],
'grey'      : ['\x1B[90m', '\x1B[39m'],
'black'     : ['\x1B[30m', '\x1B[39m'],

//colors
'blue'      : ['\x1B[34m', '\x1B[39m'],
'cyan'      : ['\x1B[36m', '\x1B[39m'],
'green'     : ['\x1B[32m', '\x1B[39m'],
'magenta'   : ['\x1B[35m', '\x1B[39m'],
'red'       : ['\x1B[31m', '\x1B[39m'],
'yellow'    : ['\x1B[33m', '\x1B[39m']
*/
