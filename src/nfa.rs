use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
pub struct State(usize);

#[derive(Debug)]
pub enum NFATypeError {
    InvalidStartState,
    InvalidAcceptState,
    InvalidTransitionFunction,
    ReservedCharacterInAlphabet,
}

const EPSILON: char = '~';

pub struct NFA {
    states: HashSet<State>,
    start: State,
    accept: HashSet<State>,
    alphabet: HashSet<char>,
    tfn: HashMap<(State, char), State>,
}

impl NFA {
    fn validate_nfa(
        states: usize,
        start: usize,
        accept: &HashSet<usize>,
        alphabet: &HashSet<char>,
        tfn: &HashMap<(usize, char), usize>,
    ) -> Result<(), NFATypeError> {
        Ok(())
    }

    pub fn new(
        states: usize,
        start: usize,
        accept: HashSet<usize>,
        alphabet: HashSet<char>,
        tfn: HashMap<(usize, char), usize>,
    ) -> Result<Self, NFATypeError> {
        Self::validate_nfa(states, start, &accept, &alphabet, &tfn)?;

        let states: HashSet<State> = HashSet::from_iter((0..states).map(|s| State(s)));
        let start = State(start);
        let accept = accept.into_iter().map(|s| State(s)).collect();
        let tfn: HashMap<(State, char), State> = tfn
            .into_iter()
            .map(|(k, v)| ((State(k.0), k.1), State(v)))
            .collect();

        let nfa = Self {
            states,
            start,
            accept,
            alphabet,
            tfn,
        };

        Ok(nfa)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_nfa_succeeds() {
        let _ = NFA::new(1, 0, HashSet::new(), HashSet::new(), HashMap::new()).unwrap();
        assert!(true);
    }
}
