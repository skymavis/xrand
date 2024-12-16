const N: usize = 25;
const M: usize = 7;
const S: usize = 7;
const T: usize = 15;
const A: u32 = 0x8ebfd028;
const B: u32 = 0x2b5b2500;
const C: u32 = 0xdb8b0000;

pub struct TGFSR {
    x: [u32; N],
    k: usize,
    initialized: bool,
}

impl TGFSR {
    pub fn new(seed: u32) -> Self {
        let mut tgfsr = TGFSR {
            x: [0; N],
            k: 0,
            initialized: false,
        };
        tgfsr.initialize(seed);
        tgfsr
    }

    fn initialize(&mut self, seed: u32) {
        let mut seed = seed;
        for i in 0..N {
            self.x[i] = seed & 0xffffffff;
            seed = (seed.wrapping_mul(1313).wrapping_add(88897)) & 0xffffffff;
        }
        self.k = N - 1;
        self.initialized = true;
    }

    fn iterate(&mut self) {
        for i in 0..N - M {
            self.x[i] = self.x[i + M] ^ (self.x[i] >> 1) ^ if self.x[i] & 1 != 0 { A } else { 0 };
        }
        for i in N - M..N {
            self.x[i] =
                self.x[i + M - N] ^ (self.x[i] >> 1) ^ if self.x[i] & 1 != 0 { A } else { 0 };
        }
    }

    fn next(&mut self) -> u32 {
        if !self.initialized {
            self.initialize(1);
        }
        self.k += 1;
        if self.k == N {
            self.iterate();
            self.k = 0;
        }
        let mut y = self.x[self.k] ^ ((self.x[self.k] << S) & B);
        y ^= (y << T) & C;
        y & 0xffffffff
    }

    pub fn random(&mut self) -> i32 {
        (self.next() & 0x7fffffff) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let tgfsr = TGFSR::new(12345);
        assert!(tgfsr.initialized);
    }

    #[test]
    fn test_random_number_bounds() {
        let mut tgfsr = TGFSR::new(12345);
        let random_number = tgfsr.random();
        assert!(random_number >= 0);
        assert!(random_number <= 0x7fffffff);
    }

    #[test]
    fn test_consecutive_numbers_different() {
        let mut tgfsr = TGFSR::new(12345);
        let first_random = tgfsr.random();
        let second_random = tgfsr.random();
        assert_ne!(first_random, second_random);
    }

    #[test]
    fn test_consistent_results_same_seed() {
        let mut tgfsr1 = TGFSR::new(12345);
        let mut tgfsr2 = TGFSR::new(12345);

        let numbers1: Vec<i32> = (0..10).map(|_| tgfsr1.random()).collect();
        let numbers2: Vec<i32> = (0..10).map(|_| tgfsr2.random()).collect();

        assert_eq!(numbers1, numbers2);
    }

    #[test]
    fn test_multiple_iterations() {
        let mut tgfsr = TGFSR::new(12345);
        let numbers: Vec<i32> = (0..100).map(|_| tgfsr.random()).collect();
        assert_eq!(numbers.len(), 100);
    }
}
