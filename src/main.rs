use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::{Duration},
};

use rand::Rng;
use tokio::runtime::Builder;

#[derive(Debug, Clone)]
enum Action { Add, Mul }

#[tokio::main]
 async fn main() {

    // 1. Maksymalna liczba wątków wynosi 4
    // 2. Przydział puli wątków w Tokio jest losowy
    // 3. Pętla oczekuje na zakończenie głównej procedury
    // 4. Zbyt szybkie spawnowanie wątków gubi je

    let threads = 4;
    let iter  = Arc::new(tokio::sync::Mutex::new(0));
    let end = Arc::new(tokio::sync::Mutex::new(128));
    let inc = tokio::sync::Mutex::new(1);
    let action = tokio::sync::Mutex::new(Action::Add);

    //let runtime = Builder::new_multi_thread()
    //    .worker_threads(threads)
    //    .thread_name("my-custom-name")
    //    .thread_stack_size(3 * 1024 * 1024)
    //    .build()
    //    .unwrap();

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
                    println!("[Thread {}] Hazard access variable", i);
                }

                let mut rng = rand::thread_rng();
                let value = rng.gen_range(100, 2000);
                thread::sleep(Duration::from_millis(value));

            }

            println!("[Thread {}] Ended task", i);

        });

        thread::sleep(Duration::from_millis(500));

    }
}
