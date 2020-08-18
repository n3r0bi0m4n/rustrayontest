mod parser;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Parser {
    runtime: Arc<ThreadPool>,
    queue: Arc<RwLock<VecDeque<String>>>,
    running: Arc<AtomicBool>,
}

impl Parser {
    pub fn new() -> Self {
        let rt: ThreadPool = ThreadPoolBuilder::new()
            .thread_name(|idx| format!("parser-thread-{}", idx))
            .num_threads(8)
            .build()
            .unwrap();

        Parser {
            runtime: Arc::new(rt),
            queue: Arc::new(RwLock::new(VecDeque::with_capacity(8))),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) {
        let running = self.running.clone();
        let queue = self.queue.clone();
        let runtime = self.runtime.clone();
        running.store(true, Ordering::SeqCst);
        println!("spawn thread");
        thread::spawn(move || {
            println!("Loop thread: {:?}", thread::current());

            loop {
                if running.load(Ordering::SeqCst) == false {
                    println!("got stop");
                    break;
                }

                let r_queue = queue.read().unwrap();
                // println!("empty? {}", r_queue.is_empty());

                if !r_queue.is_empty() {
                    println!("not empty");
                    let len = r_queue.len();
                    drop(r_queue);
                    let mut w_queue = queue.write().unwrap();
                    let to_parse = w_queue.pop_front().unwrap();
                    drop(w_queue);
                    println!("readed: {}", &to_parse);

                    runtime.install(|| {
                        println!(
                            "{} passed to queue. Queue len: {} [Thread: {:?}]",
                            to_parse,
                            len,
                            if let Some(name) = thread::current().name() {
                                name
                            } else {
                                "No name"
                            }
                        );
                    });
                }
            }
        });
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
