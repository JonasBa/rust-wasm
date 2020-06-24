use std::ptr;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct TrieNode {
  key: u8,
  terminal: bool,
  parent: *const TrieNode,
  children: HashMap<u8, TrieNode>
}

impl TrieNode {
  pub fn get_word(&self) -> Vec<u8> {
    let mut word: Vec<u8> = Vec::new();
    let mut node = self;

    while !node.parent.is_null() {
      word.push(node.key);
      unsafe {
        node = &*node.parent;
      }
    }

    word
  }
}

#[wasm_bindgen]
struct Trie {
  root: Option<TrieNode>
}

#[wasm_bindgen]
impl Trie {
  fn insert_root(&mut self, word: &Vec<u8>){
    let first: u8 = *word.first().unwrap();

    let node: TrieNode = TrieNode {
        key: first,
        parent: ptr::null(),
        terminal: false,
        children: HashMap::new()
    };
    
    let root: Option<TrieNode> = {
        Some(node)
    };

    self.root = root;
  }

  #[wasm_bindgen(constructor)]
  pub fn new() -> Trie {
    Trie { root: None }
  }


  #[wasm_bindgen]
  pub fn insert(&mut self, word: Vec<u8>) {
    if self.root.is_none() {
        self.insert_root(&word);
        return
    }

    let mut root= self.root.as_mut().unwrap();

    for (i, char_code) in word.iter().enumerate() {
      if !root.children.contains_key(&char_code) {
        root.children.insert(*char_code, TrieNode {
            key: *char_code,
            parent: root,
            terminal: i == (word.len() - 1),
            children: HashMap::new(),
          });
      }

      root = root.children.get_mut(&char_code).unwrap();
    }
  }

  #[wasm_bindgen]
  pub fn contains(&mut self, word: Vec<u8>) -> bool {
    if self.root.is_none() {
        return false
    }

    let mut node = self.root.as_mut().unwrap();

    for char_code in word.iter() {
        if node.children.contains_key(&char_code) {
            node = node.children.get_mut(&char_code).unwrap()
        } else {
            return false
        }
    }

    return node.terminal
  }

  #[wasm_bindgen]
  pub fn find(&mut self, prefix: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut node = self.root.as_mut().unwrap();
    
    for char in prefix.iter() {
        if node.children.contains_key(&char) {
            node = node.children.get_mut(&char).unwrap()
        } else {
            return output
        }
    }
    
    if node.terminal {
      output = node.get_word();
      return output
    }
    // self.find_all_words(node, output);
    return output;
  }

//   #[wasm_bindgen]
  // pub fn find_all_words(&self, node: &TrieNode, output: Vec<u8>){
  //   if (node.end) {
  //     arr.unshift(node.getWord());
  //   }
    
    // // iterate through each children, call recursive findAllWords
    // for (var child in node.children) {
    //   findAllWords(node.children[child], arr);
    // }
//   }
}
