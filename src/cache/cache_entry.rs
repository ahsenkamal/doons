use std::time::Instant;

use crate::protocol::DnsRecord;

pub struct CacheEntry {
    pub records: Vec<DnsRecord>,
    pub expires_at: Instant,
}

impl CacheEntry {
    pub fn new(records: Vec<DnsRecord>, expires_at: Instant) -> Self {
        Self {
            records,
            expires_at,
        }
    }

    pub fn valid(&self) -> bool {
        Instant::now() < self.expires_at
    }
}
