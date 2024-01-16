Transition And Theory Analysis Machine

# Install

1. Install Rust: [Rust](https://www.rust-lang.org)
2. Install [z3](https://github.com/Z3Prover/z3)

## Ubuntu

```console
xxx@XXX:~$ sudo apt install z3
```

3. Install tatam:

```console
xxx@XXX:~$ cargo install tatam
```

# Example

```
cst x: Int
var y: Int

init inits {
    y = 0
}

inv invariants {
    x > y
}

trans tr_inc {
    y < 10 and y' = y + 1
}

trans tr_loop {
    y >= 10 and y' = 0
}

prop = G(F (y = 1))

search infinite solve

```