use crate::components::{Int2Ops, MovingDir, Position, ShootingDir};
use crate::entities::*;
use bevy::asset::AssetLoader;
use bevy::prelude::*;
use std::collections::HashMap;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Level {
    pub number: usize,
    pub width: i32,
    pub height: i32,
    pub color: String,
    pub tiles: Vec<String>,
    pub screw_count: usize,
    pub additional: AdditionalMap,
}
type AdditionalMap = HashMap<(i32, i32), Vec<usize>>;

impl Level {
    pub fn parse(data: &str) -> Level {
        let mut _level_set_name: Option<&str> = None;
        let mut default_level_color: Option<String> = None;
        let mut collecting_data: bool = false;
        let mut lines = data.split('\n');
        let mut number: Option<usize> = None;
        let mut width: Option<i32> = None;
        let mut height: Option<i32> = None;
        let mut color: Option<String> = None;
        let mut additional: AdditionalMap = AdditionalMap::new();
        let mut tiles: Vec<String> = vec![];
        let mut screw_count = 0;
        loop {
            let line = lines.next().unwrap();
            if line.starts_with('[') {
                collecting_data = false;
            }
            match line {
                "[level]" => number = lines.next().unwrap().parse().ok(),
                "[name]" => {
                    _level_set_name = lines.next();
                }
                "[colour]" => {
                    color = Some(String::from(lines.next().unwrap()));
                }
                "[default_level_colour]" => {
                    default_level_color = lines.next().map(|s| s.to_owned())
                }
                "[size]" => {
                    let mut it = lines
                        .next()
                        .unwrap()
                        .split('.')
                        .map(|v| v.parse::<i32>().unwrap());
                    width = it.next();
                    height = it.next();
                }
                "[data]" => {
                    collecting_data = true;
                }
                "[additional]" => {
                    let cnt = lines.next().unwrap().parse::<usize>().unwrap();
                    for _ in 0..cnt {
                        let line = lines.next().unwrap();
                        let parts = line.split('.').collect::<Vec<&str>>();
                        let x = parts[0].parse::<usize>().unwrap();
                        let y = parts[1].parse::<usize>().unwrap();
                        let c = parts[2].chars().next().unwrap();
                        assert!(c == tiles[y].chars().nth(x).unwrap());
                        let params = parts[3..]
                            .iter()
                            .map(|v| v.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();
                        additional.insert((x as i32, y as i32), params);
                    }
                }
                "[end]" => {
                    if color.is_none() {
                        color = default_level_color.clone();
                    }
                    return Level {
                        number: number.unwrap(),
                        width: width.unwrap(),
                        height: height.unwrap(),
                        color: color.unwrap(),
                        tiles,
                        additional,
                        screw_count,
                    };
                }
                _ => {
                    if collecting_data {
                        screw_count += line.chars().filter(|t| *t == 'T').count();
                        tiles.push(String::from(line));
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct LevelLoader;
impl AssetLoader<Level> for LevelLoader {
    fn from_bytes(
        &self,
        _asset_path: &std::path::Path,
        bytes: Vec<u8>,
    ) -> Result<Level, anyhow::Error> {
        Ok(Level::parse(&String::from_utf8(bytes).unwrap()))
    }

    fn extensions(&self) -> &[&str] {
        static EXT: &[&str] = &["txt"];
        EXT
    }
}

pub fn create_level(
    commands: &mut Commands,
    items: &mut Query<With<Position, Entity>>,
    level: &Level,
) {
    for entity in &mut items.iter() {
        commands.despawn(entity);
    }

    for (x, column) in level.tiles.iter().enumerate() {
        for (y, c) in column.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);

            let additional = level.additional.get(&(y, x)).map(|v| &v[..]);
            match c {
                'O' => create_wall(commands, 0),
                'o' => create_wall(commands, 1),
                '-' => create_wall(commands, 2),
                'Q' => create_wall(commands, 3),
                'q' => create_wall(commands, 4),
                'p' => create_wall(commands, 5),
                'P' => create_wall(commands, 6),
                's' => create_wall(commands, 7),
                'S' => create_wall(commands, 8),
                'H' => create_ground(commands),
                'R' => create_robbo(commands),
                'D' => create_door(commands),
                '#' => create_static_box(commands),
                '~' => create_push_box(commands),
                '}' => create_gun(commands, additional.unwrap_or(&[0, 0, 0, 0, 0, 0])),
                //'L' => create_horizontal_laser(commands),
                //'l' => create_vertical_laser(commands),
                '&' => create_teleport(commands, additional.unwrap_or(&[0, 0])),
                '^' => create_bird(commands).with(ShootingDir::new(1, 0).with_propability(0.05)),
                '@' => create_lbear(commands).with(MovingDir::new(-1, 0)),
                '*' => create_rbear(commands).with(MovingDir::new(1, 0)),
                'V' => create_eyes(commands),
                '\'' => create_ammo(commands),
                'T' => create_screw(commands),
                '%' => create_key(commands),
                '!' => create_capsule(commands),
                'b' => create_bomb(commands),
                '?' => create_questionmark(commands),
                '=' => create_forcefield(commands), // TODO - direction
                'M' => create_magnet(commands),     // TODO - direction
                _ => continue,
            };
            commands.with(Position::new(x, y));
        }
    }
}
