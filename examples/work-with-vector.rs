extern crate tini;
use tini::Ini;

static INPUT: &'static str = "./examples/example.ini";

fn main() {
    let config = Ini::from_file(INPUT).unwrap();

    // get ints
    let int_v: Vec<u8> = config.get_vec("section_three", "frst1").unwrap();
    println!("frst1 = {:?}", int_v);

    // get floats
    let float_v: Vec<f32> = config.get_vec("section_three", "frst2").unwrap();
    println!("frst2 = {:?}", float_v);

    // get Strings
    let str_a: Vec<String> = config.get_vec("section_three", "frst3").unwrap();
    println!("frst3 = {:?}", str_a);
    let str_b: Vec<String> = config.get_vec("section_three", "frst4").unwrap();
    println!("frst4 = {:?}", str_b);

    // get bools
    let bool_v: Vec<bool> = config.get_vec("section_three", "frst5").unwrap();
    println!("frst5 = {:?}", bool_v);
}
