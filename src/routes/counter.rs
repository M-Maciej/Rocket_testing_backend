
use rocket::request::{self, Request};
use rocket::{State, http::Cookie, http::CookieJar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Counter Struct
pub struct Counter {
    pub count: AtomicUsize,
}

impl Counter {
    pub fn new(initial_value: usize) -> Self {
        Counter {
            count: AtomicUsize::new(initial_value),
        }
    }

    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::Relaxed) + 1
    }
}
