use num_enum::FromPrimitive;

#[derive(Debug, Copy, Clone, FromPrimitive)]
#[repr(u8)]
pub enum Class {
    NAV = 0x01,
    RXM = 0x02,
    INF = 0x04,
    ACK = 0x05,
    CFG = 0x06,
    UPD = 0x09,
    MON = 0x0a,
    AID = 0x0b,
    TIM = 0x0d,
    ESF = 0x10,
    MGA = 0x13,
    LOG = 0x21,
    SEC = 0x27,
    HNR = 0x28,
    #[num_enum(default)]
    Unknown = 0,
}

impl From<Class> for u8 {
    fn from(class: Class) -> Self {
        class as u8
    }
}
