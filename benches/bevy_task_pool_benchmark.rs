use criterion::{criterion_group, criterion_main, Criterion};
use bevy_tasks::TaskPoolBuilder;
use std::time::{Instant, Duration};

pub fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

fn short_task_4_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(4)
        .build();
    group.sample_size(10)
        .bench_function(
        "10000 short tasks 4 workers",
        |b| b.iter(|| {
            let task_count = 10000;
            let _finished = pool.scope(|s| {
                for _ in 0..task_count {
                    s.spawn(async move {
                        fibonacci(500_000)
                    })
                }
            });
        })
    );
}

fn medium_task_4_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(4)
        .build();
    group.sample_size(10)
        .bench_function(
        "1000 medium tasks 4 workers",
        |b| b.iter(|| {
            let task_count = 1000;
            let _finished = pool.scope(|s| {
                for _ in 0..task_count {
                    s.spawn(async move {
                        fibonacci(5_000_000)
                    })
                }
            });
        })
    );
}

fn long_task_4_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(4)
        .build();
    group.sample_size(10)
        .bench_function(
        "100 long tasks 4 workers",
        |b| b.iter(|| {
            let task_count = 100;
            let _finished = pool.scope(|s| {
                for _ in 0..task_count {
                    s.spawn(async move {
                        fibonacci(50_000_000)
                    })
                }
            });
        })
    );
}

fn short_task_8_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(8)
        .build();
    group.sample_size(10)
        .bench_function(
            "20000 short tasks 8 workers",
            |b| b.iter(|| {
                let task_count = 20000;
                let _finished = pool.scope(|s| {
                    for _ in 0..task_count {
                        s.spawn(async move {
                            fibonacci(500_000)
                        })
                    }
                });
            })
        );
}

fn medium_task_8_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(8)
        .build();
    group.sample_size(10)
        .bench_function(
            "2000 medium tasks 8 workers",
            |b| b.iter(|| {
                let task_count = 2000;
                let _finished = pool.scope(|s| {
                    for _ in 0..task_count {
                        s.spawn(async move {
                            fibonacci(5_000_000)
                        })
                    }
                });
            })
        );
}

fn long_task_8_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(8)
        .build();
    group.sample_size(10)
        .bench_function(
            "200 long tasks 8 workers",
            |b| b.iter(|| {
                let task_count = 200;
                let _finished = pool.scope(|s| {
                    for _ in 0..task_count {
                        s.spawn(async move {
                            fibonacci(50_000_000)
                        })
                    }
                });
            })
        );
}

fn short_task_16_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(16)
        .build();
    group.sample_size(10)
        .bench_function(
        "40000 short tasks 16 workers",
        |b| b.iter(|| {
            let task_count = 40000;
            let _finished = pool.scope(|s| {
                for _ in 0..task_count {
                    s.spawn(async move {
                        fibonacci(500_000)
                    })
                }
            });
        })
    );
}

fn medium_task_16_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(16)
        .build();
    group.sample_size(10)
        .bench_function(
        "4000 medium tasks 16 workers",
        |b| b.iter(|| {
            let task_count = 4000;
            let _finished = pool.scope(|s| {
                for _ in 0..task_count {
                    s.spawn(async move {
                        fibonacci(5_000_000)
                    })
                }
            });
        })
    );
}

fn long_task_16_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(16)
        .build();
    group.sample_size(10)
        .bench_function(
        "400 long tasks 16 workers",
        |b| b.iter(|| {
            let task_count = 400;
            let _finished = pool.scope(|s| {
                for _ in 0..task_count {
                    s.spawn(async move {
                        fibonacci(50_000_000)
                    })
                }
            });
        })
    );
}

fn short_task_32_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(32)
        .build();
    group.sample_size(10)
        .bench_function(
            "80000 short tasks 32 workers",
            |b| b.iter(|| {
                let task_count = 80000;
                let _finished = pool.scope(|s| {
                    for _ in 0..task_count {
                        s.spawn(async move {
                            fibonacci(500_000)
                        })
                    }
                });
            })
        );
}

fn medium_task_32_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(32)
        .build();
    group.sample_size(10)
        .bench_function(
            "8000 medium tasks 32 workers",
            |b| b.iter(|| {
                let task_count = 8000;
                let _finished = pool.scope(|s| {
                    for _ in 0..task_count {
                        s.spawn(async move {
                            fibonacci(5_000_000)
                        })
                    }
                });
            })
        );
}

fn long_task_32_workers(c: &mut Criterion) {
    let mut group = c.benchmark_group("task pool bench");
    let pool = TaskPoolBuilder::new()
        .thread_name("Busy Behavior ThreadPool".to_string())
        .num_threads(32)
        .build();
    group.sample_size(10)
        .bench_function(
            "800 long tasks 32 workers",
            |b| b.iter(|| {
                let task_count = 800;
                let _finished = pool.scope(|s| {
                    for _ in 0..task_count {
                        s.spawn(async move {
                            fibonacci(50_000_000)
                        })
                    }
                });
            })
        );
}
criterion_group!(
benches,
short_task_4_workers,
medium_task_4_workers,
long_task_4_workers,
short_task_8_workers,
medium_task_8_workers,
long_task_8_workers,
short_task_16_workers,
medium_task_16_workers,
long_task_16_workers,
short_task_32_workers,
medium_task_32_workers,
long_task_32_workers
);
criterion_main!(benches);