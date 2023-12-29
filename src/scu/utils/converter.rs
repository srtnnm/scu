#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct MemorySize {
    pub blocks: i64,
    pub kb: i64,
    pub mb: i64,
    pub gb: i64,
}

impl MemorySize {
    pub fn new() -> MemorySize {
        MemorySize {
            blocks: 0,
            kb: 0,
            mb: 0,
            gb: 0,
        }
    }
    pub fn from_blocks(_blocks: i64) -> MemorySize {
        MemorySize {
            blocks: _blocks,
            kb: _blocks / 2,
            mb: _blocks / 2048,
            gb: _blocks / 2097152,
        }
    }

    pub fn from_kb(_kb: i64) -> MemorySize {
        MemorySize {
            blocks: _kb * 2,
            kb: _kb,
            mb: _kb / 1024,
            gb: _kb * 1048576,
        }
    }

    pub fn from_mb(_mb: i64) -> MemorySize {
        MemorySize {
            blocks: _mb * 2048,
            kb: _mb * 1024,
            mb: _mb,
            gb: _mb / 1024,
        }
    }

    pub fn from_gb(_gb: i64) -> MemorySize {
        MemorySize {
            blocks: _gb * 2097152,
            kb: _gb * 1048576,
            mb: _gb * 1024,
            gb: _gb,
        }
    }
}

impl std::ops::Add for MemorySize {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            blocks: self.blocks + other.blocks,
            kb: self.kb + other.kb,
            mb: self.mb + other.mb,
            gb: self.gb + other.gb,
        }
    }
}

impl std::ops::AddAssign for MemorySize {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            blocks: self.blocks + other.blocks,
            kb: self.kb + other.kb,
            mb: self.mb + other.mb,
            gb: self.gb + other.gb,
        };
    }
}

pub struct Time {
    pub seconds: i32,
    pub minutes: i32,
    pub hours: i32,
}

impl Time {
    pub fn new() -> Time {
        Time {
            seconds: 0,
            minutes: 0,
            hours: 0,
        }
    }
    pub fn from_seconds(_seconds: i32) -> Time {
        Time {
            seconds: _seconds % 60,
            minutes: (_seconds / 60) % 60,
            hours: _seconds / 3600,
        }
    }

    pub fn from_minutes(_minutes: i32) -> Time {
        Time {
            seconds: (_minutes * 60) % 60,
            minutes: _minutes,
            hours: _minutes / 60,
        }
    }

    pub fn from_hours(_hours: i32) -> Time {
        Time {
            seconds: _hours * 3600,
            minutes: _hours * 60,
            hours: _hours,
        }
    }
}

pub struct Frequency {
    pub hz: i64,
    pub mhz: i32,
    pub ghz: f32,
}

impl Frequency {
    pub fn new() -> Frequency {
        Frequency {
            hz: 0,
            mhz: 0,
            ghz: 0.0,
        }
    }
    pub fn from_hz(_hz: i64) -> Frequency {
        Frequency {
            hz: _hz,
            mhz: _hz as i32 / 1000,
            ghz: _hz as f32 / 1000000_f32,
        }
    }

    pub fn from_mhz(_mhz: i32) -> Frequency {
        Frequency {
            hz: _mhz as i64 * 1000,
            mhz: _mhz,
            ghz: _mhz as f32 / 1000_f32,
        }
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
