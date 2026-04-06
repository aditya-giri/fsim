use std::collections::{HashMap, HashSet, VecDeque};

use crate::dfa::{DFA, State};

pub fn minimize_dfa(dfa: &DFA) -> DFA {
    let mut minimized_dfa = dfa.clone();
    // minimization using the table filling algorithm based on the myhill nerode theorem
    // remove unreachable states
    let mut reachable = vec![false; dfa.states.len()];
    let mut work_queue = VecDeque::from([dfa.start]);

    reachable[dfa.start] = true;

    while !work_queue.is_empty() {
        let curr = work_queue.pop_front().unwrap();
        for &sym in &dfa.alphabet {
            let &next = dfa.tfn.get(&(curr, sym)).unwrap();
            if !reachable[next] {
                work_queue.push_back(next);
                reachable[next] = true;
            }
        }
    }

    let mut m_states: HashSet<usize> = dfa.states.clone();
    m_states.retain(|&s| reachable[s]);
    let mut m_tfn = dfa.tfn.clone();
    m_tfn.retain(|k, _| m_states.contains(&k.0));
    let mut m_accept = dfa.accept.clone();
    m_accept.retain(|&s| reachable[s]);
    let m_alphabet = dfa.alphabet.clone();
    let mut m_start = dfa.start;

    // precompute inverse transition map: for each ((q0, q1), a) map it to {(qi, qj) | tfn(qi, a) = q0 and tfn(qj, a) = q1}
    let mut inverse_transition_map: HashMap<(State, State), HashSet<(State, State)>> =
        HashMap::new();
    for (&(p, a), &r) in &m_tfn {
        for (&(q, b), &s) in &m_tfn {
            if a == b && p < q {
                let canonical = if r < s { (r, s) } else { (s, r) };
                inverse_transition_map
                    .entry(canonical)
                    .or_default()
                    .insert((p, q));
            }
        }
    }

    // mark distinguishable states
    let mut work_queue: VecDeque<(State, State)> = VecDeque::new();
    let mut distinguishable: HashMap<(State, State), bool> = HashMap::new();

    for &s1 in &m_states {
        for &s2 in &m_states {
            if s1 >= s2 {
                continue;
            }
            if m_accept.contains(&s1) != m_accept.contains(&s2) {
                distinguishable.insert((s1, s2), true);
                work_queue.push_back((s1, s2));
            } else {
                distinguishable.insert((s1, s2), false);
            }
        }
    }

    while !work_queue.is_empty() {
        let top = work_queue.pop_front().unwrap();
        if let Some(incoming) = inverse_transition_map.get(&top) {
            for &(s1, s2) in incoming {
                if let Some(v) = distinguishable.get_mut(&(s1, s2)) {
                    if !*v {
                        *v = true;
                        work_queue.push_back((s1, s2));
                    }
                }
            }
        }
    }
    // merge indistinguishable states, update tfn and accept and whatever else
    distinguishable.retain(|_, v| !*v); //todo: update var name, this is now tracking indistinguishable states
    let mut indistinguishable = distinguishable.iter().collect::<Vec<_>>();
    indistinguishable.sort_by_key(|(s1, _)| s1.0);

    let mut removed = vec![false; dfa.states.len()];
    for (&(s1, s2), _) in indistinguishable {
        if removed[s2] {
            continue; // early exit
        }
        // always keep s1
        if m_start == s2 {
            m_start = s1;
        }
        m_accept.remove(&s2);
        m_states.remove(&s2);
        m_tfn.retain(|k, _| k.0 != s2);
        for (_, dst) in m_tfn.iter_mut() {
            if *dst == s2 {
                *dst = s1;
            }
        }
        removed[s2] = true;
    }

    minimized_dfa.states = m_states;
    minimized_dfa.start = m_start;
    minimized_dfa.accept = m_accept;
    minimized_dfa.alphabet = m_alphabet;
    minimized_dfa.tfn = m_tfn;

    println!("{}", minimized_dfa);

    minimized_dfa
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dfa::DFA;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_distinguishable_no_redundant_states() {
        // 2-state DFA accepting even-length strings over {0, 1}
        // state 0 (accept) and state 1 are distinguishable
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 1);
        tfn.insert((1, '0'), 0);
        tfn.insert((1, '1'), 0);
        let dfa = DFA::new(2, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn).unwrap();
        minimize_dfa(&dfa); // expect (0, 1) printed as distinguishable
    }

    #[test]
    fn test_distinguishable_with_redundant_states() {
        // 3-state DFA where states 1 and 2 are redundant copies:
        // both non-accepting, both transition to 0 on any symbol
        // expect (0,1) and (0,2) distinguishable, (1,2) indistinguishable
        let mut tfn = HashMap::new();
        tfn.insert((0, '0'), 1);
        tfn.insert((0, '1'), 2);
        tfn.insert((1, '0'), 0);
        tfn.insert((1, '1'), 0);
        tfn.insert((2, '0'), 0);
        tfn.insert((2, '1'), 0);
        let dfa = DFA::new(3, 0, HashSet::from([0]), HashSet::from(['0', '1']), tfn).unwrap();
        minimize_dfa(&dfa); // expect (0,1) and (0,2) distinguishable, (1,2) not printed
    }

    #[test]
    fn test_distinguishable_r_greater_than_s() {
        // 3-state DFA over {a}: state 0 (accept), state 1 (start), state 2
        // δ(0,a)=0, δ(1,a)=2, δ(2,a)=0
        // (1,2) are distinguishable: "a" accepts from 2 but not from 1
        // exposes the r>s bug: for pair (1,2), destinations are (2,0) where r=2 > s=0
        // so (1,2) never gets added to inverse_map[(0,2)] and is incorrectly merged
        let mut tfn = HashMap::new();
        tfn.insert((0, 'a'), 0);
        tfn.insert((1, 'a'), 2);
        tfn.insert((2, 'a'), 0);
        let dfa = DFA::new(3, 1, HashSet::from([0]), HashSet::from(['a']), tfn).unwrap();
        let minimized = minimize_dfa(&dfa);
        assert_eq!(minimized.states.len(), 3); // all states are distinguishable, no merging should occur
    }
}
