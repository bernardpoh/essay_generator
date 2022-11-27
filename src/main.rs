use essay_generator::generate_essay;
use essay_generator::get_pairs;

use std::fs;
fn main() {
    let sample = fs::read_to_string("sample.txt").expect("no 'sample.txt' found");
    //println!("{}", sample);
    let new_essay = generate_essay(sample, "Christmas", 200);
    println!("{new_essay}");

    // let sample = fs::read_to_string("sample.txt").expect("no 'sample.txt' found");
    // let x = get_pairs(&sample);
    // println!("{:#?}",x)
}
