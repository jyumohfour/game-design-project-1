use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct Scene {
    tag: String,
    descs: Vec<Desc>,
    options: Vec<Option>
}

#[derive(Deserialize)]
struct Desc {
    text: String,
    min_san: i32
}

#[derive(Deserialize)]
struct Option {
    to_scene: String,
    text: String,
    text_when_chosen: String,
    min_san: i32,
    max_san: i32,
    san_change: i32
}

struct Gamestate {
    current_scene: Scene,
    sanity: i32
}

fn read_scene_data<P: AsRef<Path>>(path: P) -> Result<Vec<Scene>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
    let scenes = read_scene_data("scenes.json").unwrap();
    
    // debug - printing room data from the json
    println!("{}", scenes[0].tag);
    println!("{}", scenes[0].descs[0].text);
    println!("{}", scenes[0].options[0].text);
    println!("{}", scenes[0].options[0].text_when_chosen);
    println!("{}", scenes[1].descs[0].text);
}
