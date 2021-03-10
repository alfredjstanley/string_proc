use std::collections::HashMap;

fn main() {
    let content = std::fs::read_to_string("dummy_text.txt").unwrap();
    // println!("content is '{}'", content);
    println!("{:?}", process(&content));
}
fn process<'a>(string: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in string.lines() {
        for sentence in line.split_terminator('.') {
            for word in sentence.trim().split_whitespace() {
                if word.chars().next().unwrap().is_uppercase() {
                    result.entry(word).or_default().push(sentence);
                }
            }
        }
    }
    result
}
