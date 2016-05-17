#[macro_use]
extern crate chan;

use std::thread;
use std::time::Duration;

fn main() {
    let tick = chan::tick_ms(100);
    let boom = chan::after_ms(500);
    let (s1, r1) = chan::sync::<i32>(2);
    loop {
        chan_select! {
            default => {
                println!("   .");
                thread::sleep(Duration::from_millis(50));
            },
            tick.recv() => println!("tick."),
            boom.recv() => { println!("BOOM!"); return; },
            s1.send(3) => { println!("sent on channel") },
            r1.recv() -> v => {
                println!("recv'd on channel ({:?})", v);
                thread::sleep(Duration::from_millis(70));
            },
        };
    }
}
