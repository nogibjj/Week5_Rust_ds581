use rand::seq::SliceRandom;

pub fn generate_song_name(words: &[&str]) -> String {
    let mut rng = rand::thread_rng();
    let mut name_vec = Vec::new();
    for _i in 0..2 {
        let available_words: Vec<&str> = words.iter().filter(|&w| !name_vec.contains(w)).cloned().collect();
        if available_words.is_empty() {
            break;
        }
        let word = *available_words.choose(&mut rng).unwrap();
        name_vec.push(word);
    }
    name_vec.join(" ")
}
