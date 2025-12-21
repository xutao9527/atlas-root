use slab::Slab;
use parking_lot::Mutex;
use tokio::time::Instant;
use crate::net::packet::{Packet, Request};

/// 每个 slot 存储回调和元信息
pub struct PendingSlot {
    pub request_id: u64,
    pub callback: Box<dyn FnOnce(Packet) + Send + 'static>,
    pub _timestamp: Instant,
}

/// 高性能 PendingTable
pub struct PendingTable {
    slab: Mutex<Slab<PendingSlot>>, // Slab存储回调
}

impl PendingTable {
    pub fn new(cap: usize) -> Self {
        Self {
            slab: Mutex::new(Slab::with_capacity(cap)),
        }
    }

    #[inline]
    pub fn insert(
        &self,
        req: &mut Request,
        callback: Box<dyn FnOnce(Packet) + Send + 'static>,
    ) {
        let mut slab = self.slab.lock();
        let index = slab.insert(PendingSlot {
            request_id: req.id,
            callback,
            _timestamp: Instant::now(),
        });
        req.slot_index = index;
    }

    #[inline]
    pub fn remove(&self, slot_index: usize) -> Option<PendingSlot> {
        let mut slab = self.slab.lock();
        slab.try_remove(slot_index)
    }

    pub fn _len(&self) -> usize {
        self.slab.lock().len()
    }

    pub fn drain<F>(&self, mut f: F)
    where
        F: FnMut(PendingSlot),
    {
        let mut slab = self.slab.lock();
        for slot in slab.drain() {
            f(slot);
        }
    }
}