use bytes::Bytes;
use parking_lot::Mutex;
use slab::Slab;
use std::pin::Pin;
use tokio::time::Instant;

pub type AsyncCallback = Box<dyn FnOnce(Bytes) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

/// 每个 slot 存储回调和元信息
pub struct PendingSlot {
    pub request_id: u64,
    pub callback: Box<dyn FnOnce(Bytes) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>,
    pub _timestamp: Instant,
}

/// 高性能 PendingTable
pub struct PendingTable {
    slab: Mutex<Slab<PendingSlot>>, // Slab 存储回调
}

impl PendingTable {
    pub fn new(cap: usize) -> Self {
        Self {
            slab: Mutex::new(Slab::with_capacity(cap)),
        }
    }

    #[inline]
    pub fn insert(&self, req_id: u64, callback: AsyncCallback) -> u32 {
        let mut slab = self.slab.lock();
        let index = slab.insert(PendingSlot {
            request_id: req_id,
            callback,
            _timestamp: Instant::now(),
        });
        index as u32
    }

    #[inline]
    pub fn remove(&self, slot_index: u32) -> Option<PendingSlot> {
        let mut slab = self.slab.lock();
        slab.try_remove(slot_index as usize)
    }

    pub fn _len(&self) -> usize {
        self.slab.lock().len()
    }

    // pub fn drain<F>(&self, mut f: F)
    // where
    //     F: FnMut(PendingSlot),
    // {
    //     let mut slab = self.slab.lock();
    //     for slot in slab.drain() {
    //         f(slot);
    //     }
    // }

    pub fn drain(&self) -> Vec<PendingSlot> {
        let mut slab = self.slab.lock();
        slab.drain().collect()
    }
}
