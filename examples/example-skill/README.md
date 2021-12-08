# Simple mycroft skill example in Rust

# Usage

This assumes you have rust setup on your system.

To build the skill cd to this folder and run

```
cargo build
```

After building you can simply run the skill by

```
./target/debug/example-skill
```

Note that Mycroft needs to be started before the skill is started.

# Invoking the skill

The skill registers two intents

"Hello Rust" and "Good bye Rust" and will give an appropriate response to these utterances.
