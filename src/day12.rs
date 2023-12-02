extern crate regex;

use regex::Regex;
use std::collections::HashMap;

type RuleSet = HashMap<[u8; 5], u8>;
type State = Vec<u8>;

fn rule_lookup(state: &[u8], ruleset: &RuleSet) -> u8 {
    if state.len() == 5 {
        let _state: [u8; 5] = [state[0], state[1], state[2], state[3], state[4]];
        if let Some(output) = ruleset.get(&_state) {
            *output
        } else {
            panic!("RuleSet missing requested state");
        }
    } else {
        panic!("Can't call lookup on state length n != 5");
    }
}

fn next_state(state: &[u8], ruleset: &RuleSet, cache: &mut HashMap<Vec<u8>, Vec<u8>>) -> Vec<u8> {
    let n = state.len();
    //println!("n: {}", n);

    // First check cache (memoization)
    if let Some(answer) = cache.get(state) {
        return answer.clone();
    }

    // If not already in cache, then do the recursion stuff
    // Check base case
    let mut output;
    match n {
        5 => output = vec![rule_lookup(state, ruleset)],
        6.. => {
            let half_len = n / 2;
            //println!("half_len: {}", half_len);
            if half_len < 3 {
                panic!("we're requiring that half_len >= 3");
            }
            if n - 3 < half_len - 2 {
                panic!("we're requiring that n - 3 < half_len - 2");
            }

            // Divide the problem into 2 (carefully)
            //let left = state[..half_len+2];
            //let right = state[half_len-2..];

            // Recurse on left and right halves
            output = next_state(&state[..half_len + 2], ruleset, cache);
            let mut right_output = next_state(&state[half_len - 2..], ruleset, cache);
            output.append(&mut right_output);
        }
        _ => panic!("n < 5... we can't let this happen"),
    }
    cache.insert(state.to_vec(), output.clone());
    output
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> (State, RuleSet) {
    let re1 = Regex::new(r"initial state: ([\.\#]+)").unwrap();
    let re2 = Regex::new(r"([\.\#]{5}) => ([\.\#])").unwrap();
    let mut pots = Vec::new();
    for cap in re1.captures_iter(input) {
        for c in cap[1].chars() {
            match c {
                '.' => pots.push(0),
                '#' => pots.push(1),
                _ => panic!("WTFBBQ1"),
            }
        }
    }
    let mut rules = HashMap::new();
    for cap in re2.captures_iter(input) {
        let mut pattern = Vec::new();
        for c in cap[1].chars() {
            match c {
                '.' => pattern.push(0),
                '#' => pattern.push(1),
                _ => panic!("WTFBBQ2"),
            }
        }
        let mut output = 0;
        for c in cap[2].chars() {
            match c {
                '.' => output = 0,
                '#' => output = 1,
                _ => panic!("WTFBBQ3"),
            }
        }
        let mut arr = [0; 5];
        arr.copy_from_slice(pattern.as_slice());
        rules.insert(arr, output);
    }
    (pots, rules)
}

fn add_zeros(state: &mut Vec<u8>) -> i64 {
    let mut nzeros = 0;
    if state[0] == 1 {
        nzeros = 4;
    } else if state[1] == 1 {
        nzeros = 3;
    } else if state[2] == 1 {
        nzeros = 2;
    } else if state[3] == 1 {
        nzeros = 1;
    }
    for _ in 0..nzeros {
        state.insert(0, 0);
    }

    let n = state.len();
    let mut nzeros_ = 0;
    if state[n - 1] == 1 {
        nzeros_ = 4;
    } else if state[n - 2] == 1 {
        nzeros_ = 3;
    } else if state[n - 3] == 1 {
        nzeros_ = 2;
    } else if state[n - 4] == 1 {
        nzeros_ = 1;
    }
    for _ in 0..nzeros_ {
        state.push(0);
    }
    nzeros
}

fn trim_zeros(state: &mut Vec<u8>) -> i64 {
    let mut zeros_skipped: i64 = 0;
    while state[0] == 0 {
        state.remove(0);
        zeros_skipped += 1;
    }
    while state[state.len() - 1] == 0 {
        state.pop();
    }
    zeros_skipped
}

pub fn solver(input: &(State, RuleSet), n_iter: usize, part2: bool) -> i64 {
    let mut cache = HashMap::<Vec<u8>, Vec<u8>>::new();
    let mut state: Vec<u8> = input.0.clone();
    let mut startidx = trim_zeros(&mut state);
    let ruleset = input.1.clone();

    let mut prior_states = HashMap::new();
    let mut equalibrium = vec![];

    let mut last_iter = 0;
    for i in 0..n_iter {
        //print_state(&state, startidx);
        let zeros_added = add_zeros(&mut state); // this should always add 4 zeros to start and end
        startidx -= zeros_added;

        // This operation kind of implicitly trims 2 elements on either end of `state`
        //print_state(&state, startidx);
        state = next_state(&state, &ruleset, &mut cache);
        startidx += 2;
        let zeros_skipped = trim_zeros(&mut state);
        startidx += zeros_skipped;

        //print_state(&state, startidx);
        //println!("");

        // Keep track of visited states to cycle check
        equalibrium = state.clone();
        if prior_states.contains_key(&state) {
            //println!("Found a cycle at i: {}", i);
            //println!("Zeros Skipped: {}", zeros_skipped);
            equalibrium = state.clone();
            last_iter = i as i64;
            break;
        } else {
            prior_states.insert(state.clone(), zeros_skipped);
        }
    }

    if part2 {
        let iters_left: i64 = n_iter as i64 - (last_iter + 1);
        startidx += iters_left;
    }

    let num_ones: i64 = equalibrium.iter().map(|&x| x as i64).sum();

    let mut output = 0;
    for (i, &pot) in equalibrium.iter().enumerate() {
        if pot == 1 {
            output += i as i64;
        }
    }

    output += num_ones * startidx;
    output
}

pub fn print_state(state: &State, startidx: i64) {
    for _ in 0..5 + startidx {
        print!(" ");
    }
    for &element in state {
        if element == 0 {
            print!(".");
        } else {
            print!("#");
        }
    }
    println!();
}

#[aoc(day12, part1)]
pub fn part1(input: &(State, RuleSet)) -> i64 {
    solver(input, 20, false)
}

#[aoc(day12, part2)]
pub fn part2(input: &(State, RuleSet)) -> i64 {
    solver(input, 50000000000, true)
}
