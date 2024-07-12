# Transition And Theory Analysis Machine

## Install

- Install [Rust](https://www.rust-lang.org)
- Install [z3](https://github.com/Z3Prover/z3)

```shell
sudo apt install z3
```

- Install Tatam
```shell
cargo install tatam
```

### Install Vscode extension [tatam-lang](https://github.com/DavidD12/tatam-lang) (Optional)

```shell
cd ~/.vscode/extensions
git clone https://github.com/DavidD12/tatam-lang.git
```

## Example

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

## Execute

```shell
tatam -f file.tat
```


## Documentation

Some documentation can be found [here](docs/documentation.md)
