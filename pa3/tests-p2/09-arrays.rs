fn main() {
      [1, 2, 3];
      [1, false, 3];
      [[1], [2], [3]];

      let x = [[false || true], [[1 + 2]], [[3]], [[1 < 3]]];
      let y : [i32;5] = [1, 2, 3, 4];
      let z : [[i32;2];3] = [[1, 2], [3, 4], [1 + 2 + 3 * 4, 5]];

      z[0] + z[2][0];
      z[1][1] || z[0][0];
      z[1][1] + z[0][0]
}
