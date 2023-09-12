use std::error::Error;
use std::io;
use std::io::Write;
use std::fs::File;
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

#[derive(Debug, Clone)]
struct ManualExit;
impl Error for ManualExit {}
impl fmt::Display for ManualExit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Thanks for playing!\nShutting down...")
    }
}


fn read_scene_data<P: AsRef<Path>>(path: P) -> Result<Vec<Scene>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let scenes = serde_json::from_reader(reader)?;
    Ok(scenes)
}

fn scene_index_from_tag (scenes: &Vec<Scene>, tag:&SceneID) -> usize {
    let mut i:usize = 0;
    for scene in scenes {
        if scene.tag == *tag {return i};
        i += 1;
    }

    // If we haven't returned by now, then there is no scene with that tag.
    panic!("there is no scene with tag {}",
                 &tag.to_string());
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
    "Strange. You've entered a bizarre land, with no valid descriptions for the room you find yourself in.\nScene ID: ".to_string()
        + &scenes[state.scene_i].tag.to_string() + "\n(The scene has " 
        + descs_len.to_string().as_str()
        + " descs that have a min sanity above your current one."
        
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

fn get_player_choice(optcount: usize) -> Result<usize, Box<dyn Error>> {
    // Return an error if the player quits (or something goes wrong).


    loop {
        let mut response = String::new();

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut response).unwrap();
        response = response.trim().to_string();
        
        //First, check that the player didn't quit.
        if response == "Q" || response == "q" {
            return Err(Box::new(ManualExit))
        }

        //Then, check if it's valid.
        match response.parse::<usize>() {
            Ok(u) => {
                if u <= optcount { // Chose an option, and it's in range.
                    return Ok(u - 1);
                } else { // Chose an option outside of range.
                    println!("Sorry, there is no option with that tag.\nPlease choose an option from 1 to {}.\n",
                        optcount.to_string().as_str())
                }
            }  
            Err(_) => { // User typed in the wrong format.
                println!("Could not read response. Please type a number from 1 to {} to choose an option, or type 'Q' to quit.\n",
                    optcount.to_string().as_str())
            }
        }
    }

    

    

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
    loop {
        
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
            // the +1 is so that the displayed numbers range from 1 to len
        }
        println!(); // Add a newline between the options and the input.



        // Let the player choose an option.
        let chosen_opt;
        match get_player_choice(opts.len()) {
            Ok(u) => chosen_opt = u,
            Err(e) => {
                // Player quit game.
                println!("{}", e);
                break;
            }
        }


        // Apply the effects of that option.
        println!("{}\n", opts[chosen_opt].text_when_chosen);

        state.sanity += opts[chosen_opt].san_change;
        if state.sanity > MAX_SAN {state.sanity = MAX_SAN}
        else if state.sanity < MIN_SAN {state.sanity = MIN_SAN};

        state.scene_tag = opts[chosen_opt].to_scene.clone();
        state.scene_i = scene_index_from_tag(&scenes, &state.scene_tag);
    }
}
