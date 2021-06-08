#[cfg(test)]
mod tests {

    #[test]
    fn test_threadpool() {
        use threadpool::ThreadPool;

        let n_workers = 4;
        let n_jobs = 8;
        let pool = ThreadPool::new(n_workers);
        let (tx, rx) = std::sync::mpsc::channel();
        for _ in 0..n_jobs {
            let tx = tx.clone();
            pool.execute(move || {
                tx.send(1)
                    .expect("channel will be there waiting for the pool");
            });
        }
        assert_eq!(8, (rx.iter().take(n_jobs).fold(0, |a, b| a + b)));
    }

    #[test]
    fn test_threadpool_barrier() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::{Arc, Barrier};
        use threadpool::ThreadPool;

        // create at least as many workers as jobs or you will deadlock yourself
        let n_workers = 42;
        let n_jobs = 23;
        let pool = ThreadPool::new(n_workers);
        let an_atomic = Arc::new(AtomicUsize::new(0));

        assert!(n_jobs <= n_workers, "too many jobs, will deadlock");

        // create a barrier that waits for all jobs plus the starter thread
        let barrier = Arc::new(Barrier::new(n_jobs + 1));
        for _ in 0..n_jobs {
            let barrier = barrier.clone();
            let an_atomic = an_atomic.clone();

            pool.execute(move || {
                // do the heavy work
                an_atomic.fetch_add(1, Ordering::Relaxed);

                // then wait for the other threads
                barrier.wait();
            });
        }
    }

    #[test]
    fn test_scoped_threadpool() {
        use scoped_threadpool::Pool;

        let mut pool = Pool::new(4);
        let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        pool.scoped(|scope| {
            for e in &mut vec {
                scope.execute(move || {
                    *e += 1;
                });
            }
        });
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
