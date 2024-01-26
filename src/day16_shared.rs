use std::str::Lines;

pub fn illuminate(lines: Lines, starting_light: Light) -> usize {
    let mut grid = lines.map(format_line).collect::<Vec<Vec<Node>>>();
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let mut lights = Vec::from([starting_light]);
    let mut visited_count = 0;
    while lights.len() != 0 {
        let light = lights.pop().unwrap();
        let tile = &mut grid[light.y][light.x];
        if tile.visited_directions.len() == 0 {
            visited_count += 1;
        } else if tile.visited_directions.contains(&light.direction) {
            continue;
        }
        match (&tile.tile_type, &light.direction) {
            (TileType::VerticalSplitter, Direction::Left | Direction::Right) => {
                if light.y > 0 {
                    lights.push(Light {
                        x: light.x,
                        y: light.y - 1,
                        direction: Direction::Up,
                    });
                }
                if light.y + 1 < grid_height {
                    lights.push(Light {
                        x: light.x,
                        y: light.y + 1,
                        direction: Direction::Down,
                    });
                }
            }
            (TileType::HorizontalSplitter, Direction::Up | Direction::Down) => {
                if light.x + 1 < grid_width {
                    lights.push(Light {
                        x: light.x + 1,
                        y: light.y,
                        direction: Direction::Right,
                    });
                }
                if light.x > 0 {
                    lights.push(Light {
                        x: light.x - 1,
                        y: light.y,
                        direction: Direction::Left,
                    });
                }
            }
            (TileType::UpMirror, Direction::Up) => {
                if light.x + 1 < grid_width {
                    lights.push(Light {
                        x: light.x + 1,
                        y: light.y,
                        direction: Direction::Right,
                    });
                }
            }
            (TileType::UpMirror, Direction::Right) => {
                if light.y > 0 {
                    lights.push(Light {
                        x: light.x,
                        y: light.y - 1,
                        direction: Direction::Up,
                    });
                }
            }
            (TileType::UpMirror, Direction::Left) => {
                if light.y + 1 < grid_height {
                    lights.push(Light {
                        x: light.x,
                        y: light.y + 1,
                        direction: Direction::Down,
                    });
                }
            }
            (TileType::UpMirror, Direction::Down) => {
                if light.x > 0 {
                    lights.push(Light {
                        x: light.x - 1,
                        y: light.y,
                        direction: Direction::Left,
                    });
                }
            }

            (TileType::DownMirror, Direction::Down) => {
                if light.x + 1 < grid_width {
                    lights.push(Light {
                        x: light.x + 1,
                        y: light.y,
                        direction: Direction::Right,
                    });
                }
            }
            (TileType::DownMirror, Direction::Right) => {
                if light.y + 1 < grid_height {
                    lights.push(Light {
                        x: light.x,
                        y: light.y + 1,
                        direction: Direction::Down,
                    });
                }
            }
            (TileType::DownMirror, Direction::Left) => {
                if light.y > 0 {
                    lights.push(Light {
                        x: light.x,
                        y: light.y - 1,
                        direction: Direction::Up,
                    });
                }
            }
            (TileType::DownMirror, Direction::Up) => {
                if light.x > 0 {
                    lights.push(Light {
                        x: light.x - 1,
                        y: light.y,
                        direction: Direction::Left,
                    });
                }
            }
            (_, Direction::Up) => {
                if light.y > 0 {
                    lights.push(Light {
                        x: light.x,
                        y: light.y - 1,
                        direction: Direction::Up,
                    });
                }
            }
            (_, Direction::Right) => {
                if light.x + 1 < grid_width {
                    lights.push(Light {
                        x: light.x + 1,
                        y: light.y,
                        direction: Direction::Right,
                    });
                }
            }
            (_, Direction::Left) => {
                if light.x > 0 {
                    lights.push(Light {
                        x: light.x - 1,
                        y: light.y,
                        direction: Direction::Left,
                    });
                }
            }
            (_, Direction::Down) => {
                if light.y + 1 < grid_height {
                    lights.push(Light {
                        x: light.x,
                        y: light.y + 1,
                        direction: Direction::Down,
                    });
                }
            }
        }

        tile.visited_directions.push(light.direction);
    }
    return visited_count;
}

fn format_line(line: &str) -> Vec<Node> {
    return line
        .chars()
        .map(|c| Node::from_char(c))
        .collect::<Vec<Node>>();
}
struct Node {
    tile_type: TileType,
    visited_directions: Vec<Direction>,
}

pub struct Light {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) direction: Direction,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum TileType {
    VerticalSplitter,
    HorizontalSplitter,
    UpMirror,
    DownMirror,
    Empty,
}

impl Node {
    fn from_char(c: char) -> Self {
        let tile_type = match c {
            '|' => TileType::VerticalSplitter,
            '-' => TileType::HorizontalSplitter,
            '/' => TileType::UpMirror,
            '\\' => TileType::DownMirror,
            '.' => TileType::Empty,
            _ => panic!("Unknown tile type: {}", c),
        };
        return Node {
            tile_type: tile_type,
            visited_directions: Vec::new(),
        };
    }
}
