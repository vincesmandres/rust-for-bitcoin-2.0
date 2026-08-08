# Rust for Bitcoin 2.0 — Week 2

Build a simplified Bitcoin transaction model while practising structs, enums,
traits, ownership, borrowing, collections, and `Result`-based error handling.

The crate is intentionally incomplete. Search for `TODO` and implement each part;
do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Parts 3–5 in `transaction.rs` and `error.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits and borrowing functions in Parts 6–7.
5. Build the payment example in `main.rs`.
6. Complete UTXO selection and its tests.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in the
submission.

## Written answers
### Ownership compiler error

During Part 7, I intentionally tried to use an input after moving it into the transaction:

```text
error[E0382]: borrow of moved value: `input_1`
  --> src\main.rs:30:16
   |
11 |     let input_1 = InputKind::Regular {
   |         ------- move occurs because `input_1` has type `InputKind`, which does not implement the `Copy` trait
...
29 |     transaction.add_input(input_1);
   |                           ------- value moved here
30 |     println!("{input_1}");
   |                ^^^^^^^ value borrowed here after move
```

This happened because `add_input()` takes an InputKind by value. When I passed `input_1` to the method, ownership of that value moved into the transaction's inputs vector. Since `InputKind` does not implement Copy, the original variable could no longer be used after the move. This is Rust preventing the same owned value from being used from two places at the same time.



## Written answers
Answer in your own words. Add the ownership compiler error from Part 7 as a fenced
text block, then explain what caused it.

1. What is a Bitcoin transaction input?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

2. What is a Bitcoin transaction output?
#### Answer:
An output defines where bitcoin is sent and how much is assigned to that destination. An unspent output can later be used as an input in another transaction.

3. What is a UTXO?
#### Answer:
A ```UTXO``` is an **Unspent Transaction Output**. It is bitcoin from a previous output that has not yet been spent and is available for a new transaction.

4. What does an outpoint identify?

#### Answer: 
An outpoint identifies one specific output from a previous transaction using:

- the transaction ID (`txid`)
- the output index (`vout`)

5. How is a transaction fee calculated?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

6. Why use integers rather than floating-point numbers for bitcoin amounts?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

7. Why does `total_input_value()` borrow `self`?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

8. Why does `add_input()` take `&mut self`?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

9. What happens when an input is moved into a transaction?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

10. Why is `Result` preferable to `panic!` for validation failures?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

11. How do enums help model regular and coinbase inputs?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.

12. How does the `BitcoinValue` trait reduce duplication?
#### Answer:
An input represents bitcoin being spent. For a regular transaction, it references an output from a previous transaction.


## Design notes

Describe any choices you made, including your UTXO-selection trade-offs and (if
attempted) the optional transaction-state extension.

## Example output

Paste the output of `cargo run` here once Part 8 is complete.
