use brute_force::brute_force;
use rabin_karp::rabin_karp;
use std::fs;

use lorem_ipsum::generate;
use lorem_ipsum::generate_unique;

fn main() {
    let text = generate(300).unwrap();
    // let text = fs::read_to_string("pi.txt").unwrap();
    let pattern = "lorem";
    let positions = brute_force(&text, &pattern);
    // let positions = rabin_karp(&text, "lorem");

    if positions.is_empty() {
        println!("Pattern not found");
    } else {
        for pos in positions {
            println!("Pattern found at position {}", pos);
        }
    }
}
