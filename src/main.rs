use std::{fs, path::PathBuf};

use regex::Regex;
use walkdir::{ WalkDir};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "jokerlin <anonymous.joker.lin@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    room: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    POKERSTARS(Room),
}

/// convert to pokerstars format
#[derive(Parser, Debug)]
struct Room {
    #[clap(parse(try_from_str))]
    /// folder path
    path: String,
}

fn get_filenames(path: &str) -> Vec<PathBuf> {
    let mut v = vec![];
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.file_name().to_str().map(|s|s.ends_with(".txt")).unwrap_or(true) {
            v.push(file.path().to_owned());
        }
    }
    v
}

fn read_original_hands(file: &PathBuf) -> std::io::Result<String> {
    let hands = fs::read_to_string(file).unwrap();

    Ok(hands)
}

fn convert_hands(hands: String) -> String {
    let mut converted = str::replace(&hands, "Poker Hand #RC", "PokerStars Hand #20");

    converted = str::replace(&converted, "Dealt to Hero", "XXXXXXXXXX");

    let re = Regex::new(r"Dealt to [0-9A-Za-z_]{0,}\s+").unwrap();
    converted = re.replace_all(&converted, "").to_string();

    converted = str::replace(&converted, "XXXXXXXXXX", "Dealt to Hero");

    converted
}

fn write_converted_hands(hands: &String, filename: &PathBuf) -> std::io::Result<()> {
    let name: PathBuf = PathBuf::from(filename.file_name().unwrap());
    fs::write(std::path::Path::new("./converted/").join(name), hands)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    match opts.room {
        SubCommand::POKERSTARS(room) => {
            let file_array = get_filenames(&room.path);

            fs::create_dir_all("./converted")?;

            for f in file_array.iter() {
                write_converted_hands(&convert_hands(read_original_hands(&f)?), &f)?;
            }

            Ok(())
        }
    }
}
