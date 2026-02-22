should a dfa be parametrized by its alphabet?
need a way to "type check" the dfa
could define a type struct TransitionFunction (i32, char, i32)?
return Result<Self, Error> instead of asserting in dfa constructor
add a fn to dfa impl to incrementally create tfn
use a list slice or something instead of recreating State objects for final state? or annotate state with is_final?
