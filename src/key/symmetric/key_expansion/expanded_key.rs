use super::expand::expand_key;

pub struct ExpandedKey {
    expanded_key: [[u8; 16]; 11]
}

impl ExpandedKey {
    pub fn new(key: &[u8; 16]) -> ExpandedKey{
        ExpandedKey {
            expanded_key: expand_key(&key).clone()
        }
    }

    pub fn get(&self, index: usize) -> [u8; 16]{
        self.expanded_key[index]
    }
}