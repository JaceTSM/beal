use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::mem::size_of_val;
use byte_unit::Byte;


#[derive(Debug, Clone, PartialEq)]
pub struct PowerNumber {
    pub base: u64,
    pub exponent: u64,
    pub result: u64,
}

impl PowerNumber {
    fn new(base: u64, exponent: u64, result: u64) -> PowerNumber {
        PowerNumber { base, exponent, result}
    }
}

#[derive(Debug)]
pub struct Solution {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Debug)]
pub struct SolutionSet {
    pub solutions: Vec<Solution>,
    pub limit_breaks: u64,
    pub inner_loop_count: u64,
    pub continue_count: u64,
}


pub fn run_beal_analysis(limit: u64) -> SolutionSet {
    let start = Instant::now();
    let power_numbers = list_power_numbers(limit);
    let end = start.elapsed();
    // for pn in &power_numbers {
    //     println!("{:?}", pn);
    // }
    println!("Total power numbers below {}: {}", limit, power_numbers.len());
    println!("Time to calculate: {} ms", end.as_millis());
    let bytes = Byte::from_bytes(size_of_val(&*power_numbers).try_into().unwrap());
    let adjusted_unit = bytes.get_appropriate_unit(false);
    println!("Memory usage of power_numbers: {}", adjusted_unit.to_string());

    let start = Instant::now();
    let solution_set = scan_for_solutions(power_numbers, limit);
    let end = start.elapsed();
    println!("Total Beal solutions: {}", solution_set.solutions.len());
    println!("Time to find Beal solutions: {} ms", end.as_millis());
    // for solution in solution_set.solutions {
    //     println!("{}^{} + {}^{} = {}^{}", solution.a, solution.x, solution.b, solution.y, solution.c, solution.z);
    // }
    for solution in solution_set.solutions.iter() {
        match check_common_factors(solution) {
            None => println!("{:?} => NO COMMON FACTOR!", solution),
            Some(factor) => println!("{:?} => {}", solution, factor),
        }
    }
    solution_set
}

fn list_power_numbers(limit: u64) -> Vec<PowerNumber> {
    // 1^n is the same for all n, use 1^3 as the base case
    let power_number_one = PowerNumber::new(1, 3, 1);
    let mut power_numbers: Vec<PowerNumber> = vec![power_number_one];
    let max_base = (limit as f64).powf(1.0/3.0) as u64 + 1;  // TODO: determine if this is safe for 100T
    for base in 2..max_base {
        for exponent in 3..limit {
            let result = base.pow(exponent as u32);
            if result > limit {
                break
            }
            power_numbers.push(PowerNumber::new(base, exponent, result))
        }
    }
    power_numbers
}

fn scan_for_solutions(power_numbers: Vec<PowerNumber>, limit: u64) -> SolutionSet {
    let result_map: HashMap<u64, &PowerNumber> = power_numbers
        .iter()
        .map(|pn| (pn.result, pn))
        .collect();

    let clone_start = Instant::now();
    let mut sorted_power_numbers = power_numbers.clone();
    let clone_time = clone_start.elapsed();
    let sort_start = Instant::now();
    sorted_power_numbers.sort_by_key(|pn| pn.result);
    let sort_time = sort_start.elapsed();
    println!("Clone time: {} | Sort time: {} ms", clone_time.as_millis(), sort_time.as_millis());

    let mut inner_loop_count = 0;
    let mut continue_count = 0;
    let mut limit_breaks = 0;
    let mut solutions: Vec<Solution> = vec![];
    'outer: for pn1 in sorted_power_numbers.iter() {
        'inner: for pn2 in sorted_power_numbers.iter() {
            inner_loop_count += 1;
            let third_term = pn1.result + pn2.result;
            if third_term > limit {
                limit_breaks += 1;
                continue 'outer
            }
            match result_map.get(&third_term) {
                None => {
                    continue_count += 1;
                    continue 'inner
                },
                Some(res) => {
                    let solution = Solution {
                        a: pn1.base,
                        b: pn2.base,
                        c: res.base,
                        x: pn1.exponent,
                        y: pn2.exponent,
                        z: res.exponent,
                    };
                    solutions.push(solution);
                }
            }
        }
    }
    println!(
        "limit_breaks: {} | inner_loop_count: {} | continue_count: {}",
        limit_breaks, inner_loop_count, continue_count
    );
    SolutionSet { solutions, limit_breaks, inner_loop_count, continue_count }
}

fn factorize(n: &u64) -> HashSet<u64> {
    let mut factors: HashSet<u64> = HashSet::new();
    let top_bound = *n / 2;
    for i in 2..=top_bound {
        if n % i == 0 {
            factors.insert(i);
        }
    }
    // A number is always a factor of itself
    factors.insert(*n);
    // println!("Factors of {} are {:?}", n, factors);
    factors
}

fn check_common_factors(solution: &Solution) -> Option<u64> {
    let a_factors = factorize(&solution.a);
    let b_factors = factorize(&solution.b);
    let c_factors = factorize(&solution.c);
    for factor in a_factors {
        if b_factors.contains(&factor) && c_factors.contains(&factor) {
            return Some(factor)
        }
    }
    return None
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_power_numbers() {
        let gen_pns = list_power_numbers(100);
        let test_pns = vec![
            PowerNumber::new(1, 3, 1),
            PowerNumber::new(2, 3, 8),
            PowerNumber::new(2, 4, 16),
            PowerNumber::new(2, 5, 32),
            PowerNumber::new(2, 6, 64),
            PowerNumber::new(3, 3, 27),
            PowerNumber::new(3, 4, 81),
            PowerNumber::new(4, 3, 64),
        ];
        assert_eq!(gen_pns.len(), test_pns.len());
        for pn in test_pns.iter() {
            assert!(gen_pns.contains(pn));
        }
    }

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(&1), HashSet::from([1]));
        assert_eq!(factorize(&2), HashSet::from([2]));
        assert_eq!(factorize(&3), HashSet::from([3]));
        assert_eq!(factorize(&4), HashSet::from([2, 4]));
        assert_eq!(factorize(&5), HashSet::from([5]));
        assert_eq!(factorize(&6), HashSet::from([2, 3, 6]));
        assert_eq!(factorize(&7), HashSet::from([7]));
        assert_eq!(factorize(&8), HashSet::from([2, 4, 8]));
        assert_eq!(factorize(&9), HashSet::from([3, 9]));
        assert_eq!(factorize(&10), HashSet::from([2, 5, 10]));
        assert_eq!(factorize(&84), HashSet::from([2, 3, 4, 6, 7, 12, 14, 21, 28, 42, 84]));
    }

    #[test]
    fn test_check_common_factors_valid() {
        let sample_valid_solutions = vec![
            Solution { a: 2, b: 2, c: 2, x: 3, y: 3, z: 4 },        // 16
            Solution { a: 7, b: 7, c: 14, x: 3, y: 4, z: 3 },       // 2744
            Solution { a: 28, b: 84, c: 28, x: 3, y: 3, z: 4 },     // 614656
            Solution { a: 35, b: 310, c: 435, x: 5, y: 3, z: 3 },   // 82312875
        ];
        for solution in sample_valid_solutions {
            match check_common_factors(&solution) {
                None => {
                    println!("Failed test case: {:?}", solution);
                    panic!("Test cases should all be valid Beal Solutions with common factors.")
                },
                Some(factor) => {
                    assert!(factorize(&solution.a).contains(&factor));
                    assert!(factorize(&solution.b).contains(&factor));
                    assert!(factorize(&solution.c).contains(&factor));
                },
            }
        }
    }

    #[test]
    fn test_check_common_factors_invalid() {
        let sample_invalid_solutions = vec![
            Solution { a: 2, b: 3, c: 5, x: 3, y: 3, z: 3 },
            Solution { a: 7, b: 19, c: 31, x: 3, y: 4, z: 3 },
            Solution { a: 101, b: 113, c: 337, x: 3, y: 3, z: 4 },
            Solution { a: 10, b: 11, c: 13131313, x: 5, y: 3, z: 3 },
        ];
        for solution in sample_invalid_solutions {
            if let Some(factor) = check_common_factors(&solution) {
                println!("Failed test case: {:?} -> Factor = {}", solution, factor);
                panic!("Test cases should not share common factors.");
            }
        }
    }

    #[test]
    fn test_scan_for_solutions() {
        todo!()
    }
}
