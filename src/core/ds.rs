pub(in crate::core) struct DS0 {
    pub(in crate::core) v0: u8,
}

impl DS0 {
    pub(in crate::core) fn new() -> Self {
        Self { 
            v0: 0x21 
        }
    }
}

pub(in crate::core) struct DS1 {
    pub(in crate::core) v0: u8,
    pub(in crate::core) v1: u8,
    pub(in crate::core) v2: u8,
    pub(in crate::core) v3: u8,
    pub(in crate::core) v4: u8,
    pub(in crate::core) v5: u8,
    pub(in crate::core) v6: u8,
}

impl DS1 {
    pub(in crate::core) fn new() -> Self {
        Self { 
            v0: 0x00,
            v1: 0x16,
            v2: 0x04,
            v3: 0x12,
            v4: 0x0E,
            v5: 0x0C,
            v6: 0x04,
        }
    }
}

pub(in crate::core) struct DS2 {
    pub(in crate::core) v0: u8,
    pub(in crate::core) v1: u8,
    pub(in crate::core) v2: u8,
}

impl DS2 {
    pub(in crate::core) fn new() -> Self {
        Self { 
            v0: 0x00,
            v1: 0x11,
            v2: 0x04,
        }
    }
}

pub(in crate::core) struct DS3 {
    pub(in crate::core) v0: u8,
    pub(in crate::core) v1: u8,
    pub(in crate::core) v2: u8,
    pub(in crate::core) v3: u8,
    pub(in crate::core) v4: u8,
    pub(in crate::core) v5: u8,
}

impl DS3 {
    pub(in crate::core) fn new() -> Self {
        Self { 
            v0: 0x02,
            v1: 0x0E,
            v2: 0x11,
            v3: 0x0F,
            v4: 0x12,
            v5: 0x04,
        }
    }
}

pub(in crate::core) struct DS4 {
    pub(in crate::core) v0: u8,
    pub(in crate::core) v1: u8,
    pub(in crate::core) v2: u8,
    pub(in crate::core) v3: u8,
    pub(in crate::core) v4: u8,
    pub(in crate::core) v5: u8,
    pub(in crate::core) v6: u8,
    pub(in crate::core) v7: u8,
}

impl DS4 {
    pub(in crate::core) fn new() -> Self {
        Self { 
            v0: 0x02,
            v1: 0x00,
            v2: 0x0D,
            v3: 0x0D,
            v4: 0x08,
            v5: 0x01,
            v6: 0x00,
            v7: 0x0B,
        }
    }
}