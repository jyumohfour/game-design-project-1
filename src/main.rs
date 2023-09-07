struct Scene {
    tag: String,
    descs: Vec<Desc>,
    is_end: bool,
    options: Vec<Option>
}

struct Desc {
    text: String,
    min_san: i32,
    san_change: i32
}

struct Option {
    to_scene: String,
    min_san: i32
}

struct Gamestate {
    current_scene: Scene,
    sanity: i32
}

fn main() {
    println!("Hello, world!");
}
