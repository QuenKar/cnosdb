use crate::TseriesFamilyId;
use std::sync::{
    atomic::{AtomicU32, AtomicU64, Ordering},
    Arc,
};

#[derive(Default, Debug)]
pub struct GlobalContext {
    file_id: AtomicU64,
    mem_seq: AtomicU64,
    last_seq: AtomicU64,
    tsfamily_id: AtomicU32,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            file_id: AtomicU64::new(0),
            mem_seq: AtomicU64::new(0),
            last_seq: AtomicU64::new(0),
            tsfamily_id: AtomicU32::new(0),
        }
    }
}

impl GlobalContext {
    pub fn file_id(&self) -> u64 {
        self.file_id.load(Ordering::Acquire)
    }

    /// # Examples
    ///  ```
    ///  assert_eq!(foo.file_id_next(), 0);
    ///  assert_eq!(foo.file_id(), 1);
    /// ```
    pub fn file_id_next(&self) -> u64 {
        self.file_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn mem_seq_next(&self) -> u64 {
        self.mem_seq.fetch_add(1, Ordering::SeqCst)
    }

    pub fn last_seq(&self) -> u64 {
        self.last_seq.load(Ordering::Acquire)
    }

    pub fn tsfamily_id(&self) -> TseriesFamilyId {
        self.tsfamily_id.load(Ordering::Acquire)
    }

    /// # Examples
    ///  ```
    ///  assert_eq!(foo.tsfamily_id_next(), 0);
    ///  assert_eq!(foo.tsfamily_id(), 1);
    /// ```
    pub fn tsfamily_id_next(&self) -> TseriesFamilyId {
        self.tsfamily_id.fetch_add(1, Ordering::SeqCst)
    }
    pub fn fetch_add_log_seq(&self, n: u64) -> u64 {
        self.file_id.fetch_add(n, Ordering::SeqCst)
    }

    pub fn set_last_seq(&self, v: u64) {
        self.last_seq.store(v, Ordering::Release);
    }
    pub fn set_file_id(&self, v: u64) {
        self.file_id.store(v, Ordering::Release);
    }
    pub fn set_tsfamily_id(&self, v: TseriesFamilyId) {
        self.tsfamily_id.store(v, Ordering::Release);
    }

    pub fn mark_log_number_used(&self, v: u64) {
        let mut old = self.file_id.load(Ordering::Acquire);
        while old <= v {
            match self
                .file_id
                .compare_exchange(old, v + 1, Ordering::SeqCst, Ordering::SeqCst)
            {
                Ok(_) => break,
                Err(x) => old = x,
            }
        }
    }
}
