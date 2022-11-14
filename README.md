# Channel-drain

Drains a (crossbeam) channel.

[Hoare Was Right](https://www.cs.cmu.edu/~crary/819-f09/Hoare78.pdf) - we should
prefer channels over shared memory for communicating between threads.
This is trivial for a thread that consumes a single channel in Rust:

```rust
fn my_thread(rx: Receiver<i32>) {
    while let Ok(num) = rx.recv() {
        // Cool stuff
    }
}
```

Awesome! With almost no code, it

- Handles synchronization for you, doing work when inputs are ready and sleeping
  when they're not.

- Automatically closes itself when no inputs remain.
  (We can construct entire DAGs of these, close the original producer,
  and watch all consumers gracefully shut themselves down.)

But in more complicated systems, we often want to receive from _multiple_ channels.
Crossbeam (and some other channel libraries) provide a nice
[`Select`](https://docs.rs/crossbeam-channel/latest/crossbeam_channel/struct.Select.html)
mechanism for this, but you need to write a decent bit of boilerplate to
remove a channel from the set you're polling when it closes.

No more:
```rust
fn smoke_test() {
    let (tx1, rx1) = bounded(10);
    let (tx2, rx2) = bounded(10);

    tx1.send("For a successful technology").unwrap();
    tx1.send("reality must take precedence over public relations").unwrap();
    tx1.send("for nature cannot be fooled").unwrap();
    tx2.send(42).unwrap();
    tx2.send(22).unwrap();
    tx2.send(99).unwrap();

    drop(tx1);
    drop(tx2);

    drain!{
        rx1(bar) => { println!("Feynman Says: \"{}\"", bar) },
        rx2(baz) => println!("Some num: {}", baz)
    };
}
```
gives us:
```
Some num: 42
Some num: 22
Feynman Says: "For a successful technology"
Some num: 99
Feynman Says: "reality must take precedence over public relations"
Feynman Says: "for nature cannot be fooled"
```
(Crossbeam randomly selects a ready channel to promote fairness.)
