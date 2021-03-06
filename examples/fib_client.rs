extern crate wire;

use wire::SizeLimit;

fn main() {
    // Only allow incomming messages of at max 16 bytes, and verify that all of
    // our outgoing messages aren't over 8 bytes.
    let (read_limit, write_limit) = (SizeLimit::Bounded(16),
                                     SizeLimit::Bounded(8));

    // Connect to our running fib-server.
    // incoming: (u64, u64)
    // outgoing: u64
    let (i, mut o) = wire::connect_tcp(("localhost", 8080), read_limit, write_limit).unwrap();

    // Send all the numbers from 0 to 10.
    for x in 0u64 .. 10u64 {
        o.send(&x).ok();
    }

    // Close our outgoing pipe. This is necessary because otherwise,
    // the server will keep waiting for the client to send it data and
    // we will deadlock.
    o.close();

    // Print everything that we get back.
    for a in i.into_blocking_iter() {
        let (x, fx): (u64, u64) = a;
        println!("{} -> {}", x, fx);
    }
}
