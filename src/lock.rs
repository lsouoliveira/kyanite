use once_cell::sync::Lazy;
use std::sync::Condvar;
use std::sync::Mutex;

static LOCKED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static CONDVAR: Lazy<Condvar> = Lazy::new(|| Condvar::new());

pub fn kya_acquire_lock() {
    let mut locked = LOCKED.lock().unwrap();

    while *locked {
        locked = CONDVAR.wait(locked).unwrap();
    }

    *locked = true;
}

pub fn kya_release_lock() {
    let mut locked = LOCKED.lock().unwrap();

    *locked = false;

    CONDVAR.notify_one();
}
