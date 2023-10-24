#![allow(unused)]

#[cfg(test)]
mod tests {
    use std::{panic, thread};
    use std::any::Any;
    use std::sync::{Arc, Condvar, Mutex};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn scope() {
        let scope_return = thread::scope(|scope| {
            let handle = scope.spawn(|| {
                sleep(Duration::from_millis(100));
            });

            // Can join handle within scope or let it auto-join when dropped out
            assert!(!handle.is_finished());
            "Scope Return"
        });

        // Threads joined and handles dropped
        assert_eq!(scope_return, "Scope Return");
    }

    #[test]
    fn mutate_slices() {
        const N: usize = 4096;
        const T: usize = 4;

        let mut buf = [0_u8; N];
        let (bufs, []) = buf.as_chunks_mut::<{ N / T }>() else {
            panic!("bufs not evenly divisible N={N}, T={T}");
        };

        thread::scope(|scope| {
            for b in bufs {
                scope.spawn(|| b.fill(1));
            }
        });

        assert_eq!(buf, [1; N]);
    }

    #[test]
    fn thread_name() {
        let builder = thread::Builder::new().name("Test thread".to_string());
        thread::scope(|scope| {
            let spawned = builder.spawn_scoped(scope, || {
                // me binding required for lifetime, or alternatively clone the name
                let me = thread::current();
                assert_eq!(me.name(), Some("Test thread"));
            });

            assert!(spawned.is_ok());
        });
    }

    fn error_message<T>(r: Result<T, Box<dyn Any + Send + 'static>>) -> Result<T, &str> {
        r.map_err(|box_any|
            box_any.downcast()
                .map(|e| *e)
                .unwrap_or("(No error message)")
        )
    }

    #[test]
    fn thread_panic() {
        let flag = AtomicBool::new(false);

        let panicked = panic::catch_unwind(|| {
            thread::scope(|scope| {
                scope.spawn(|| {
                    sleep(Duration::from_millis(250));
                    flag.store(true, Ordering::Release);
                });

                scope.spawn(|| {
                    panic!("Oh no")
                });
            });
        });

        // All threads run to completion: flag is set after other thread panics
        assert!(flag.into_inner());

        // Generic error message returned by thread::scope
        assert_eq!(error_message(panicked), Err("a scoped thread panicked"));
    }

    #[test]
    fn thread_join_panic() {
        thread::scope(|scope| {
            let handle = scope.spawn(|| panic!("Oh no"));
            // Join inside the scope for access to each thread's (panic) result
            assert_eq!(error_message(handle.join()), Err("Oh no"));
        });
    }

    #[test]
    fn mutex() {
        const N: usize = 4;

        // Don't need an Arc here since &Mutex is Send and scope gives us control over lifetimes
        let vec = Mutex::new(Vec::with_capacity(N));
        thread::scope(|scope| {
            // Use mutex by ref, only i is moved
            let vec = &vec;
            for i in 0..N {
                scope.spawn(move || {
                    let mut vec_guard = vec.lock().expect("Mutex shouldn't be poisoned");
                    vec_guard.push(i);
                });
            }
        });

        let v = vec.into_inner().expect("Mutex shouldn't be poisoned");
        assert_eq!(v.len(), N);
    }

    #[test]
    fn condvar() {
        let lock_cvar1 = Arc::new((Mutex::new(false), Condvar::new()));
        let lock_cvar2 = Arc::clone(&lock_cvar1);

        let handle = thread::spawn(move || {
            let (lock, cvar) = &*lock_cvar2;
            let mut started_g = lock.lock().unwrap();
            *started_g = true;
            cvar.notify_one();
        });

        let (lock, cvar) = &*lock_cvar1;
        let started_g = cvar.wait_while(lock.lock().unwrap(),
            |&mut started| !started
        ).unwrap();
        // Equivalently:
        // let mut started_g = lock.lock().unwrap();
        // while !*started_g {
        //     started_g = cvar.wait(started_g).unwrap();
        // }

        assert!(*started_g);
        handle.join().unwrap();
    }

    #[test]
    fn condvar_timeout() {
        let lock_cvar1 = Arc::new((Mutex::new(false), Condvar::new()));
        let lock_cvar2 = Arc::clone(&lock_cvar1);

        let handle = thread::spawn(move || {
            let (lock, cvar) = &*lock_cvar2;
            // Note because there is no std lock timeout (see below), the test will fail if we
            // sleep while holding the lock: there will be no condvar timeout, only lock contention
            sleep(Duration::from_millis(100));
            let mut started_g = lock.lock().unwrap();
            *started_g = true;
            cvar.notify_one();
        });

        let (lock, cvar) = &*lock_cvar1;
        // Note: there is no timeout to get this lock, so perhaps not a very useful construct
        let started_g = lock.lock().unwrap();
        // Zero is treated normally, not as forever 😊
        let timeout = Duration::from_millis(0);
        let (started_g, result) = cvar.wait_timeout_while(started_g, timeout,
            |&mut started| !started
        ).unwrap();

        // Lock is reacquired even after timeout
        assert!(!*started_g);
        drop(started_g);

        assert!(result.timed_out());
    }

    #[test]
    fn available_parallelism() {
        dbg!(thread::available_parallelism().unwrap().get());
    }
}
