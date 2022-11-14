use channel_drain::drain;

#[test]
fn somke() {
    let foo = 42;
    let bar = 11;

    drain!(
        foo(bar) => { println!("Hi!") },
        bar(baz) => println!("ho")
    );
}
