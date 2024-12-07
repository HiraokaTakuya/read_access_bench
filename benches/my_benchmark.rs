use criterion::{criterion_group, criterion_main, Criterion};
use once_cell::sync::OnceCell;
use std::cell::UnsafeCell;
use std::sync::Mutex;
use std::sync::OnceLock;

// グローバル変数
static mut GLOBAL_MUT_VALUE: u32 = 0;

struct MyUnsafeCell(UnsafeCell<u32>);
unsafe impl Sync for MyUnsafeCell {}
static MY_UNSAFE_CELL_VALUE: MyUnsafeCell = MyUnsafeCell(UnsafeCell::new(0));

static ONCE_CELL_VALUE: OnceCell<u32> = OnceCell::new();

static ONCE_LOCK_VALUE: OnceLock<u32> = OnceLock::new();

static GLOBAL_MUTEX_VALUE: Mutex<u32> = Mutex::new(0);

// グローバル変数へのアクセス
fn read_global_mut_value() -> u32 {
    unsafe { GLOBAL_MUT_VALUE }
}

fn read_unsafe_cell_value() -> u32 {
    unsafe { *MY_UNSAFE_CELL_VALUE.0.get() }
}

fn read_once_cell_value() -> u32 {
    *ONCE_CELL_VALUE.get().unwrap()
}

fn read_once_lock_value() -> u32 {
    *ONCE_LOCK_VALUE.get().unwrap()
}

fn read_mutex_value() -> u32 {
    *GLOBAL_MUTEX_VALUE.lock().unwrap()
}

// ベンチマークセットアップ
fn setup_once_cell_value() {
    ONCE_CELL_VALUE.set(42).ok();
}

fn setup_once_lock_value() {
    ONCE_LOCK_VALUE.set(42).ok();
}

fn setup_benchmark() {
    unsafe {
        GLOBAL_MUT_VALUE = 42;
    }
    unsafe { *MY_UNSAFE_CELL_VALUE.0.get() = 42 };
    *GLOBAL_MUTEX_VALUE.lock().unwrap() = 42;
    setup_once_cell_value();
    setup_once_lock_value();
}

// ベンチマーク関数
fn bench_read_access(c: &mut Criterion) {
    setup_benchmark();

    c.bench_function("Read global mut value", |b| {
        b.iter(|| read_global_mut_value())
    });
    c.bench_function("Read UnsafeCell value", |b| {
        b.iter(|| read_unsafe_cell_value())
    });

    c.bench_function("Read OnceCell value", |b| b.iter(|| read_once_cell_value()));
    c.bench_function("Read OnceLock value", |b| b.iter(|| read_once_lock_value()));

    c.bench_function("Read Mutex value", |b| b.iter(|| read_mutex_value()));
}

// 必要なマクロを定義
criterion_group!(benches, bench_read_access);
criterion_main!(benches);
