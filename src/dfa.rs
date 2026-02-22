use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(PartialEq, Hash, Eq)]
pub struct State(usize);

pub struct DFA {
    // states
    // should this be a set of concrete objects? or just a number? we dont care about naming each state if we can find a way to make
    // the transition function work?
    states: HashSet<State>,
    start: State,
    accept: HashSet<State>,
    alphabet: HashSet<char>,
    tfn: HashMap<(State, char), State>,
}

impl DFA {
    fn new(
        states: usize,
        start: usize,
        accept: HashSet<usize>,
        alphabet: HashSet<char>,
        tfn: HashMap<(usize, char), usize>,
    ) -> Self {
        assert!(start < states);
        assert!(accept.iter().all(|&s| s < states));
        assert!(
            (0..states)
                .cartesian_product(alphabet.iter())
                .all(|(s, &a)| tfn.contains_key(&(s, a)))
        );
        let states: HashSet<State> = HashSet::from_iter((0..states).map(|s| State(s)));
        let start = State(start);
        let accept = accept.into_iter().map(|s| State(s)).collect();
        let tfn: HashMap<(State, char), State> = tfn
            .into_iter()
            .map(|(k, v)| ((State(k.0), k.1), State(v)))
            .collect();
        Self {
            states,
            start,
            accept,
            alphabet,
            tfn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_dfa() {
        let dfa = DFA::new(1, 0, HashSet::new(), HashSet::new(), HashMap::new());
        let _ = dfa;
        assert!(true)
    }
}
