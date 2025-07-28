use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::MutexGuard;

static GIL: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

thread_local! {
    static GIL_GUARD: RefCell<Option<MutexGuard<'static, ()>>> = RefCell::new(None);
}

pub fn kya_acquire_lock() {
    GIL_GUARD.with(|cell| {
        let guard = GIL.lock().unwrap();
        *cell.borrow_mut() = Some(guard);
    });
}

pub fn kya_release_lock() {
    GIL_GUARD.with(|cell| {
        *cell.borrow_mut() = None;
    });
}
