//! Stub Watcher implementation

#![allow(unused_variables)]

use super::{EventFn, RecursiveMode, Result, Watcher};
use std::path::Path;

/// Stub `Watcher` implementation
///
/// Events are never delivered from this watcher.
pub struct NullWatcher;

impl Watcher for NullWatcher {
    fn new_immediate<F: EventFn>(_event_fn: F) -> Result<NullWatcher> {
        Ok(NullWatcher)
    }

    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        Ok(())
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        Ok(())
    }
}
