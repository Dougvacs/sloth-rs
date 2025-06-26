use std::ops::{Deref};
use std::sync::{Arc};
use parking_lot::Mutex;

pub struct Ptr<T>(Arc<Mutex<T>>);
impl<T: Clone> Ptr<T> {
    pub fn new(val: T) -> Self {
        Self(Arc::new(Mutex::new(val)))
    }
    pub fn set(&mut self, val: T) {
        let mut v = self.0.lock();
        *v = val
    }
    pub fn clone_inner(&self) -> T {
        self.0.lock().deref().clone()
    }
}