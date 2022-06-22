pub struct Grid2d {
    obstacles: Vec<bool>,
    default_enterable_dirs: Vec<[bool; 4]>,
    enterable_dirs: Vec<[bool; 4]>,
    height: usize,
    width: usize,
}

// TODO: 各マスを跨いだ時の移動コストを追加？
impl Grid2d {
    const L: usize = 0;
    const R: usize = 1;
    const U: usize = 2;
    const D: usize = 3;

    fn rev_dir(dir: usize) -> usize {
        match dir {
            Self::L => Self::R,
            Self::R => Self::L,
            Self::U => Self::D,
            Self::D => Self::U,
            _ => unreachable!(),
        }
    }

    fn new(height: usize, width: usize) -> Self {
        let mut default_enterable_dirs = vec![[true; 4]; height * width];
        for x in 0..width {
            // 最上行
            let up_pos = 0 * width + x;
            default_enterable_dirs[up_pos][Self::U] = false;

            // 最下行
            let bottom_pos = (height - 1) * width + x;
            default_enterable_dirs[bottom_pos][Self::D] = false;
        }

        for y in 0..height {
            // 最左列
            let left_pos = y * width + 0;
            default_enterable_dirs[left_pos][Self::L] = false;

            // 最右列
            let right_pos = y * width + width - 1;
            default_enterable_dirs[right_pos][Self::R] = false;
        }

        Grid2d {
            obstacles: vec![false; height * width],
            enterable_dirs: default_enterable_dirs.clone(),
            default_enterable_dirs,
            height,
            width,
        }
    }

    fn get_pos(&self, coordinate: (usize, usize)) -> usize {
        coordinate.0 * self.width + coordinate.1
    }

    fn get_moved_coordinate(&self, coordinate: (usize, usize), dir: usize) -> (usize, usize) {
        match dir {
            Self::L => (coordinate.0, coordinate.1 - 1),
            Self::R => (coordinate.0, coordinate.1 + 1),
            Self::U => (coordinate.0 - 1, coordinate.1),
            Self::D => (coordinate.0 + 1, coordinate.1),
            _ => unreachable!(),
        }
    }

    /// set obstacle (y.x)
    fn set_obstacle(&mut self, coordinate: (usize, usize)) {
        let pos = self.get_pos(coordinate);
        self.obstacles[pos] = true;

        let block_list: Vec<(usize, usize)> = self.default_enterable_dirs[pos]
            .iter()
            .enumerate()
            .filter_map(|(dir, enterable)| {
                if *enterable {
                    let new_coordinate = self.get_moved_coordinate(coordinate, dir);
                    let new_pos = self.get_pos(new_coordinate);
                    let rev_dir = Self::rev_dir(dir);

                    Some((new_pos, rev_dir))
                } else {
                    None
                }
            })
            .collect();

        block_list.iter().for_each(|&(block_pos, block_dir)| {
            self.enterable_dirs[block_pos][block_dir] = false;
        });
    }

    // ダイクストラ(2点間最小距離、2点間最小経路(座標の配列、方向の配列))
    fn calc_min_dist(&self, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
        let mut que = std::collections::VecDeque::new();
        que.push_back((start, 0));

        let mut visited = vec![false; self.height * self.width];

        let mut s_to_g = None;

        while !que.is_empty() {
            let (now, dist) = que.pop_back().unwrap();
            if now == goal {
                s_to_g = Some(dist);
                break;
            }

            let pos = self.get_pos(now);
            if visited[pos] {
                continue;
            }
            visited[pos] = true;

            self.enterable_dirs[self.get_pos(now)]
                .iter()
                .enumerate()
                .for_each(|(dir, &enterable)| {
                    if enterable {
                        let new = self.get_moved_coordinate(now, dir);
                        que.push_front((new, dist + 1));
                    }
                });
        }

        s_to_g
    }

    // ダイクストラ(2点間最小経路(座標の配列))
    fn calc_min_route_coordinate_sequence() {
        todo!();
    }

    // ダイクストラ(2点間最小経路(方向の配列))
    fn calc_min_route_dirs() {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h4w4_empty_test() {
        let grid = Grid2d::new(4, 4);
        assert_eq!(grid.calc_min_dist((0, 0), (3, 3)), Some(6));
    }

    #[test]
    fn h4w4_impossible_test() {
        /*
        S---
        ####
        ----
        ---G
        */
        let mut grid = Grid2d::new(4, 4);
        grid.set_obstacle((1, 0));
        grid.set_obstacle((1, 1));
        grid.set_obstacle((1, 2));
        grid.set_obstacle((1, 3));
        assert_eq!(grid.calc_min_dist((0, 0), (3, 3)), None);
    }

    #[test]
    fn h5w4_map_test() {
        /*
        S---
        ###-
        ----
        -###
        ---G
        */
        let mut grid = Grid2d::new(5, 4);
        grid.set_obstacle((1, 0));
        grid.set_obstacle((1, 1));
        grid.set_obstacle((1, 2));
        grid.set_obstacle((3, 1));
        grid.set_obstacle((3, 2));
        grid.set_obstacle((3, 3));
        assert_eq!(grid.calc_min_dist((0, 0), (4, 3)), Some(13));
    }
}
