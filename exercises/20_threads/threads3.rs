use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    let q1 = Arc::clone(&q);
    let tx1 = tx.clone();
    thread::spawn(move || {
        let lq1 = q1.lock().unwrap();
        for val in &lq1.first_half {
            println!("Sending {val:?}");
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let q2 = Arc::clone(&q);
    let tx2 = tx.clone();
    thread::spawn(move || {
        let lq2 = q2.lock().unwrap();
        for val in &lq2.second_half {
            println!("Sending {val:?}");
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(Arc::new(Mutex::new(queue)), tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
