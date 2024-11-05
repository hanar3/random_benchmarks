use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;

const BUFFERS: usize = 1024 * 10;
#[derive(Debug, Copy, Clone)]
pub struct Buffer {
    pub data: Option<&'static str>,
}

pub struct Pool {
    pub buffers: [Buffer; BUFFERS],
    pub windex: i32,
    pub rindex: i32,
}

impl Pool {
    pub fn new() -> Self {
        Pool {
            windex: 0,
            rindex: 0,
            buffers: [Buffer { data: None }; BUFFERS],
        }
    }
    pub fn write(&mut self, data: &'static str) -> bool {
        let next_windex = (self.windex + 1) % BUFFERS as i32;
        if next_windex == self.rindex {
            // println!("Failed to log, log buffer is full");
            return false;
        }

        self.buffers[self.windex as usize].data = Some(data);
        // println!("wrote {}, windex: {}", data, self.windex,);
        self.windex = next_windex;
        return true;
    }
    pub fn read(&mut self) -> Option<&'static str> {
        if self.rindex != self.windex {
            let data = self.buffers[self.rindex as usize].data;
            let next_rindex = (self.rindex + 1) % BUFFERS as i32;
            self.rindex = next_rindex;
            // println!("read {:?} ridx: {}", data, self.rindex);
            return data;
        } else {
            None
        }
    }
}

pub fn run_bench() {
    let p = Arc::new(Mutex::new(Pool::new()));

    {
        let p_clone = Arc::clone(&p);
        let p_clone2 = Arc::clone(&p);
        let video_thread = std::thread::spawn(move || {
            let mut index = 0;
            println!("Spawned video thread");
            let mut now = SystemTime::now();

            loop {
                let mut lock = p_clone.lock().expect("Unable to acquire lock");
                let write_result = lock.write("Hello");

                if write_result {
                    index += 1;
                }

                // perf
                if now.elapsed().unwrap() > std::time::Duration::from_secs(1) {
                    println!("Writes: {}/s", index);
                    now = SystemTime::now();
                    index = 0;
                }
            }
        });

        let process_logs_thread = std::thread::spawn(move || {
            println!("Spawned process logs thread");
            let mut now = SystemTime::now();
            let mut index = 0;
            loop {
                let mut lock = p_clone2.lock().expect("Unable to acquire lock");
                if let Some(val) = lock.read() {
                    index += 1;
                } else {
                    // Do nothing..?
                }

                if now.elapsed().unwrap() > std::time::Duration::from_secs(1) {
                    println!("Reads: {}/s", index);
                    now = SystemTime::now();
                    index = 0;
                }
            }
        });

        let _ = video_thread.join();
        let _ = process_logs_thread.join();
    }
}
