use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;
use std::fmt;

const MIN_SAN:i32 = 0;
const MAX_SAN:i32 = 100;
const FILE_TO_LOAD:&str = "scenes.json";

#[derive(Deserialize)]
struct Scene {
    tag: SceneID,
    descs: Vec<Desc>,
    options: Vec<Option>
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
struct SceneID(String);
impl fmt::Display for SceneID { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize)]
struct Desc {
    text: String,
    min_san: i32
}

#[derive(Deserialize, Clone)]
struct Option {
    to_scene: SceneID,
    text: String,
    text_when_chosen: String,
    min_san: i32,
    max_san: i32,
    san_change: i32
}

struct Gamestate {
    scene_tag: SceneID,
    scene_i: usize,
    sanity: i32
}


fn read_scene_data<P: AsRef<Path>>(path: P) -> Result<Vec<Scene>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let scenes = serde_json::from_reader(reader)?;
    Ok(scenes)
}

fn scene_index_from_tag (scenes: &Vec<Scene>, tag:&SceneID) -> Result<usize, Box<dyn Error>> {
    let mut i:usize = 0;
    for scene in scenes {
        if scene.tag == *tag {return Ok(i)};
        i += 1;
    }

    // If we haven't returned by now, then there is no scene with that tag.
    return Err("")? 
}

fn get_desc (scenes: &Vec<Scene>, state: &Gamestate) -> String {
    let mut i: usize = 0;
    let descs_len: usize = scenes[state.scene_i].descs.len();
    while i < descs_len {
        if state.sanity >= scenes[state.scene_i].descs[i].min_san {
            return scenes[state.scene_i].descs[i].text.clone();
        }
        i += 1;
    }

    // If you have no valid description, then notify of the error.
    // This shouldn't prevent the game from running, though.
    return "Strange. You've entered a bizarre land, with no valid descriptions
        for the room you find yourself in.\nScene ID: ".to_string() + 
        &scenes[state.scene_i].tag.to_string() + "\n(The scene has " + 
        descs_len.to_string().as_str() + 
        " descs that have a min sanity above your current one."
        
}

fn get_valid_options_list(scenes: &Vec<Scene>, state: &Gamestate) -> Vec<Option> {
    let mut valid_opts: Vec<Option> = Vec::new();
    let opts = scenes[state.scene_i].options.clone();

    for opt in opts {
        if opt.max_san >= state.sanity && state.sanity >= opt.min_san {
            valid_opts.push(opt)
        }
    }
    
    valid_opts
}

fn main() {
    // Initialize the vector of the game's scenes.
    let scenes: Vec<Scene> = read_scene_data(FILE_TO_LOAD).unwrap();

    let mut state:Gamestate = Gamestate {
        scene_tag: scenes[0].tag.clone(),
        scene_i: 0,
        sanity: 100
    };

    // Begin the main game loop.
    let mut exit_bool:bool = false;
    while !exit_bool {
        
        // Print the current scene.
        println!("{}", get_desc(&scenes, &state));
        println!(); //newline


        // Print the scene's options.
        let opts = get_valid_options_list(&scenes, &state);
        if opts.len() == 0 {
            // If there are no options, then the game is over.
            println!("Thanks for playing!\nShutting down...");
            break;
        }
        for opt in opts.iter().enumerate() {
            println!("{}: {}", opt.0 + 1, opt.1.text)
        }
        println!(); // Add a newline between the options and the input.


        // TODO: get the player's input.
        let chosen_opt:usize = 0;


        // Apply the effects of that option.
        println!("{}", opts[chosen_opt].text_when_chosen);

        state.sanity += opts[chosen_opt].san_change;
        if state.sanity > MAX_SAN {state.sanity = MAX_SAN}
        else if state.sanity < MIN_SAN {state.sanity = MIN_SAN};

        state.scene_tag = opts[chosen_opt].to_scene.clone();
        match scene_index_from_tag(&scenes, &state.scene_tag) {
            Ok(u) => state.scene_i = u,
            Err(_) => {
                println!("ERROR: there is no scene with tag {}.\n
                Exiting game...", &state.scene_tag.to_string());
                break
            }
        }

        exit_bool = true; // TODO: remove this. it's debug, so that we don't
                          // loop forever (yet).
    }
}
