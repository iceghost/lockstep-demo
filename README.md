# Lockstep protocol

This is a simple [lockstep protocol][lockstep wiki] demo, using the rock-paper-scissor game as an example.

Rust nightly is used to try [`-Z sparse-registry`][sparse registry docs] speed. Run:

```bash
$ cargo run
```

Sample output:

```log
2023-01-13T09:25:29.625333Z  INFO commit{self=Alice hand=Rock}: lock_step_demo: done
2023-01-13T09:25:29.625333Z  INFO commit{self=Trudy hand=Scissor}: lock_step_demo: done
2023-01-13T09:25:29.625478Z  INFO send_commit{self=Alice}: lock_step_demo: done
2023-01-13T09:25:29.625492Z  INFO send_commit{self=Trudy}: lock_step_demo: done
2023-01-13T09:25:29.625591Z  INFO wait_commit{self=Alice}: lock_step_demo: got commit
2023-01-13T09:25:29.625595Z  INFO wait_commit{self=Trudy}: lock_step_demo: got commit
2023-01-13T09:25:29.625703Z  INFO throw_hand{self=Alice hand=Rock}: lock_step_demo: done
2023-01-13T09:25:29.625716Z  INFO wait_hand{self=Trudy}: lock_step_demo: got rock
2023-01-13T09:25:29.625864Z  WARN change_hand{self=Trudy}: lock_step_demo: change hand to paper
2023-01-13T09:25:29.625968Z  INFO throw_hand{self=Trudy hand=Paper}: lock_step_demo: done
2023-01-13T09:25:29.626033Z  INFO wait_hand{self=Alice}: lock_step_demo: got paper
2023-01-13T09:25:29.626177Z ERROR lock_step_demo: cheater detected!
```

Try changing the player in [main.rs](./src/main.rs) to see the protocol in effect.

[lockstep wiki]: https://en.wikipedia.org/wiki/Lockstep_protocol
[sparse registry docs]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#sparse-registry