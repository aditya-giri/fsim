use std::collections::HashSet;

pub struct DFA {
    // states
    // should this be a set of concrete objects? or just a number? we dont care about naming each state if we can find a way to make
    // the transition function work?
    states: i32,
    start: i32,
    accept: HashSet<i32>,
    alphabet: HashSet<char>,
    tfn: HashSet<(i32, char, i32)>,
}

impl DFA {
    fn new(
        states: i32,
        start: i32,
        accept: HashSet<i32>,
        alphabet: HashSet<char>,
        tfn: HashSet<(i32, char, i32)>,
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
        let dfa = DFA::new(1, 0, HashSet::new(), HashSet::new(), HashSet::new());
        let _ = dfa;
        assert!(true)
    }
}
