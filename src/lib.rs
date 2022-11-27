use rand::Rng;
use std::collections::HashMap;

pub fn get_pairs<'a>(sample: &'a String) -> HashMap<&'a str, HashMap<&'a str, u32>> {
    let mut pairs = HashMap::new();
    let mut sample = sample.split_whitespace();
    let mut prev_word = sample.next().expect("the sample is empty");
    for word in sample {
        let count = pairs
            .entry(prev_word)
            .or_insert(HashMap::new())
            .entry(word)
            .or_insert(0);
        *count += 1;
        prev_word = word;
    }
    pairs
}

fn find_next_word<'a>(
    prev_word: &str,
    pairs: &HashMap<&str, HashMap<&'a str, u32>>,
    all_words: &Vec<&'a str>,
) -> &'a str {
    let mut rng = rand::thread_rng();

    let possible_words = pairs.get(&prev_word);

    match possible_words {
        None => return all_words[rng.gen_range(0..pairs.len())],
        Some(possibilites) => {
            let mut lookup = Vec::new();

            for (k, v) in possibilites {
                for _ in 0..*v {
                    lookup.push(k)
                }
            }
            return lookup[rng.gen_range(0..lookup.len())];
        }
    }
}

pub fn generate_essay(sample: String, prompt: &str, length: u32) -> String {
    let all_words: Vec<&str> = sample.split_whitespace().collect();
    let pairs = get_pairs(&sample);
    let mut essay = String::new();

    let mut current_word = prompt;
    for _ in 0..length {
        essay.push_str(current_word);
        essay.push(' ');
        current_word = find_next_word(current_word, &pairs, &all_words)
    }
    essay.push_str(current_word);

    // while essay.chars().last().unwrap() != '.' {
    //     essay.push(' ');
    //     essay.push_str(current_word);
    //     current_word = find_next_word(current_word, &pairs, &all_words)
    // }
    essay
}
