use num_enum::FromPrimitive;
use std::fmt;

use crate::Class;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Id {
    NAV(NAV),
    RXM(RXM),
    INF(INF),
    ACK(ACK),
    CFG(CFG),
    UPD(UPD),
    MON(MON),
    AID(AID),
    TIM(TIM),
    ESF(ESF),
    MGA(MGA),
    LOG(LOG),
    SEC(SEC),
    HNR(HNR),
    Unknown,
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::NAV(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::RXM(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::INF(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::ACK(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::CFG(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::UPD(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::MON(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::AID(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::TIM(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::ESF(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::MGA(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::LOG(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::SEC(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::HNR(id) => write!(f, "{:?}-{:?}", Class::from(*self), id),
            Id::Unknown => write!(f, "{:?}", Class::from(*self)),
        }
    }
}

impl From<Id> for u8 {
    fn from(id: Id) -> Self {
        match id {
            Id::NAV(nav) => nav as u8,
            Id::RXM(rxm) => rxm as u8,
            Id::INF(inf) => inf as u8,
            Id::ACK(ack) => ack as u8,
            Id::CFG(cfg) => cfg as u8,
            Id::UPD(upd) => upd as u8,
            Id::MON(mon) => mon as u8,
            Id::AID(aid) => aid as u8,
            Id::TIM(tim) => tim as u8,
            Id::ESF(esf) => esf as u8,
            Id::MGA(mga) => mga as u8,
            Id::LOG(log) => log as u8,
            Id::SEC(sec) => sec as u8,
            Id::HNR(hnr) => hnr as u8,
            Id::Unknown => 0,
        }
    }
}

impl From<Id> for Class {
    fn from(id: Id) -> Self {
        match id {
            Id::NAV(_nav) => Class::NAV,
            Id::RXM(_rxm) => Class::RXM,
            Id::INF(_inf) => Class::INF,
            Id::ACK(_ack) => Class::ACK,
            Id::CFG(_cfg) => Class::CFG,
            Id::UPD(_upd) => Class::UPD,
            Id::MON(_mon) => Class::MON,
            Id::AID(_aid) => Class::AID,
            Id::TIM(_tim) => Class::TIM,
            Id::ESF(_esf) => Class::ESF,
            Id::MGA(_mga) => Class::MGA,
            Id::LOG(_log) => Class::LOG,
            Id::SEC(_sec) => Class::SEC,
            Id::HNR(_hnr) => Class::HNR,
            Id::Unknown => Class::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum ACK {
    NAK = 0x00,
    ACK = 0x01,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum AID {
    INI = 0x01,
    HUI = 0x02,
    ALM = 0x30,
    EPH = 0x31,
    AOP = 0x33,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum CFG {
    PRT = 0x00,
    MSG = 0x01,
    INF = 0x02,
    RST = 0x04,
    DAT = 0x06,
    RATE = 0x08,
    CFG = 0x09,
    RXM = 0x11,
    ANT = 0x13,
    SBAS = 0x16,
    NMEA = 0x17,
    USB = 0x1b,
    ODO = 0x1e,
    NAVX5 = 0x23,
    NAV5 = 0x24,
    TP5 = 0x31,
    RINV = 0x34,
    ITFM = 0x39,
    PM2 = 0x3b,
    TMODE2 = 0x3d,
    GNSS = 0x3e,
    LOGFILTER = 0x47,
    TXSLOT = 0x53,
    PWR = 0x57,
    HNR = 0x5c,
    ESRC = 0x60,
    DOSC = 0x61,
    SMGR = 0x62,
    GEOFENCE = 0x69,
    DGNSS = 0x70,
    TMODE3 = 0x71,
    PMS = 0x86,
    SLAS = 0x8d,
    BATCH = 0x93,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum ESF {
    MEAS = 0x02,
    RAW = 0x03,
    STATUS = 0x10,
    INS = 0x15,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum HNR {
    PVT = 0x00,
    INS = 0x02,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum INF {
    ERROR = 0x00,
    WARNING = 0x01,
    NOTICE = 0x02,
    TEST = 0x03,
    DEBUG = 0x04,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum LOG {
    ERASE = 0x03,
    STRING = 0x04,
    CREATE = 0x07,
    INFO = 0x08,
    RETRIEVE = 0x09,
    RETRIEVEPOS = 0x0b,
    RETRIEVESTRING = 0x0d,
    FINDTIME = 0x0e,
    RETRIEVEPOSEXTRA = 0x0f,
    RETRIEVEBATCH = 0x10,
    BATCH = 0x11,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum MGA {
    GPS = 0x00,
    GAL = 0x02,
    BDS = 0x03,
    QZSS = 0x05,
    GLO = 0x06,
    ANO = 0x20,
    FLASH = 0x21,
    INI = 0x40,
    ACKDATA0 = 0x60,
    DBD = 0x80,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum MON {
    IO = 0x02,
    VER = 0x04,
    MSGPP = 0x06,
    RXBUF = 0x07,
    TXBUF = 0x08,
    HW = 0x09,
    HW2 = 0x0b,
    RXR = 0x21,
    PATCH = 0x27,
    GNSS = 0x28,
    SMGR = 0x2e,
    BATCH = 0x32,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum NAV {
    POSECEF = 0x01,
    POSLLH = 0x02,
    STATUS = 0x03,
    DOP = 0x04,
    ATT = 0x05,
    SOL = 0x06,
    PVT = 0x07,
    ODO = 0x09,
    RESETODO = 0x10,
    VELECEF = 0x11,
    VELNED = 0x12,
    HPPOSECEF = 0x13,
    HPPOSLLH = 0x14,
    TIMEGPS = 0x20,
    TIMEUTC = 0x21,
    CLOCK = 0x22,
    GIMEGLO = 0x23,
    TIMEBDS = 0x24,
    TIMEGAL = 0x25,
    TIMEELS = 0x26,
    NMI = 0x28,
    SVINFO = 0x30,
    DGPS = 0x31,
    SBAS = 0x32,
    ORB = 0x34,
    SAT = 0x35,
    GEOFENCE = 0x39,
    SVIN = 0x3b,
    RELPOSNED = 0x3c,
    SLAS = 0x42,
    AOPSTATUS = 0x60,
    EOE = 0x61,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum RXM {
    SFRBX = 0x13,
    MEASX = 0x14,
    RAWX = 0x15,
    SVSI = 0x20,
    RTCM = 0x32,
    PMREQ = 0x41,
    RLM = 0x59,
    IMES = 0x61,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum SEC {
    UNIQID = 0x03,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum TIM {
    TP = 0x01,
    TM2 = 0x03,
    SVIN = 0x04,
    VRFY = 0x06,
    DOSC = 0x11,
    TOS = 0x12,
    SMEAS = 0x13,
    VCOCAL = 0x15,
    FCHG = 0x16,
    HOC = 0x17,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum UPD {
    SOS = 0x14,
    #[num_enum(default)]
    Unknown,
}
