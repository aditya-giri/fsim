use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
pub enum DFATypeError {
    InvalidStartState,
    InvalidAcceptState,
    InvalidTransitionFunction,
    NonTotalTransitionFunction,
}

#[derive(PartialEq, Hash, Eq)]
pub struct State(usize);

pub struct DFA {
    states: HashSet<State>,
    start: State,
    accept: HashSet<State>,
    alphabet: HashSet<char>,
    tfn: HashMap<(State, char), State>,
}

impl DFA {
    fn validate(
        states: usize,
        start: usize,
        accept: &HashSet<usize>,
        alphabet: &HashSet<char>,
        tfn: &HashMap<(usize, char), usize>,
    ) -> Result<(), DFATypeError> {
        if !(start < states) {
            return Err(DFATypeError::InvalidStartState);
        }
        if !(accept.iter().all(|&s| s < states)) {
            return Err(DFATypeError::InvalidAcceptState);
        }
        let domain_of_tfn = (0..states).cartesian_product(alphabet.iter());
        if !(domain_of_tfn
            .into_iter()
            .all(|(s, &a)| tfn.contains_key(&(s, a))))
        {
            return Err(DFATypeError::NonTotalTransitionFunction);
        }
        if !(tfn.len() == states * alphabet.len()) || !(tfn.values().all(|&v| v < states)) {
            return Err(DFATypeError::InvalidTransitionFunction);
        }

        Ok(())
    }

    pub fn new(
        states: usize,
        start: usize,
        accept: HashSet<usize>,
        alphabet: HashSet<char>,
        tfn: HashMap<(usize, char), usize>,
    ) -> Result<Self, DFATypeError> {
        Self::validate(states, start, &accept, &alphabet, &tfn)?;

        let states: HashSet<State> = HashSet::from_iter((0..states).map(|s| State(s)));
        let start = State(start);
        let accept = accept.into_iter().map(|s| State(s)).collect();
        let tfn: HashMap<(State, char), State> = tfn
            .into_iter()
            .map(|(k, v)| ((State(k.0), k.1), State(v)))
            .collect();

        let dfa = Self {
            states,
            start,
            accept,
            alphabet,
            tfn,
        };

        Ok(dfa)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_dfa_succeeds() {
        let _ = DFA::new(1, 0, HashSet::new(), HashSet::new(), HashMap::new()).unwrap();
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn bad_dfa_fails() {
        let _ = DFA::new(0, 0, HashSet::new(), HashSet::new(), HashMap::new()).unwrap();
    }
}
