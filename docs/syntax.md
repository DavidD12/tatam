# Varibles and Constants Types

Variable and constant types can be Boolean, enumerated, integer, interval or even real.

```bnf
enum E = {A, B, C}
interval I = 10..1000

var b: Bool
var e: E
var i: Int
var j: I

init my_init {
    b = true and
    e = B and
    i = j
}
```

# Multiple files

```bnf
include "file.tat"
```

# Expression language

The priority of expressions follows the classic rules of literature.

## Boolean operation

```bnf
expr := not expr | expr and expr | expr or expr | expr implies expr | (expr)
```

## Arithmetic operation

```bnf
expr := - expr | expr * expr | expr + expr | expr - expr | expr min expr | expr max expr | (expr)
```

## Comparison

```bnf
expr := expr = expr | expr != expr | expr > expr | expr >= expr | expr < expr | expr <= expr 
```

## If Then Else

```bnf
expr := if expr then expr (elif expr thend expr)* else expr end
```

# "Scope"

The scope expression is a writing shortcut used when specifying the transition function. This expression allows you to specify only those variables that change. Variables not present remain unchanged after the transition.

```bnf
expr := | ident (, ident)* | (expr)
```

In the following example, the value of y remains unchanged by the transition:
```
var x, y: Int

trans my_transition {
    |x|(x' = x + 1)
}
```

This is equivalent to the following definition: 
```
var x, y: Int

trans my_transition {
    (x' = x + 1) and (y' = y)
}
```


# LTL Formulas

# Search

You can specify the number of transitions in the search using an interval.

```bnf
search [integer..integer?]
```

```
search[10..20] truncated solve
```

## Optimize

You can specify a criterion to be optimized. This criterion can relate to one of the states of the transition system, starting from a reference. For purely technical reasons of SMT resolution, a bound must be added.

```bnf
minimize expr until expr
```
```bnf
state := first [(+|-) integer] | current [(+|-) integer] | last [(+|-) integer]
```

```
search truncated minimize (time at last) until 0
```