use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut counters = HashMap::new();
    for word in magazine {
        *counters.entry(*word).or_insert(0) += 1;
    }
    for word in note {
        *counters.entry(*word).or_insert(0) -= 1;
    }
    counters.values().all(|count| *count >= 0) 
}