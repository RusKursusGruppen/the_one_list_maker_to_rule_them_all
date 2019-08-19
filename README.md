# TheOneListMakerToRuleThemAll... in Rust

## Requirements
- Rust
- A good large LaTeX install

## How to build
Run `cargo build --release` in this directory

## Examples
### Beer list
`the_one_list_maker_to_rule_them_all -b < data.csv`

`cat data.csv | the_one_list_maker_to_rule_them_all --beer`

data.csv
```
Bob
Alice
Ekans
Eve
```
### Gigs
Generate default list:
`the_one_list_maker_to_rule_them_all_rust -g`

Generate custom list:
`the_one_list_maker_to_rule_them_all_rust -g < data.csv`

data.csv
```
Anti-kaos, false, true, true
Morgenmad, false, true, true
Frokost, false, true, true
Oprydning (Morgenmad), false, true, true
Oprydning (Frokost), false, true, true
Aftensmad, true, true, true
Oprydning (Aftensmad), true, true, true
```

