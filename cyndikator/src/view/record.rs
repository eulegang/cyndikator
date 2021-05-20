use crate::db::{Database, Entry, Error};
use std::collections::VecDeque;

pub struct Cache {
    db: Database,
    loc: u32,
    total: u32,
    cache: Vec<Entry>,

    undo_queue: VecDeque<Entry>,
}

impl Cache {
    const FETCH_SIZE: u32 = 256;

    pub fn new(db: Database) -> Cache {
        let cache = Vec::new();
        let undo_queue = VecDeque::with_capacity(Self::FETCH_SIZE as usize);
        let loc = 0;
        let total = 0;
        Cache {
            db,
            loc,
            cache,
            total,
            undo_queue,
        }
    }

    pub fn window(&mut self, offset: u32, win: u32) -> Result<&[Entry], Error> {
        if self.needs_load(offset, win) {
            self.load(offset, win)?;
        }

        let rel = (offset - self.loc) as usize;
        let rel_end = (rel + win as usize).min(self.cache.len());

        Ok(&self.cache[rel..rel_end])
    }

    fn needs_load(&self, offset: u32, win: u32) -> bool {
        if self.cache.capacity() == 0 {
            return true;
        }

        let inbounds = self.loc < offset && offset + win < self.loc + Cache::FETCH_SIZE;

        !inbounds
    }

    fn load(&mut self, offset: u32, win: u32) -> Result<(), Error> {
        let mask = Cache::FETCH_SIZE - 1;
        let region = offset & !mask;
        self.loc = region;

        let mut shift = 0;
        while offset + win > region + (Cache::FETCH_SIZE << shift) {
            shift += 1;
        }

        self.cache = self.db.records(region, Cache::FETCH_SIZE << shift)?;
        self.total = self.db.count_records()?;

        Ok(())
    }

    pub fn delete(&mut self, offset: usize) {
        if offset >= self.cache.len() {
            return;
        }

        let entry = self.cache.swap_remove(offset);

        // we don't care about error here (not a good way to show to user currently)
        let _ = self.db.delete_record(entry.id);

        self.undo_queue.push_back(entry);
        self.mk_dirty();
    }

    pub fn undo(&mut self) {
        if let Some(entry) = self.undo_queue.pop_front() {
            // we don't care about error here (not a good way to show to user currently)
            let _ = self.db.insert_record(&entry);

            self.mk_dirty()
        }
    }

    pub fn total(&mut self) -> u32 {
        if self.cache.capacity() == 0 {
            if let Ok(cnt) = self.db.count_records() {
                self.total = cnt;
            }
        }

        self.total
    }

    fn mk_dirty(&mut self) {
        self.cache = Vec::new();
    }
}
