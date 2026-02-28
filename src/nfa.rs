use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct State(usize);

#[derive(Debug)]
pub enum NFATypeError {
    InvalidStartState,
    InvalidAcceptState,
    InvalidTransitionFunction,
    ReservedCharacterInAlphabet,
}

pub enum SimulationResult {
    Accepted,
    Rejected,
}

#[derive(Debug)]
pub enum InputError {
    InvalidSymbol,
}

const EPSILON: char = '~';

pub struct NFA {
    states: HashSet<State>,
    start: State,
    accept: HashSet<State>,
    alphabet: HashSet<char>,
    tfn: HashMap<(State, char), HashSet<State>>,
}

impl NFA {
    fn validate_nfa(
        states: usize,
        start: usize,
        accept: &HashSet<usize>,
        alphabet: &HashSet<char>,
        tfn: &HashMap<(usize, char), HashSet<usize>>,
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
        if !tfn.values().all(|st| st.iter().all(|&s| s < states)) {
            return Err(NFATypeError::InvalidTransitionFunction);
        }
        Ok(())
    }

    pub fn new(
        states: usize,
        start: usize,
        accept: HashSet<usize>,
        alphabet: HashSet<char>,
        tfn: HashMap<(usize, char), HashSet<usize>>,
    ) -> Result<Self, NFATypeError> {
        Self::validate_nfa(states, start, &accept, &alphabet, &tfn)?;

        let states: HashSet<State> = HashSet::from_iter((0..states).map(|s| State(s)));
        let start = State(start);
        let accept = accept.into_iter().map(|s| State(s)).collect();
        let tfn: HashMap<(State, char), HashSet<State>> = tfn
            .into_iter()
            .map(|(k, v)| ((State(k.0), k.1), v.into_iter().map(|s| State(s)).collect()))
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

    fn validate_input(&self, input: &String) -> Result<(), InputError> {
        if input.chars().all(|c| self.alphabet.contains(&c)) {
            return Ok(());
        }
        Err(InputError::InvalidSymbol)
    }

    fn epsilon_closure(&self, states: &HashSet<State>) -> HashSet<State> {
        let mut closure = states.clone();
        let mut worklist: VecDeque<State> = states.iter().cloned().collect();

        while let Some(s) = worklist.pop_front() {
            if let Some(nexts) = self.tfn.get(&(s, EPSILON)) {
                for &next in nexts {
                    if closure.insert(next) {
                        // discovered a new state
                        worklist.push_back(next);
                    }
                }
            }
        }
        closure
    }

    pub fn simulate(&self, input: &String) -> Result<SimulationResult, InputError> {
        self.validate_input(input)?;
        // TODO: understand better what is going on here. is self.start moved? cloned? what happens in the loop?
        let mut current_states = HashSet::from([self.start]);
        for s in input.chars() {
            let epsilon_closure = self.epsilon_closure(&current_states);
            let mut new_states: HashSet<State> = HashSet::new();
            for current_state in epsilon_closure {
                let new_states_for_current_state = self.tfn.get(&(current_state, s));
                match new_states_for_current_state {
                    Some(s) => new_states.extend(s),
                    None => (),
                }
            }
            current_states = new_states;
        }
        let final_epsilon_closure = self.epsilon_closure(&current_states);
        let y: HashSet<&State> = self.accept.intersection(&final_epsilon_closure).collect();
        if y.is_empty() {
            return Ok(SimulationResult::Rejected);
        }
        Ok(SimulationResult::Accepted)
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
        tfn.insert((0, '0'), HashSet::from([1]));
        tfn.insert((0, '1'), HashSet::from([1]));
        tfn.insert((1, '0'), HashSet::from([0]));
        tfn.insert((1, '1'), HashSet::from([0]));

        tfn.insert((2, '0'), HashSet::from([1]));
        let bad_nfa = NFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_nfa,
            Err(NFATypeError::InvalidTransitionFunction)
        ));
    }

    #[test]
    fn invalid_transition_fn_bad_alphabet_in_domain_fails() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), HashSet::from([1]));
        tfn.insert((0, EPSILON), HashSet::from([1]));
        tfn.insert((1, '0'), HashSet::from([0]));
        tfn.insert((1, '1'), HashSet::from([0]));
        tfn.insert((0, '2'), HashSet::from([0]));
        let bad_nfa = NFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_nfa,
            Err(NFATypeError::InvalidTransitionFunction)
        ));
    }

    #[test]
    fn invalid_transition_fn_bad_state_in_range_fails() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), HashSet::from([1]));
        tfn.insert((0, '1'), HashSet::from([2]));
        tfn.insert((1, '0'), HashSet::from([0]));
        tfn.insert((1, '1'), HashSet::from([0]));
        let bad_nfa = NFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn);
        assert!(matches!(
            bad_nfa,
            Err(NFATypeError::InvalidTransitionFunction)
        ));
    }

    #[test]
    fn epsilon_closure_is_correct() {
        let mut tfn = HashMap::new();
        tfn.insert((0, EPSILON), HashSet::from([1]));
        tfn.insert((0, '1'), HashSet::from([1]));
        tfn.insert((1, EPSILON), HashSet::from([2, 3]));
        tfn.insert((1, '1'), HashSet::from([0]));
        let nfa = NFA::new(4, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn).unwrap();

        let ec0 = nfa.epsilon_closure(&HashSet::from([State(0)]));
        let ec1 = nfa.epsilon_closure(&HashSet::from([State(1)]));
        let ec2 = nfa.epsilon_closure(&HashSet::from([State(2)]));

        assert_eq!(ec0, HashSet::from([State(0), State(1), State(2), State(3)]));
        assert_eq!(ec1, HashSet::from([State(1), State(2), State(3)]));
        assert_eq!(ec2, HashSet::from([State(2)]));
    }

    #[test]
    fn simulate_fails_on_invalid_input() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), HashSet::from([1]));
        tfn.insert((0, '1'), HashSet::from([1]));
        tfn.insert((1, '0'), HashSet::from([0]));
        tfn.insert((1, '1'), HashSet::from([0]));
        let nfa = NFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn).unwrap();

        let input = String::from("00a11");

        let sim = nfa.simulate(&input);
        assert!(matches!(sim, Err(InputError::InvalidSymbol)));
    }

    #[test]
    fn simulate_accepts_string_ending_with_11() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), HashSet::from([0]));
        tfn.insert((0, '1'), HashSet::from([0, 1]));
        tfn.insert((1, '1'), HashSet::from([2]));
        let nfa = NFA::new(3, 0, HashSet::from([2]), HashSet::from(['0', '1']), tfn).unwrap();

        let input = String::from("0011");

        let sim = nfa.simulate(&input);
        assert!(matches!(sim, Ok(SimulationResult::Accepted)));
    }

    #[test]
    fn simulate_rejects_strings_not_ending_in_11() {
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), HashSet::from([0]));
        tfn.insert((0, '1'), HashSet::from([0, 1]));
        tfn.insert((1, '1'), HashSet::from([2]));
        let nfa = NFA::new(3, 0, HashSet::from([2]), HashSet::from(['0', '1']), tfn).unwrap();

        let sim = nfa.simulate(&String::from("0000"));
        assert!(matches!(sim, Ok(SimulationResult::Rejected)));
        let sim = nfa.simulate(&String::from("0001"));
        assert!(matches!(sim, Ok(SimulationResult::Rejected)));
        let sim = nfa.simulate(&String::from("0010"));
        assert!(matches!(sim, Ok(SimulationResult::Rejected)));
        let sim = nfa.simulate(&String::from(""));
        assert!(matches!(sim, Ok(SimulationResult::Rejected)));
    }
}
