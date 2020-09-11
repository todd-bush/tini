extern crate tini;
use tini::Ini;

static INPUT: &'static str = "./examples/example.ini";

fn main() {
    // load ini config from `INPUT` file
    let config = Ini::from_file(INPUT).unwrap();

    // if you are sure
    let name1: i32 = config.get("section_one", "name1").unwrap();

    // if you aren't sure
    let name5: bool = config.get("section_one", "name5").unwrap_or(false); // for non-existing key

    // check
    println!("name1: {}", name1);
    println!("name5: {}", name5);
}
