
pub fn rotword(word: &[u8; 4]) -> [u8; 4] {
    let mut new_word: [u8; 4] = [0; 4];
    new_word[0] = word[1];
    new_word[1] = word[2];
    new_word[2] = word[3];
    new_word[3] = word[0];
    new_word
}