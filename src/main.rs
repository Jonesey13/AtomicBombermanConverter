use std::{collections::HashMap, fs::File, io::{Read, Write}, path::Path};

use clap::{App, Arg};
use data::block::Block;

use crate::data::{player_pos::PlayerPos, powerup::Powerup, poweruptype::PowerupType, scheme::Scheme, team::Team};

mod data;
use serde_json;

fn main() {
    let matches = App::new("Atomic Bomberman Schema Serialiser")
        .version("0.1")
        .author("Alexander Jones <adjonesey13@gmail.com>")
        .about("Converts Atomic Bomberman Schema File to JSON")
        .arg(
            Arg::with_name("Scheme")
                .long("scheme-file")
                .short("s")
                .required(true)
                .value_name("FILE")
                .help("Location of the schema file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Json Output")
                .long("output")
                .short("o")
                .help("The Json Output File Name")
                .required(true)
                .value_name("FILE"),
        )
        .get_matches();

    let scheme = matches.value_of("Scheme").unwrap();
    println!("Value for Scheme: {}", scheme);

    let path = Path::new(scheme);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut scheme_string = String::new();
    match file.read_to_string(&mut scheme_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, scheme_string),
    }

    let scheme_lines: Vec<(&str, &str)> = scheme_string
        .split("\r\n")
        .filter(|line| line.chars().next() == Some('-'))
        .filter_map(|line| line.split_once(','))
        .collect();
    print!("{} contains key lines:\n{:?}", display, scheme_lines);

    let version = scheme_lines
        .iter()
        .find(|(pre, _)| *pre == "-V")
        .expect("No Version!")
        .1
        .to_string();

    let name = scheme_lines
        .iter()
        .find(|(pre, _)| *pre == "-N")
        .expect("No Name!")
        .1
        .to_string();

    let brick_density = scheme_lines
        .iter()
        .find(|(pre, _)| *pre == "-B")
        .expect("No Brick Density!")
        .1
        .parse::<usize>()
        .unwrap_or_default();

    let grid = generate_grid(&scheme_lines);

    let start_positions: Vec<PlayerPos> = build_player_positions(&scheme_lines);

    let powerups: HashMap<PowerupType, Powerup> = HashMap::new();

    let scheme = Scheme::new(
        name,
        version,
        brick_density,
        grid,
        start_positions,
        powerups
    );

    let scheme_json = serde_json::to_string(&scheme).expect("Serialisation failed!!!");

    let output_path = matches.value_of("Json Output").unwrap();
    println!("Value for Output File: {}", output_path);

    let path = Path::new(output_path);
    let display = path.display();


    let mut output_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match output_file.write_all(scheme_json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn build_player_positions(scheme_lines: &Vec<(&str, &str)>) -> Vec<PlayerPos> {
    scheme_lines
    .iter()
    .filter(|(pre, _)| *pre == "-S")
    .map(|line| line.1.split(',').filter_map(|string| string.parse::<usize>().ok()))
    .map(|mut parts| {
        let pos = (parts.nth(1).unwrap_or_default(), parts.next().unwrap_or_default());
        let team = Team::new(parts.next().unwrap_or_default());
        PlayerPos::new(pos, team)
    }).collect()
}

fn generate_grid(scheme_lines: &Vec<(&str, &str)>) -> Vec<Vec<Block>> {
    scheme_lines
        .iter()
        .filter(|(pre, _)| *pre == "-R")
        .filter_map(|line| line.1.split_once(',').and_then(|line| Some(line.1)))
        .map(|line| line.chars().map(|ch| Block::new(ch)).collect())
        .collect()
}
