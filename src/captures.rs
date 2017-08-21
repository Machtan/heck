//! Capture assignment code.
//! Finds out how many time a 'captured' pattern inside a rule is matched, and
//! find out how many captures and in which order they appear.
use grammar::{Capture, Pat};
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaptureType {
    Single,
    Optional,
    Multiple,
}

#[derive(Debug, Clone)]
pub struct ParserRule {
    pub pat: Pat,
    pub captures: Vec<CaptureType>,
}

#[derive(Debug, Clone)]
struct CaptureState {
    pub single_ids: Vec<usize>,
    pub shared_ids: Vec<usize>, // [#dollars-1 -> idx]
    pub capture_types: Vec<CaptureType>,
}

impl CaptureState {
    fn assign(&mut self, group: Option<usize>, context: CaptureContext) -> usize {
        println!("CaptureState.assign({:?}, {:?})", group, context);
        use self::CaptureContext::*;
        if let Some(group) = group {
            // The group is not the next shared one, and is much larger
            if group > self.shared_ids.len() {
                panic!("Shared id '{}' assigned before the previous one.", group)
            
            // The group is the next shared group
            } else if group == self.shared_ids.len() {
                let id = self.capture_types.len();
                self.shared_ids.push(id);
                let ty = match context {
                    Free => CaptureType::Single,
                    Optional => CaptureType::Optional,
                    Repetition => CaptureType::Multiple,
                };
                self.capture_types.push(ty);
                id
            
            // The group is an already assigned shared group
            } else {
                self.capture_types[group] = CaptureType::Multiple;
                group
            }
        } else {
            let id = self.capture_types.len();
            self.single_ids.push(id);
            let ty = match context {
                Free => CaptureType::Single,
                Optional => CaptureType::Optional,
                Repetition => CaptureType::Multiple,
            };
            self.capture_types.push(ty);
            id
        }
    }
}

/// Maps the capture indices of one 'AnyOf' branch to the indices of another,
/// so that shared index 1 of branch 2 becomes the same as that of branch 1
fn combine_caps(indices: &mut Vec<usize>, types: &mut Vec<CaptureType>,
    oindices: &[usize], otypes: &[CaptureType]) 
{
    use self::CaptureType::*;
    let n1 = indices.len();
    let n2 = oindices.len();
    for i in 0..cmp::min(n1, n2) {
        match otypes[i] {
            Optional => {
                if let Single = types[i] {
                    types[i] = Optional;
                }
            }
            Multiple => {
                types[i] = Multiple;
            }
            _ => {}
        }
    }
    if n1 < n2 {
        for i in n1..n2 {
            indices.push(i);
            match otypes[i] {
                Single | Optional => {
                    types.push(Optional);
                }
                Multiple => {
                    types.push(Multiple);
                }
            }
        }
    } else if n1 > n2 {
        for i in n2..n1 {
            if let Single = types[i] {
                types[i] = Optional;
            }
        }
    }
}

fn reorder_capture_indices(pat: Pat, good: &CaptureState, bad: &CaptureState) -> Pat {
    assert!(good.capture_types.len() >= bad.capture_types.len());
    let mut map = vec![0, good.capture_types.len()];
    for (&bad_idx, &good_idx) in bad.single_ids.iter().zip(&good.single_ids) {
        map[bad_idx] = good_idx;
    }
    for (&bad_idx, &good_idx) in bad.shared_ids.iter().zip(&good.shared_ids) {
        map[bad_idx] = good_idx;
    }
    fn inner(pat: Pat, map: &[usize]) -> Pat {
        use grammar::Pat::*;
        match pat {
            Rule(_) | Token(_) | BreakOnToken(_) => pat,
            Seq(pats) => {
                Seq(pats.into_iter().map(|p| inner(p, map)).collect())
            }
            Cap(Capture::Assigned(idx), boxed) => {
                Cap(Capture::Assigned(map[idx]), Box::new(inner(*boxed, map)))
            }
            Cap(_, _) => panic!("NONONONONONO"),
            Opt(boxed) => {
                Opt(Box::new(inner(*boxed, map)))
            }
            ZeroPlus(boxed) => {
                ZeroPlus(Box::new(inner(*boxed, map)))
            }
            OnePlus(boxed) => {
                OnePlus(Box::new(inner(*boxed, map)))
            }
            Loop(boxed) => {
                Loop(Box::new(inner(*boxed, map)))
            }
            AnyOf(pats) => {
                AnyOf(pats.into_iter().map(|p| inner(p, map)).collect())
            }
        }
    }
    inner(pat, &map)
}

#[derive(Debug, Clone, Copy)]
enum CaptureContext {
    Free,
    Optional,
    Repetition,
}

pub fn find_and_assign_captures(pat: Pat) -> ParserRule {
    fn is_single(pat: &Pat) -> bool {
        match *pat {
            Pat::Token(_) | Pat::Rule(_) => true,
            _ => false,
        }
    }
    
    fn inner(pat: Pat, context: CaptureContext, state: &mut CaptureState) -> Pat {
        use grammar::Pat::*;
        use self::CaptureContext::*;
        match pat {
            Seq(pats) => {
                Seq(pats.into_iter().map(|p| inner(p, context, state)).collect())
            }
            Cap(captype, boxed) => {
                let group = match captype {
                    Capture::Unnamed => None,
                    Capture::Shared(idx) => Some(idx),
                    Capture::Assigned(_) => {
                        panic!("find_and_assign_captures called on a pattern whose captures were already assigned!");
                    }
                };
                let inner_context = match *boxed {
                    Token(_) | Rule(_) => Free,
                    Seq(_) | ZeroPlus(_) | OnePlus(_) | Loop(_) => Repetition,
                    Cap(_, _) => {
                        panic!("Cannot have a capture just inside a capture :/");
                    }
                    Opt(ref opt_pat) => {
                        // Find out what's inside it...
                        if is_single(opt_pat) {
                            Optional
                        } else {
                            Repetition
                        }
                    }
                    AnyOf(ref pats) => {
                        if pats.iter().all(is_single) {
                            Free
                        } else {
                            Repetition
                        }
                    },
                    BreakOnToken(_) => {
                        panic!("Cannot capture 'break on token ( \"token\"! )' pattern!");
                    }
                };
                let actual = match (context, inner_context) {
                    (Repetition, _) => Repetition,
                    (Optional, Free) => Optional,
                    (Optional, Optional) => Optional,
                    (Optional, Repetition) => Repetition,
                    (Free, other) => other,
                };
                
                let id = state.assign(group, actual);
                let new_cap_pat = inner(*boxed, context, state);
                Cap(Capture::Assigned(id), Box::new(new_cap_pat))
            }
            Opt(boxed) => {
                Opt(Box::new(if let CaptureContext::Repetition = context {
                    inner(*boxed, context, state)
                } else {
                    inner(*boxed, CaptureContext::Optional, state)
                }))
            }
            ZeroPlus(boxed) => {
                ZeroPlus(Box::new(inner(*boxed, CaptureContext::Repetition, state)))
            }
            OnePlus(boxed) => {
                OnePlus(Box::new(inner(*boxed, CaptureContext::Repetition, state)))
            }
            Loop(boxed) => {
                Loop(Box::new(inner(*boxed, CaptureContext::Repetition, state)))
            }
            AnyOf(mut pats) => {
                let pre_state = state.clone();
                let mut drained = pats.drain(..);
                let first = inner(drained.next().unwrap(), context, state);
                let mut assigned_pats = vec![first];
                for pat in drained {
                    let mut pat_state = pre_state.clone();
                    let assigned = inner(pat, context, &mut pat_state);
                    
                    
                    // Combine both capture groups, then reorder the indices of the branch pattern
                    // so that they match up.
                    // ie, if path 1 is [0shared, 1single, 2shared, 3single]
                    // and path 2 is [0single, 1shared, 2shared, 3single]
                    // path 2 needs to be changed so that the indices point to
                    // the corresponding single/shared of path 1
                    
                    combine_caps(
                        &mut state.single_ids, 
                        &mut state.capture_types,
                        &pat_state.single_ids, 
                        &pat_state.capture_types
                    );
                    combine_caps(
                        &mut state.shared_ids, 
                        &mut state.capture_types,
                        &pat_state.shared_ids, 
                        &pat_state.capture_types
                    );
                    
                    let mapped = reorder_capture_indices(assigned, state, &pat_state);
                    assigned_pats.push(mapped);
                }
                AnyOf(assigned_pats)
            }
            Token(_) | Rule(_) | BreakOnToken(_) => pat,
        }
    }
    let mut state = CaptureState { 
        single_ids: Vec::new(),
        shared_ids: Vec::new(),
        capture_types: Vec::new(),
    };
    let context = CaptureContext::Free;
    let assigned_pat = inner(pat, context, &mut state);
    ParserRule {
        pat: assigned_pat,
        captures: state.capture_types,
    }
}
