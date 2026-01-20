use rand::Rng;

pub fn apply_glitch(text: &str) -> String {
    let mut rng = rand::rng();
    let glitch_chars: Vec<char> = "░▒▓█@#$%&*".chars().collect();
    text.chars()
        .map(|c| {
            if rng.random_bool(0.1) {
                let idx = rng.random_range(0..glitch_chars.len());
                glitch_chars[idx]
            } else {
                c
            }
        })
        .collect()
}
