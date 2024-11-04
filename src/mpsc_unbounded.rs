use std::sync::mpsc::channel;
use std::time::SystemTime;
pub fn run_bench() {
    let (tx, rx) = channel::<&'static str>();

    let write_thread = std::thread::spawn(move || {
        let mut index = 0;
        let mut now = SystemTime::now();

        loop {
            match tx.send("Hello") {
                Ok(_) => {
                    index += 1;
                }
                Err(..) => {}
            }
            // perf
            if now.elapsed().unwrap() > std::time::Duration::from_secs(1) {
                println!("Writes: {}/s", index);
                now = SystemTime::now();
                index = 0;
            }
        }
    });
    let read_thread = std::thread::spawn(move || {
        let mut index = 0;
        let mut now = SystemTime::now();
        loop {
            match rx.try_recv() {
                Ok(..) => {
                    index += 1;
                }
                Err(_) => {}
            }
            if now.elapsed().unwrap() > std::time::Duration::from_secs(1) {
                println!("Reads: {}/s", index);
                now = SystemTime::now();
                index = 0;
            }
        }
    });

    let _ = write_thread.join();
    let _ = read_thread.join();
}
