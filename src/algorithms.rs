use std::collections::{HashMap, HashSet, VecDeque};

use crate::dfa::{DFA, State};

pub fn minimize_dfa(dfa: &DFA) {
    let minimized_dfa = dfa.clone();
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

    // precompute reverse transition map: for each ((q0, q1), a) map it to {(qi, qj) | tfn(qi, a) = q0 and tfn(qj, a) = q1}
    let mut inverse_transition_map: HashMap<(State, State), HashSet<(State, State)>> =
        HashMap::new();
    for &c in &dfa.alphabet {
        for &s1 in &dfa.states {
            for &s2 in &dfa.states {
                if s1 > s2 {
                    continue;
                }
                let &t1 = dfa.tfn.get(&(s1, c)).unwrap();
                let &t2 = dfa.tfn.get(&(s2, c)).unwrap();
                if t1 > t2 {
                    continue;
                }
                inverse_transition_map
                    .entry((t1, t2))
                    .or_default()
                    .insert((s1, s2));
            }
        }
    }

    let mut work_queue: VecDeque<(State, State)> = VecDeque::new();
    let mut distinguishable: HashMap<(State, State), bool> = HashMap::new();

    for &s1 in &dfa.states {
        for &s2 in &dfa.states {
            if s1 >= s2 {
                continue;
            }
            if dfa.accept.contains(&s1) && !dfa.accept.contains(&s2) {
                distinguishable.insert((s1, s2), true);
                work_queue.push_back((s1, s2));
            } else if dfa.accept.contains(&s2) && !dfa.accept.contains(&s1) {
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

    distinguishable
        .iter()
        // .filter(|&(_, v)| *v)
        // .collect::<HashMap<_, _>>()
        // .iter()
        .for_each(|(&k, &v)| {
            if v {
                println!("DIST {}, {}", k.0, k.1)
            } else {
                println!("INDIST {}, {}", k.0, k.1)
            }
        });
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
}
