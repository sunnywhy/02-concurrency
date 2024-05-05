use anyhow::Result;
use rand::random;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = producer(i, tx);
        });
    }
    drop(tx); // drop tx to close rx since all producers used the clone of tx

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Consumer: {:?}", msg);
        }
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))?;
    println!("Secret: {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit
        if random::<u8>() % 10 == 0 {
            println!("Producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
