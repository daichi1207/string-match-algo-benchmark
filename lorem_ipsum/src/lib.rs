use rand::{seq::SliceRandom, thread_rng};
use std::{collections::HashSet, fs};

pub fn generate(n: usize) -> Result<String, std::io::Error> {
    println!("n = {}", n);
    let contents = fs::read_to_string("loremipsum.txt")?;
    let words: Vec<&str> = contents.split_whitespace().collect();
    let mut rng = rand::thread_rng();
    let mut result = String::new();

    for _ in 0..n {
        if let Some(word) = words.choose(&mut rng) {
            result.push_str(word);
            result.push(' ');
        }
    }
    Ok(result)
}

pub fn generate_unique(n: usize) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("loremipsum.txt")?;
    let mut words: Vec<&str> = contents.split_whitespace().collect();
    let mut rng = thread_rng();
    let mut used_words = HashSet::new();
    let mut lorem = String::new();
    words.shuffle(&mut rng);
    for word in words {
        if used_words.contains(word) {
            continue;
        }
        lorem.push_str(word);
        lorem.push(' ');
        used_words.insert(word);
        if used_words.len() == n {
            break;
        }
    }
    if used_words.len() < n {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Not enough unique words in file",
        ));
    }

    Ok(lorem)
}
