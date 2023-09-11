use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct Scene {
    tag: SceneID,
    descs: Vec<Desc>,
    options: Vec<Option>
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
struct SceneID(String);

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

fn scene_index_from_tag (scenes: &Vec<Scene>, tag:&SceneID) -> i32 {
    let mut i:i32 = 0;
    for scene in scenes {
        if scene.tag == *tag {return i};
        i += 1;
    }
    return 0; //TODO: add better error handling- what if no scenes with tag?
}

fn get_desc (scenes: &Vec<Scene>, state: &Gamestate) -> String {
    let mut i: usize = 0;
    let descs_len = scenes[state.scene_i].descs.len();
    while i < descs_len {
        if state.sanity >= scenes[state.scene_i].descs[i].min_san {
            return scenes[state.scene_i].descs[i].text.clone();
        }
    }

    // If you have no valid description, then notify of the error.
    return "Strange. You've entered a bizarre land, with no valid descriptions
        for the room you find yourself in. (The scene has ".to_string() + 
        descs_len.to_string().as_str() + 
        " descs that have a min sanity above your current one."
        
}

fn get_valid_options_list(scenes: &Vec<Scene>, state: &Gamestate) -> Vec<Option> {
    let mut valid_opts = Vec::new();
    let opts = scenes[state.scene_i].options.clone();

    for opt in opts {
        if opt.max_san >= state.sanity && state.sanity >= opt.min_san {
            valid_opts.push(opt)
        }
    }
    
    valid_opts
}

fn main() {
    let scenes: Vec<Scene> = read_scene_data("scenes.json").unwrap();

    let mut state:Gamestate = Gamestate {
        scene_tag: scenes[0].tag.clone(),
        scene_i: 0,
        sanity: 100
    };

    let mut exit_bool:bool = false;
    while !exit_bool {
        
        // Print the current scene.
        println!("{}", get_desc(&scenes, &state));
        println!(); //newline

        // Print the scene's options.
        let mut opts = get_valid_options_list(&scenes, &state);
        for opt in opts.iter().enumerate() {
            println!("{}: {}", opt.0 + 1, opt.1.text)
        }
        println!(); //newline

        // TODO: get the player's input.
        let chosen_opt:usize = 0;

        // Apply the effects of that option.
        println!("{}", opts[chosen_opt].text_when_chosen);
        state.sanity += opts[chosen_opt].san_change;
        // TODO: change the state.




        exit_bool = true; // TODO: remove this. it's debug, so that we don't
                          // loop forever (yet).
    }
}
