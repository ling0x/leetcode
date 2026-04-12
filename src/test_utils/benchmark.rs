use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt::Debug;
use std::sync::Once;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tracing::info;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static TRACING_INIT: Once = Once::new();

struct TrackingAllocator;

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static TRACKING_ALLOCATOR: TrackingAllocator = TrackingAllocator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkResult {
    pub elapsed: Duration,
    pub heap_delta_bytes: isize,
}

pub fn current_heap_allocated_bytes() -> usize {
    ALLOCATED.load(Ordering::SeqCst)
}

pub fn init_benchmark_tracing() {
    TRACING_INIT.call_once(|| {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_target(false)
            .with_level(false)
            .without_time()
            .compact()
            .try_init();
    });
}

fn format_duration(elapsed: Duration) -> String {
    let nanos = elapsed.as_nanos();
    if nanos >= 1_000_000_000 {
        format!("{:.3} s", nanos as f64 / 1_000_000_000.0)
    } else if nanos >= 1_000_000 {
        format!("{:.3} ms", nanos as f64 / 1_000_000.0)
    } else if nanos >= 1_000 {
        format!("{:.3} us", nanos as f64 / 1_000.0)
    } else {
        format!("{nanos} ns")
    }
}

fn format_signed_bytes(bytes: isize) -> String {
    let sign = if bytes >= 0 { "+" } else { "-" };
    let abs = (bytes as i128).abs() as f64;
    if abs >= 1024.0 * 1024.0 {
        format!("{sign}{:.2} MiB", abs / (1024.0 * 1024.0))
    } else if abs >= 1024.0 {
        format!("{sign}{:.2} KiB", abs / 1024.0)
    } else {
        format!("{sign}{} B", abs as u64)
    }
}

pub fn run_with_metrics<T, F>(label: &str, test: F) -> (T, BenchmarkResult)
where
    F: FnOnce() -> T,
{
    init_benchmark_tracing();

    let mem_before = current_heap_allocated_bytes() as isize;
    let start = Instant::now();

    let output = test();

    let elapsed = start.elapsed();
    let mem_after = current_heap_allocated_bytes() as isize;
    let result = BenchmarkResult {
        elapsed,
        heap_delta_bytes: mem_after - mem_before,
    };

    info!(
        "\n[benchmark] {label}\n  time       : {}\n  heap delta : {}",
        format_duration(result.elapsed),
        format_signed_bytes(result.heap_delta_bytes)
    );

    (output, result)
}

pub fn assert_vec_any_order<T>(mut actual: Vec<T>, mut expected: Vec<T>)
where
    T: Ord + Debug,
{
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected);
}

pub fn run_and_assert_vec_any_order<T, F>(label: &str, expected: Vec<T>, test: F) -> BenchmarkResult
where
    T: Ord + Debug,
    F: FnOnce() -> Vec<T>,
{
    let (actual, metrics) = run_with_metrics(label, test);
    assert_vec_any_order(actual, expected);
    metrics
}
