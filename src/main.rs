extern crate parking_lot;
extern crate time;

const LOOPS: i64 = 500_000_000;


fn parkinglot() {
    let lock = parking_lot::RwLock::new(0u64);
    let before = time::precise_time_ns();

    for _ in 0..LOOPS {
        let mut w = lock.write();
        *w += 1;
    }

    let after = time::precise_time_ns();

    println!("parkinglot: completed in {}ns", after - before)
}

fn stdlib() {
    let lock = std::sync::RwLock::new(0u64);
    let before = time::precise_time_ns();

    for _ in 0..LOOPS {
        let mut w = lock.write().ok().unwrap();
        *w += 1;
    }

    let after = time::precise_time_ns();

    println!("stdlib: completed in {}ns", after - before)
}

fn spinlock() {
    let before = time::precise_time_ns();

    let mut w: u64 = 0;

    for _ in 0..LOOPS {
        w += 1;
    }

    let after = time::precise_time_ns();

    println!("spinlock: completed in {}ns", after - before)
}

fn main() {
    parkinglot();
    stdlib();
    spinlock();
}
