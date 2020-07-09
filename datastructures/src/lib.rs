mod utils;

use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Queue {
    items: VecDeque<Vec<u8>>
}

#[wasm_bindgen]
impl Queue {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Queue {
        Queue {
            items: VecDeque::new()
        }
    }

    #[wasm_bindgen]
    pub fn enqueue(&mut self, item: Vec<u8>) {
        self.items.push_back(item.clone());
    }

    #[wasm_bindgen]
    pub fn dequeu(&mut self) -> Option<Vec<u8>> {
        let item = self.items.pop_front();
        if item.is_none(){
            return None;
        }

        return item;
    }

    #[wasm_bindgen]
    pub fn peek(&self) -> Option<Vec<u8>> {
        match self.items.clone().get(self.items.len()) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }

    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.items.len()
    }

    #[wasm_bindgen]
    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}
