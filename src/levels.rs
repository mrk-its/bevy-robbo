use crate::components::{Int2Ops, Kind, MovingDir, Position, ShootingDir, StartPosition};
use crate::entities;
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
    pub additional: AdditionalMap,
}
type AdditionalMap = HashMap<(i32, i32), Vec<u16>>;

impl Level {
    pub fn parse(data: &str) -> Level {
        let mut level_set_name: Option<&str> = None;
        let mut default_level_color: Option<String> = None;
        let mut collecting_data: bool = false;
        let mut lines = data.split('\n');
        let mut number: Option<usize> = None;
        let mut width: Option<i32> = None;
        let mut height: Option<i32> = None;
        let mut color: Option<String> = None;
        let mut additional: AdditionalMap = AdditionalMap::new();
        let mut tiles: Vec<String> = vec![];

        loop {
            let line = lines.next().unwrap();
            if line.starts_with('[') {
                collecting_data = false;
            }
            match line {
                "[level]" => number = lines.next().unwrap().parse().ok(),
                "[name]" => {
                    level_set_name = lines.next();
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
                            .map(|v| v.parse::<u16>().unwrap())
                            .collect::<Vec<u16>>();
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
                    };
                }
                _ => {
                    if collecting_data {
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
    items: &mut Query<(Entity, &Kind, &StartPosition)>,
    level: &Level,
) {
    for (x, column) in level.tiles.iter().enumerate() {
        for (y, c) in column.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            let additional = level.additional.get(&(y, x)).map(|v| &v[..]);
            match c {
                'O' => entities::wall(commands, 0),
                'o' => entities::wall(commands, 1),
                '-' => entities::wall(commands, 2),
                'Q' => entities::wall(commands, 3),
                'q' => entities::wall(commands, 4),
                'p' => entities::wall(commands, 5),
                'P' => entities::wall(commands, 6),
                's' => entities::wall(commands, 7),
                'S' => entities::wall(commands, 8),
                'H' => entities::ground(commands),
                'R' => entities::robbo(commands),
                'D' => entities::door(commands),
                '#' => entities::static_box(commands),
                '~' => entities::push_box(commands),
                '}' => entities::gun(commands, additional.unwrap_or(&[0, 0, 0, 0, 0, 0])),
                //'L' => entities::horizontal_laser(commands),
                //'l' => entities::vertical_laser(commands),
                '&' => entities::teleport(commands),
                '^' => entities::bird(commands).with(ShootingDir::new(1, 0).with_propability(0.05)),
                '@' => entities::lbear(commands).with(MovingDir::new(-1, 0)),
                '*' => entities::rbear(commands).with(MovingDir::new(1, 0)),
                '\'' => entities::ammo(commands),
                'T' => entities::screw(commands),
                '%' => entities::key(commands),
                '!' => entities::capsule(commands),
                'b' => entities::bomb(commands),
                '?' => entities::questionmark(commands),
                _ => {
                    for (entity, _, pos) in &mut items.iter() {
                        if pos.as_tuple() == (x, y) {
                            commands.despawn(entity);
                        }
                    }
                    continue;
                }
            };
            let entity = items
                .iter()
                .into_iter()
                .find(|(_, _, pos)| pos.as_tuple() == (x, y))
                .map(|t| t)
                .is_some();

            if entity {
                commands.despawn(commands.current_entity().unwrap());
                continue;
            }

            commands
                .with(Position::new(x, y))
                .with(StartPosition::new(x, y));
        }
    }
}
