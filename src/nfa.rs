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
        if !(start < states) {
            return Err(NFATypeError::InvalidStartState);
        }
        if !(accept.iter().all(|&s| s < states)) {
            return Err(NFATypeError::InvalidAcceptState);
        }
        if alphabet.contains(&EPSILON) {
            return Err(NFATypeError::ReservedCharacterInAlphabet);
        }
        if !tfn
            .keys()
            .all(|&(s, c)| s < states && (c == EPSILON || alphabet.contains(&c)))
        {
            return Err(NFATypeError::InvalidTransitionFunction);
        }
        if !tfn.values().all(|&s| s < states) {
            return Err(NFATypeError::InvalidTransitionFunction);
        }
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
        let mut alphabet_with_epsilon = alphabet.clone();
        alphabet_with_epsilon.insert(EPSILON);

        let nfa = Self {
            states,
            start,
            accept,
            alphabet: alphabet_with_epsilon,
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

    #[test]
    fn invalid_start_state_fails() {
        let bad_nfa = NFA::new(0, 0, HashSet::new(), HashSet::new(), HashMap::new());
        assert!(matches!(bad_nfa, Err(NFATypeError::InvalidStartState)));
    }

    #[test]
    fn invalid_accept_state_fails() {
        let bad_nfa = NFA::new(1, 0, HashSet::from([1]), HashSet::new(), HashMap::new());
        assert!(matches!(bad_nfa, Err(NFATypeError::InvalidAcceptState)));
    }

    #[test]
    fn invalid_transition_fn_bad_state_in_domain_fails() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 1);
        tfn.insert((1, '0'), 0);
        tfn.insert((1, '1'), 0);

        tfn.insert((2, '0'), 1);
        let bad_nfa = NFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_nfa,
            Err(NFATypeError::InvalidTransitionFunction)
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
        let bad_nfa = NFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_nfa,
            Err(NFATypeError::InvalidTransitionFunction)
        ));
    }

    #[test]
    fn invalid_transition_fn_bad_state_in_range_fails() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 2);
        tfn.insert((1, '0'), 0);
        tfn.insert((1, '1'), 0);
        let bad_nfa = NFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_nfa,
            Err(NFATypeError::InvalidTransitionFunction)
        ));
    }
}
