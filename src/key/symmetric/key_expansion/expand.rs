use super::subword::subword;
use super::rotword::rotword;
use super::rcon::rcon;

pub fn expand_key(key: &[u8; 16]) -> [[u8; 16]; 11]{
    let mut expanded_key = [[0u8;16]; 11];
    expanded_key[0] = key.clone();

    for i in 1..11 {
        expanded_key[i] = get_next_key(&expanded_key[i-1], i as u8).clone();
    }

    expanded_key
}

// Split 16 byte key into 4 4-byte words
fn split_key(key: &[u8; 16]) -> [[u8; 4]; 4]{
    let mut words = [[0u8; 4]; 4];
    for i in 0..4{
        for j in 0..4{
            words[i][j] = key[i*4 + j];
        }
    }
    words
}

// Get next round key
fn get_next_key(old_key: &[u8; 16], round: u8) -> [u8; 16]{
    let old_split_key = split_key(&old_key);
    let mut new_split_key = [[0u8; 4]; 4];

    // get the first word by using the functions
    new_split_key[0] = get_first_word(&old_split_key[3], &old_split_key[0], round);

    // get the other 3 words
    for i in 1..4{
        // xor the previous word with the word 4 places back
        // must xor byte by byte
        for j in 0..4{
            new_split_key[i][j] = old_split_key[i][j] ^ new_split_key[i-1][j];
        }
    }

    // reassemble key
    reassemble_key(&new_split_key)
}

// Converts 4 4-byte words into a 16-byte key
fn reassemble_key(key: &[[u8; 4]; 4]) -> [u8; 16]{
    let mut result = [0u8; 16];
    for i in 0..4{
        for j in 0..4{
            result[i*4 + j] = key[i][j];
        }
    }
    result
}

fn xor_words(word1: &[u8; 4], word2: &[u8; 4]) -> [u8; 4]{
    let mut result = [0u8; 4];
    for i in 0..4{
        result[i] = word1[i] ^ word2[i];
    }
    result
}

// Get first word of the next round key
fn get_first_word(previous_word: &[u8;4], first_word: &[u8;4], round: u8) -> [u8; 4]{
    let mut result = [0u8; 4];
    // Rotate last word of previous key
    result = rotword(&previous_word);
    // Substitute bytes using subbox
    result = subword(&result);
    // xor with first word of previous key
    result = xor_words(&result, &first_word);
    // xor with rcon
    result[0] = result[0] ^ rcon(round);
    result
}