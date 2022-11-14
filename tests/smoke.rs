use channel_drain::drain;

use crossbeam::channel::bounded;

#[test]
#[rustfmt::skip]
fn smoke() {
    let (tx1, rx1) = bounded(10);
    let (tx2, rx2) = bounded(10);

    tx1.send("For a successful technology").unwrap();
    tx1.send("reality must take precedence over public relations") .unwrap();
    tx1.send("for nature cannot be fooled").unwrap();
    tx2.send(42).unwrap();
    tx2.send(22).unwrap();
    tx2.send(99).unwrap();

    drop(tx1);
    drop(tx2);

    drain! {
        rx1(bar) => { println!("Feynman Says: \"{}\"", bar) },
        rx2(baz) => println!("Some num: {}", baz),
    };
}
