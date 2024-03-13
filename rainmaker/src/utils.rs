// utility types and functions for using throughout the crate
use std::sync::{Arc, Mutex};

pub(crate) type WrappedInArcMutex<T> = Arc<Mutex<T>>;

pub(crate) fn wrap_in_arc_mutex<T>(inp: T) -> WrappedInArcMutex<T>{
    Arc::new(Mutex::new(inp))
}