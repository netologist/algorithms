use chrono::Utc;
use std::sync::Mutex;

const EPOCH: u64 = 1577836800000; // Custom epoch (January 1, 2020)
const MACHINE_ID_BITS: u8 = 5;
const SEQUENCE_BITS: u8 = 12;
pub const MACHINE_ID_MAX: u64 = (1 << MACHINE_ID_BITS) - 1;
const SEQUENCE_MAX: u64 = (1 << SEQUENCE_BITS) - 1;

pub struct SnowflakeGenerator {
    pub machine_id: u64,
    pub last_timestamp: u64,
    pub sequence: u64,
    pub lock: Mutex<()>,
}

impl SnowflakeGenerator {
    pub fn new(machine_id: u64) -> Self {
        assert!(machine_id <= MACHINE_ID_MAX, "Machine ID out of range");

        SnowflakeGenerator {
            machine_id,
            last_timestamp: 0,
            sequence: 0,
            lock: Mutex::new(()),
        }
    }

    fn current_timestamp() -> u64 {
        Utc::now().timestamp_millis() as u64
    }

    pub fn generate(&mut self) -> u64 {
        let _lock = self.lock.lock().unwrap();
        let mut timestamp = SnowflakeGenerator::current_timestamp();

        if self.last_timestamp == timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MAX;
            if self.sequence == 0 {
                while timestamp <= self.last_timestamp {
                    timestamp = SnowflakeGenerator::current_timestamp();
                }
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        ((timestamp - EPOCH) << (MACHINE_ID_BITS + SEQUENCE_BITS))
            | (self.machine_id << SEQUENCE_BITS)
            | self.sequence
    }
}