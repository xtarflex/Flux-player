#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use futures::stream::{self, StreamExt};
    use std::collections::HashMap;

    // A mock item mimicking a path that needs scanning
    #[derive(Clone, Debug)]
    struct MockFile {
        _id: usize,
        is_tv_show: bool,
        show_name: Option<String>,
    }

    #[tokio::test]
    async fn test_concurrent_show_cache_safety() {
        // We simulate a library of 100 items. 50 are movies, 50 are episodes of "Test Show"
        let mut files = Vec::new();
        for _i in 0..100 {
            if _i % 2 == 0 {
                files.push(MockFile { _id: _i, is_tv_show: true, show_name: Some("Test Show".into()) });
            } else {
                files.push(MockFile { _id: _i, is_tv_show: false, show_name: None });
            }
        }

        let show_cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let processed_count = Arc::new(AtomicUsize::new(0));
        let tmdb_calls = Arc::new(AtomicUsize::new(0));

        let stream = stream::iter(files).map(|file| {
            let cache_clone = Arc::clone(&show_cache);
            let count_clone = Arc::clone(&processed_count);
            let tmdb_clone = Arc::clone(&tmdb_calls);

            async move {
                // Simulate some I/O delay
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;

                if file.is_tv_show {
                    let name = file.show_name.unwrap();
                    let mut cache = cache_clone.lock().await;

                    if !cache.contains_key(&name) {
                        // "Call TMDB"
                        tmdb_clone.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        cache.insert(name.clone(), "Cached Metadata".into());
                    } else {
                        // Use Cache! No TMDB call needed.
                    }
                } else {
                    // Movie: always calls TMDB
                    tmdb_clone.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }

                // Emulate emitting progress
                count_clone.fetch_add(1, Ordering::SeqCst);
            }
        });

        // Run with concurrency of 10
        stream.buffer_unordered(10).collect::<Vec<()>>().await;

        assert_eq!(processed_count.load(Ordering::SeqCst), 100, "Should have processed 100 items");

        // 50 Movies = 50 TMDB calls.
        // 50 Episodes of "Test Show" = 1 TMDB call (cached).
        // Total should be 51.
        assert_eq!(tmdb_calls.load(Ordering::SeqCst), 51, "Show cache failed to prevent duplicate TMDB calls");
    }

    #[tokio::test]
    async fn test_atomic_progress_emits_order_independent() {
        let files: Vec<usize> = (0..50).collect();
        let current_progress = Arc::new(AtomicUsize::new(0));
        let total = files.len();

        let stream = stream::iter(files).map(|_id| {
            let prog_clone = Arc::clone(&current_progress);
            async move {
                tokio::time::sleep(std::time::Duration::from_millis(2)).await;

                // This is how we will emit to frontend safely
                let count = prog_clone.fetch_add(1, Ordering::SeqCst) + 1;
                // Emit event (count, total)
                assert!(count <= total);
            }
        });

        stream.buffer_unordered(5).collect::<Vec<()>>().await;

        assert_eq!(current_progress.load(Ordering::SeqCst), total, "Did not hit 100% progress");
    }
}
