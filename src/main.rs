use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::{Duration},
};

use rand::Rng;

#[derive(Debug, Clone)]
enum Action { Add, Mul }

#[tokio::main]
 async fn main() {

    let threads = 4;
    let iter  = Arc::new(tokio::sync::Mutex::new(0));
    let end = Arc::new(tokio::sync::Mutex::new(32));
    let inc = tokio::sync::Mutex::new(1);
    let action = tokio::sync::Mutex::new(Action::Add);

    for i in 0..threads {

        let iter_clone = Arc::clone(&iter);
        let end_clone = Arc::clone(&end);

        tokio::spawn(async move {

            let local_iter_clone = &iter_clone;
            let local_end_clone = &end_clone;

            println!("[Thread {}] Creating thread", i);

            loop {

                let lock_iter = local_iter_clone.try_lock();
                match lock_iter {
                    Ok(v) => {
                        let lock_end = local_end_clone.try_lock();
                        match lock_end {
                            Ok(w) => {
                                if *v >= *w {
                                    break;
                                }
                            },
                            _ => { }
                        }
                    },
                    _ => { }
                };

                let lock = local_iter_clone.try_lock();
                if let Ok(mut v) = lock {
                    *v += 1;
                    println!("[Thread {}] Accessed variable ({})", i, v);
                } else {
                    println!("[Thread {}] Hazard acces variable", i);
                }

                let mut rng = rand::thread_rng();
                let value = rng.gen_range(100, 5000);
                thread::sleep(Duration::from_millis(value));

            }

            println!("[Thread {}] Ended task", i);

        });

    }
}