use std::collections::{HashMap, HashSet};

pub struct DFA {
    // states
    // should this be a set of concrete objects? or just a number? we dont care about naming each state if we can find a way to make
    // the transition function work?
    states: usize,
    start: usize,
    accept: HashSet<usize>,
    alphabet: HashSet<char>,
    tfn: HashMap<(usize, char), usize>,
}

impl DFA {
    fn new(
        states: usize,
        start: usize,
        accept: HashSet<usize>,
        alphabet: HashSet<char>,
        tfn: HashMap<(usize, char), usize>,
    ) -> Self {
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
