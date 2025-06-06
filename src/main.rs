
// If an event a occurs before another event b, then a should happen at an earlier time than b.
// We state this condition more formally as follows. 

// Clock Condition.
// For any events a, b: if a---> b then C(a) < C(b).
// The Clock condition is satisfied when the following conditions hold.

// C1: 
// If a and b are events in process Pi, and a comes before b, then Ci(a) < Ci(b).

// C2: 
// If a is the sending of a Message by process Pi and b is the receipt of that Message by process Pj, then Ci(a) < Cj(b).

// Implementation rules.
// To guarantee that the system of clocks satisfies the Clock Condition, we will insure that it satisfies conditions C 1 and C2.

// Condition C1 is simple; the processes need only obey the following implementation rule:

// IR1: 
// Each process Pi increments Ci between any two successive events.

// IR2: 
// (a) If event a is the sending of a Message m by process Pi, then the Message m contains a timestamp Tm= Ci(a).
// (b) Upon receiving a Message m, process Pj sets Cj greater than or equal to its present value and greater than Tm.

use std::cmp::max;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Debug)]
struct LogicalClock {
    count: u32
}

impl LogicalClock {
    fn new() -> LogicalClock{
        LogicalClock { count: 0 }
    }

    fn tick(&mut self) -> u32 {
        // let res = self.count;
        self.count += 1;
        self.count
    }
}
#[derive(Debug)]
struct Message {
    tm:u32
}

fn main() {
    let clock_a = Arc::new(Mutex::new(LogicalClock::new()));
    let clock_b = Arc::new(Mutex::new(LogicalClock::new()));
    let (tx_a, rx_a) = mpsc::sync_channel::<Message>(0);
    let (tx_b, rx_b) = mpsc::sync_channel::<Message>(0);

    // Set up process 1
    let clock_a_clone = Arc::clone(&clock_a);
    let tx_b_clone = tx_b.clone();
    let proc_a = thread::spawn(move || {
        //let tx_b_clone = tx_b.clone();
        let mut clock_lock = clock_a_clone.lock().unwrap();

        for i in 0..=20 {
            let mut rng = rand::rng();
            let random_number: u32 = rng.random_range(0..=100); // Generate a u32 between 0 and 100 (inclusive)
            if let Ok(m) = rx_a.try_recv() {
                clock_lock.count = max(clock_lock.count, m.tm)+1;
                // let _ = clock_lock.tick();
                println!("a-recv-{:?}", clock_lock.count);
                println!("  |");
            } else if random_number%2 == 0 {
                let reading = clock_lock.tick();
                let m = Message {tm: reading};
                tx_b_clone.send(m).unwrap();
                //tx_b.send(m).unwrap();
                println!("  |");
                println!("a-send-{:?}", reading);
                println!("  |");
            }else {
                println!("  |");
            }
        }
        thread::sleep(Duration::from_millis(500));
    });

    // Set up process 2
    let clock_b_clone = Arc::clone(&clock_b);
    let tx_a_clone = tx_a.clone();
    let proc_b = thread::spawn(move || {
        // let tx_a_clone = tx_a.clone();
        let mut clock_lock = clock_b_clone.lock().unwrap();

        for i in 0..=20 {
            let mut rng = rand::rng();
            let random_number: u32 = rng.random_range(0..=100); // Generate a u32 between 0 and 100 (inclusive)
            if let Ok(m) = rx_b.try_recv() {
                clock_lock.count = max(clock_lock.count, m.tm)+1;

                // let _ = clock_lock.tick();
                println!("          b-recv-{:?}", clock_lock.count);
                println!("              |");
            } else if random_number%3 == 0 {
                let reading = clock_lock.tick();
                let m = Message {tm: reading};
                tx_a_clone.send(m).unwrap();
                // tx_a.send(m).unwrap();
                println!("              |");
                println!("          b-send-{:?}", reading);
                println!("              |");
            }else {
                println!("              |");
            }
        }
        thread::sleep(Duration::from_millis(500));
    });

    thread::sleep(Duration::from_millis(500));
    // drop the senders or else the thread handlers won't return
    drop(tx_a);
    drop(tx_b);

    proc_a.join().unwrap();
    proc_b.join().unwrap();

    let final_a = clock_a.lock().unwrap().count;
    let final_b = clock_b.lock().unwrap().count;
    println!("Final clocks: Ca={}, Cb={}", final_a, final_b);
}

