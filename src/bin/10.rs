use std::str::FromStr;

advent_of_code::solution!(10);

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn get_s_pos(&self) -> (usize, usize) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 'S' {
                    return (i, j);
                }
            }
        }
        panic!("No s found");
    }

    fn get_loop_start_pos(&self) -> (usize, usize) {
        let (i, j) = self.get_s_pos();
        // try to go up
        if i > 0 && "|7F".contains(self.map[i - 1][j]) {
            return (i - 1, j);
        }
        // try to go down
        if i < self.map.len() - 1 && "|LJ".contains(self.map[i + 1][j]) {
            return (i + 1, j);
        }
        // try to go left
        if j > 0 && "-FL".contains(self.map[i][j - 1]) {
            return (i, j - 1);
        }
        // try to go right
        if j < self.map[i].len() - 1 && "-J7".contains(self.map[i][j + 1]) {
            return (i, j + 1);
        }
        panic!("No loop start found");
    }

    fn get_loop_length_and_vertices(&self) -> (u32, Vec<(usize, usize)>) {
        let mut prev = self.get_s_pos();
        let mut cur = self.get_loop_start_pos();
        let mut loop_len = 1;
        let mut vertices = vec![prev];
        while self.map[cur.0][cur.1] != 'S' {
            let pos = cur;
            match self.map[cur.0][cur.1] {
                '7' => {
                    if prev.0 == cur.0 {
                        cur.0 += 1
                    } else {
                        cur.1 -= 1
                    }
                    vertices.push(pos);
                }
                'F' => {
                    if prev.0 == cur.0 {
                        cur.0 += 1
                    } else {
                        cur.1 += 1
                    }
                    vertices.push(pos);
                }
                'L' => {
                    if prev.0 == cur.0 {
                        cur.0 -= 1
                    } else {
                        cur.1 += 1
                    }
                    vertices.push(pos);
                }
                'J' => {
                    if prev.0 == cur.0 {
                        cur.0 -= 1
                    } else {
                        cur.1 -= 1
                    }
                    vertices.push(pos);
                }
                '|' => {
                    if prev.0 < cur.0 {
                        cur.0 += 1
                    } else {
                        cur.0 -= 1
                    }
                }
                '-' => {
                    if prev.1 < cur.1 {
                        cur.1 += 1
                    } else {
                        cur.1 -= 1
                    }
                }
                _ => panic!("Invalid char"),
            }
            prev = pos;
            loop_len += 1;
        }
        (loop_len, vertices)
    }

    fn get_max_dist_from_s(&self) -> u32 {
        self.get_loop_length_and_vertices().0 / 2
    }

    fn get_surface(&self) -> u32 {
        let (loop_len, vertices) = self.get_loop_length_and_vertices();
        let mut surface = 0;
        // use shoelace formula
        for i in 0..vertices.len() {
            let x1 = vertices[i as usize].1 as i32;
            let y1 = vertices[i as usize].0 as i32;
            let x2 = vertices[(i + 1) as usize % vertices.len() as usize].1 as i32;
            let y2 = vertices[(i + 1) as usize % vertices.len() as usize].0 as i32;
            surface += x1 * y2 - x2 * y1;
        }
        surface.abs() as u32 / 2 - loop_len / 2 + 1
    }
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Map { map })
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = input.parse::<Map>().expect("Failed to parse map");
    Some(map.get_max_dist_from_s())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input.parse::<Map>().expect("Failed to parse map");
    Some(map.get_surface())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
