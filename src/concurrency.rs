
macro_rules! concurrency {
    ($start: expr, $end: expr, |$x: ident| -> $dd: block) => {
        let handlers: Vec<_> = ($start..$end).map(|$x| {
            thread::spawn(move || {
                $dd;
            })
        }).collect();

        for h in handlers {
            h.join().unwrap();
        }

    }
}

#[cfg(test)]
mod tests {
    use vectors;

    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread() {
        let handlers: Vec<_> = (0..10).map(|i| {
            thread::spawn(move || {
                print!("{}, ", i);
            })
        }).collect();
        vectors::print_vec(&(0..10).collect::<Vec<i32>>());
        //
        for h in handlers {
            h.join().unwrap();
        }

    }

    #[test]
    fn test_sleep() {
        concurrency!(0, 10, |x| -> {
            thread::sleep(Duration::from_millis(50));
            print!("{}, ", x);
        });
    }

    #[test]
    fn test_arc() {
        use std::sync::{Arc, Mutex};

        let data = (0..10).collect::<Vec<_>>();
        //
        // shared memory
        let data = Arc::new(Mutex::new(data));

        let handlers: Vec<_> = (0..10).map(|i| {
            let data = data.clone();
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data[i] += i * i;
            })
        }).collect();
        //vectors::print_vec(&handles);
        //
        for h in handlers {
            h.join().unwrap();
        }

        // what the huck?
        let data = data.lock().unwrap();
        for (i,v) in data.iter().enumerate() {
            // when use * when use &?
            assert_eq!(i * i + i, *v);
        }
    }

    #[test]
    fn test_channel() {
        use std::sync::{Arc, Mutex};
        use std::sync::mpsc;

        let data = Arc::new(Mutex::new(0));

        let (tx, rx) = mpsc::channel();

        for _ in 0..10 {
            let (data, tx) = (data.clone(), tx.clone());

            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                *data += 1;

                tx.send(()).unwrap();
            });
        }

        for _ in 0..10 {
            assert_eq!(rx.recv().unwrap(), ());
        }

        // * means de reference, fock!
        assert_eq!(*data.lock().unwrap(), 10);
    }

    #[test]
    fn test_thread_panic() {
        use std::thread;

        let handle = thread::spawn(move || {
            panic!("ooooooooooops!");
        });

        let result = handle.join();

        assert!(result.is_err());
    }
}
