use crate::CACHE_MODE;
use crate::cache::CacheEntry;
use crate::protocol::{DnsQuestion, DnsRecord};
use std::collections::HashMap;
use std::time::{Duration, Instant};

type CacheHashMap = HashMap<DnsQuestion, CacheEntry>;

pub struct DnsCache {
    cache: CacheHashMap,
}

impl DnsCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, question: DnsQuestion, records: Vec<DnsRecord>, ttl: u32) {
        let entry = CacheEntry::new(records, Instant::now() + Duration::from_secs(ttl as u64));
        self.cache.insert(question, entry);
    }

    pub fn lookup(&mut self, question: &DnsQuestion) -> Option<Vec<DnsRecord>> {
        if !CACHE_MODE {
            return None;
        }

        let key = question;
        match self.cache.get(&key) {
            Some(entry) if entry.valid() => Some(entry.records.clone()),
            Some(_) => {
                self.cache.remove(&key);
                None
            }
            None => None,
        }
    }
}
