#[derive(Clone, PartialEq, Debug)]
pub struct MemorySize {
    pub blocks: i64,
    pub kb: i64,
    pub mb: i64,
    pub gb: i64,
}

pub fn memory_size_from_blocks(_blocks: i64) -> MemorySize {
    MemorySize {
        blocks: _blocks,
        kb: _blocks / 2,
        mb: _blocks / 2048,
        gb: _blocks / 2097152,
    }
}

pub fn memory_size_from_kb(_kb: i64) -> MemorySize {
    MemorySize {
        blocks: _kb * 2,
        kb: _kb,
        mb: _kb / 1024,
        gb: _kb * 1048576,
    }
}

pub fn memory_size_from_mb(_mb: i64) -> MemorySize {
    MemorySize {
        blocks: _mb * 2048,
        kb: _mb * 1024,
        mb: _mb,
        gb: _mb / 1024,
    }
}

pub fn memory_size_from_gb(_gb: i64) -> MemorySize {
    MemorySize {
        blocks: _gb * 2097152,
        kb: _gb * 1048576,
        mb: _gb * 1024,
        gb: _gb,
    }
}

pub struct Time {
    pub seconds: i32,
    pub minutes: i32,
    pub hours: i32,
}

pub fn time_from_seconds(_seconds: i32) -> Time {
    Time {
        seconds: _seconds % 60,
        minutes: (_seconds / 60) % 60,
        hours: _seconds / 3600,
    }
}

pub fn time_from_minutes(_minutes: i32) -> Time {
    Time {
        seconds: (_minutes * 60) % 60,
        minutes: _minutes,
        hours: _minutes / 60,
    }
}

pub fn time_from_hours(_hours: i32) -> Time {
    Time {
        seconds: _hours * 3600,
        minutes: _hours * 60,
        hours: _hours,
    }
}

pub struct Frequency {
    pub hz: i64,
    pub mhz: i32,
    pub ghz: f32,
}

pub fn frequency_from_hz(_hz: i64) -> Frequency {
    Frequency {
        hz: _hz,
        mhz: _hz as i32 / 1000,
        ghz: _hz as f32 / 1000000_f32,
    }
}

pub fn frequency_from_mhz(_mhz: i32) -> Frequency {
    Frequency {
        hz: _mhz as i64 * 1000,
        mhz: _mhz,
        ghz: _mhz as f32 / 1000_f32,
    }
}

pub struct Size2D {
    pub width: usize,
    pub height: usize,
}

impl Size2D {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}
