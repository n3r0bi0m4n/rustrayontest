use crate::Parser;
use simd_json;
use simd_json::OwnedValue;

impl Parser {
    pub fn parse(&mut self, json: String) {
        let mut queue = self.queue.write().unwrap();
        queue.push_back(json);
    }
}
