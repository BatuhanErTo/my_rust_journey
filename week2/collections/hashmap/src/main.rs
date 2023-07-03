use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("GS"),82);
    scores.insert(String::from("FB"),22);

    let gs_score = scores.get(&String::from("GS"));
    println!("{:?}",gs_score);
    println!("{:?}",scores);

    scores.remove(&String::from("FB"));
    println!("{:?}",scores);

    for (k,v) in &scores{
        println!("{}: {}", k, v);
    }
}
