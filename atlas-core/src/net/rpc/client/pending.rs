use parking_lot::Mutex;
use slab::Slab;
use tokio::time::Instant;

/// 每个 slot 存储回调和元信息
pub struct PendingSlot<R> {
    pub request_id: u64,
    pub callback: Box<dyn FnOnce(R) + Send + 'static>,
    pub _timestamp: Instant,
}


/// 高性能 PendingTable
pub struct PendingTable<R> {
    slab: Mutex<Slab<PendingSlot<R>>>, // Slab存储回调
}


impl<R> PendingTable<R> {
    pub fn new(cap: usize) -> Self {
        Self {
            slab: Mutex::new(Slab::with_capacity(cap)),
        }
    }

    #[inline]
    pub fn insert(
        &self,
        req_id: u64,
        callback: Box<dyn FnOnce(R) + Send + 'static>,
    ) -> usize
    {
        let mut slab = self.slab.lock();
        let index = slab.insert(PendingSlot {
            request_id: req_id,
            callback,
            _timestamp: Instant::now(),
        });
        index
    }

    #[inline]
    pub fn remove(&self, slot_index: usize) -> Option<PendingSlot<R>> {
        let mut slab = self.slab.lock();
        slab.try_remove(slot_index)
    }

    pub fn _len(&self) -> usize {
        self.slab.lock().len()
    }

    pub fn drain<F>(&self, mut f: F)
    where
        F: FnMut(PendingSlot<R>),
    {
        let mut slab = self.slab.lock();
        for slot in slab.drain() {
            f(slot);
        }
    }
}

