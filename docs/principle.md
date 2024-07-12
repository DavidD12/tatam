Un système de transition en Tatam est spécifié à l'aide de différents éléments :

1. des variables et constantes
2. un état initial
3. des invariants
4. des transitions et des triggers
5. une formule LTL
6. une recherche

# Variables and constants
Variables and constants are defined by a unique name and have a type. Variables can change value with each state of the transition system. Constants, on the other hand, have a value that does not change. Variable declarations are preceded by the keyword *var*, while constants are preceded by the keyword *cst*. The order of declaration is irrelevant.

```
var b: Bool
cst x, y: Int
var i: Int
```

# Initial state
An initial state can be defined. More precisely, we define constraints that must be satisfied on the initial state. An initial constraint definition block is preceded by the keyword *init* and has a name. It is possible to define different ones, in which case the initial state is constrained by the conjunction of the formulas.

```
init first_init_constraint {
    b and i = 0
}
init second_init_constraint {
    x > 0 or y > 0
}
```

# Invariants
In the same way as we specified the initial constraints, we can specify the invariants. Invariants are the constraints that must be satisfied at each state of the transition system's evolution. Invariant blocks are preceded by the keyword *inv*.

```
inv one_invariant {
    i >= 0
}
inv other_invariant {
    x > 0 implies b
}
```

# Transitions and Triggers

The transition system is defined by transitions and triggers. The system can be composed of an arbitrary number of transitions and triggers (possibly zero).

## Transitions
Transitions are defined as a constraint between the current state and the next state. The next state of a variable is denoted *'*. Transition blocks are preceded by the keyword *trans* and given a name. The transition system is constrained by the non-exclusive disjunction of transitions. This means that several transitions can be executed at the same time (constraints permitting).

```
trans first_transition {
    (i < 10) and (i' = i+ 1)
}
trans second_transition {
    i >= 10 and i' = 0
}
```

## Trigger
Triggers are defined as a constraint between the current state and the next state, in the same way as transitions. Trigger blocks are preceded by the keyword *trig* and have a name. The transition system is constrained by the conjunction of triggers. In a way, they are transition invariants. We'll look at their use in conjunction with transitions in section [advanced](advanced.md).

# LTL formula
The LTL formula to be satisfied on the transition system is defined after the *prop* keyword. The usual LTL operators are present (C, F, G, U, R). In addition, any expression on the current state is allowed.

```
prop = G F (i = 0)
```

# Search
Once the transition system and formula have been defined, we can specify the search. For each search, we specify the LTL semantics, the termination and the search type.
There are two types of search *solve* and *optimize*. It is also possible to limit the size of the transition function.

It is important to specify that the solver searches for a trace that satisfies the constraints and does not search for a proof. In order to make a proof, the negation of the formula must be analyzed.

### LTL semantics
There are 4 semantics:
- *truncated* considers that each trace can stop at any transition.
- infinite* corresponds to classic infinite traces.
- finite* gives *truncated* semantics, but only on maximal traces. In other words, the trace must be finite (i.e. have no possible next transition).
- complete* corresponds to the proof of termination of the analysis.

Semantics can be combined with the *+* operator.

```
search infinite + complete solve
```

# Example

The complete example can be found [here](../files/docs/principle.tat).