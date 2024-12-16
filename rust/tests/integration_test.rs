use xrand::TGFSR;

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
