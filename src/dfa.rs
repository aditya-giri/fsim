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
        assert!(true);
    }

    #[test]
    fn invalid_start_state_fails() {
        let bad_dfa = DFA::new(0, 0, HashSet::new(), HashSet::new(), HashMap::new());
        assert!(matches!(bad_dfa, Err(DFATypeError::InvalidStartState)));
    }

    #[test]
    fn invalid_accept_state_fails() {
        let bad_dfa = DFA::new(1, 0, HashSet::from([1]), HashSet::new(), HashMap::new());
        assert!(matches!(bad_dfa, Err(DFATypeError::InvalidAcceptState)));
    }

    #[test]
    fn non_total_transition_fn_fails() {
        // use a DFA that accepts strings of even length
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 1);
        tfn.insert((1, '0'), 0);

        let bad_dfa = DFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_dfa,
            Err(DFATypeError::NonTotalTransitionFunction)
        ));
    }

    #[test]
    fn invalid_transition_fn_bad_state_in_domain_fails() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 1);
        tfn.insert((1, '0'), 0);
        tfn.insert((1, '1'), 0);

        tfn.insert((2, '0'), 1);
        let bad_dfa = DFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_dfa,
            Err(DFATypeError::InvalidTransitionFunction)
        ));
    }

    #[test]
    fn invalid_transition_fn_bad_alphabet_in_domain_fails() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 1);
        tfn.insert((1, '0'), 0);
        tfn.insert((1, '1'), 0);
        tfn.insert((0, '2'), 0);
        let bad_dfa = DFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_dfa,
            Err(DFATypeError::InvalidTransitionFunction)
        ));
    }

    #[test]
    fn invalid_transition_fn_bad_state_in_range_fails() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 2);
        tfn.insert((1, '0'), 0);
        tfn.insert((1, '1'), 0);
        let bad_dfa = DFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_dfa,
            Err(DFATypeError::InvalidTransitionFunction)
        ));
    }
}
