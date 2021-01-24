use std::collections::HashMap;

use ggez::event::KeyCode;

pub struct KeyInputHashMap {
    hashmap: HashMap<KeyCode, bool>,
}
impl KeyInputHashMap {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        for (_k, flag) in self.hashmap.iter_mut() {
            *flag = false;
        }
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        if let Some(pressed) = self.hashmap.get(&key) {
            *pressed
        } else {
            false
        }
    }

    pub fn set_down(&mut self, key: KeyCode) {
        self.hashmap.insert(key, true);
    }

    pub fn set_up(&mut self, key: KeyCode) {
        self.hashmap.insert(key, false);
    }
}
