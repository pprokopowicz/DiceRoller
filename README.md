# DiceRoller

Simple Rust cli used to roll dice.

Crates used:
- [clap](https://github.com/clap-rs/clap)
- [cli-table](https://github.com/devashishdxt/cli-table)
- [rand](https://github.com/rust-random/rand)

## Usage
To roll once 20-sided dice:

```console
foo@bar:~$ roller d20
```
or
```console
foo@bar:~$ roller 1d20
```
---
To roll 4 times a 20-sided dice:
```console
foo@bar:~$ roller 4d20
```
---
To roll multiple dice at once separate them with a whitespace:
```console
foo@bar:~$ roller d4 1d6 5d20
```
---

## Output
Output is printed in a table with all rolls and sum:
```console
foo@bar:~$ roller d4 1d6 5d20 2d16

+------+-----------------+-----+
| Dice |     Throws      | Sum |
+------+-----------------+-----+
|  d4  |        2        |  2  |
+------+-----------------+-----+
|  d6  |        5        |  5  |
+------+-----------------+-----+
| d20  | 3, 4, 17, 13, 7 | 44  |
+------+-----------------+-----+
| d16  |      2, 1       |  3  |
+------+-----------------+-----+
```
