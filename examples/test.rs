extern crate tiny_ini;
use tiny_ini::Ini;

fn main() {
    let config = Ini::from_file("./example.ini");
    println!("conif = {:?}", config);
    let n1: u32 = config.get("section_one", "name1", "0");
    println!("[section_one][name1] = {}", n1);
}
