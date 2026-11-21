use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use rand::distr::{Alphanumeric, SampleString};

pub trait CodeGenerator: Send + Sync {
    fn generate(&self) -> String;
}

pub struct Base62Generator;

impl CodeGenerator for Base62Generator {
    fn generate(&self) -> String {
        Alphanumeric.sample_string(&mut rand::rng(), 7)
    }
}

pub struct SequenceGenerator {
    codes: Mutex<VecDeque<String>>,
    counter: AtomicUsize,
}

impl SequenceGenerator {
    pub fn new(codes: Vec<String>) -> Self {
        Self {
            codes: Mutex::new(codes.into()),
            counter: AtomicUsize::new(0),
        }
    }
}

impl CodeGenerator for SequenceGenerator {
    fn generate(&self) -> String {
        if let Some(code) = self.codes.lock().unwrap().pop_front() {
            return code;
        }
        let next = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("s{:06}", next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base62_codes_have_seven_alphanumeric_characters() {
        let code = Base62Generator.generate();
        assert_eq!(code.len(), 7);
        assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn sequence_generator_uses_provided_codes_then_falls_back() {
        let generator = SequenceGenerator::new(vec!["aaaaaaa".to_string()]);
        assert_eq!(generator.generate(), "aaaaaaa");
        assert_eq!(generator.generate(), "s000000");
        assert_eq!(generator.generate(), "s000001");
    }
}
