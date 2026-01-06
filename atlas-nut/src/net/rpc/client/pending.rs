use parking_lot::Mutex;
use slab::Slab;
use tokio::time::Instant;

/// 每个 slot 存储回调和元信息
pub struct PendingSlot<T> {
    pub request_id: u64,
    pub body: T,
    pub _timestamp: Instant,
}

/// 高性能 PendingTable
pub struct PendingTable<T> {
    slab: Mutex<Slab<PendingSlot<T>>>, // Slab 存储回调
}

impl<T> PendingTable<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            slab: Mutex::new(Slab::with_capacity(cap)),
        }
    }

    #[inline]
    pub fn insert(&self, req_id: u64, body: T) -> u32 {
        let mut slab = self.slab.lock();
        let index = slab.insert(PendingSlot {
            request_id: req_id,
            body,
            _timestamp: Instant::now(),
        });
        index as u32
    }

    #[inline]
    pub fn remove(&self, slot_index: u32) -> Option<PendingSlot<T>> {
        let mut slab = self.slab.lock();
        slab.try_remove(slot_index as usize)
    }

    pub fn _len(&self) -> usize {
        self.slab.lock().len()
    }

    pub fn drain(&self) -> Vec<PendingSlot<T>> {
        let mut slab = self.slab.lock();
        slab.drain().collect()
    }
}
