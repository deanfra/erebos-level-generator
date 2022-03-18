pub type XY = (i32, i32);

/// multiplies the y position by the map width (80), and adds x. This guarantees one tile per location, and efficiently maps it in memory for left-to-right reading
pub fn xy_idx(x: i32, y: i32, w: i32) -> usize {
  (y * w + x) as usize
}
