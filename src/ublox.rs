use log::{error, trace};
use std::fmt;
use std::io::Read;
use std::num::Wrapping;

use crate::{Class, Error, Id};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Sync1,
    Sync2,
    Class,
    Id,
    LengthLow,
    LengthHigh,
    Payload,
    CheckA,
    CheckB,
    Done,
}

impl Default for State {
    fn default() -> Self {
        Self::Sync1
    }
}

#[derive(Default)]
pub struct Ublox {
    pub sync_1: u8,
    pub sync_2: u8,
    pub class: u8,
    pub id: u8,
    pub length: u16,
    pub payload: Vec<u8>,
    pub check_a: u8,
    pub check_b: u8,
}

impl Ublox {
    pub fn new(class: Class, id: Id, payload: &[u8]) -> Self {
        let mut s = Self {
            sync_1: 0xb5,
            sync_2: 0x62,
            class: class.into(),
            id: id.into(),
            length: payload.len() as u16,
            payload: Vec::from(payload),
            check_a: 0,
            check_b: 0,
        };

        s.checksum_set();

        s
    }

    pub fn id(&self) -> Id {
        match self.class.into() {
            Class::NAV => Id::NAV(self.id.into()),
            Class::RXM => Id::RXM(self.id.into()),
            Class::INF => Id::INF(self.id.into()),
            Class::ACK => Id::ACK(self.id.into()),
            Class::CFG => Id::CFG(self.id.into()),
            Class::UPD => Id::UPD(self.id.into()),
            Class::MON => Id::MON(self.id.into()),
            Class::AID => Id::AID(self.id.into()),
            Class::TIM => Id::TIM(self.id.into()),
            Class::ESF => Id::ESF(self.id.into()),
            Class::MGA => Id::MGA(self.id.into()),
            Class::LOG => Id::LOG(self.id.into()),
            Class::SEC => Id::SEC(self.id.into()),
            Class::HNR => Id::HNR(self.id.into()),
            Class::Unknown => Id::Unknown,
        }
    }

    fn checksum(&self) -> (u8, u8) {
        let mut a = Wrapping(0u8);
        let mut b = Wrapping(0u8);

        a += Wrapping(u8::from(self.class));
        b += a;

        a += Wrapping(self.id);
        b += a;

        a += Wrapping((self.length & 0xff) as u8);
        b += a;

        a += Wrapping((self.length >> 8) as u8);
        b += a;

        for c in self.payload.iter() {
            a += Wrapping(*c);
            b += a;
        }

        (a.0, b.0)
    }

    fn checksum_set(&mut self) {
        let (a, b) = self.checksum();

        self.check_a = a;
        self.check_b = b;
    }

    fn checksum_validate(&self) -> Result<(), Error> {
        if (self.check_a, self.check_b) == self.checksum() {
            Ok(())
        } else {
            Err(Error::Checksum(
                (self.check_a, self.check_b),
                self.checksum(),
            ))
        }
    }

    fn parse_one(&mut self, mut state: State, c: u8) -> Result<State, Error> {
        trace!("state: {:?}, byte: {:x}", state, c);

        state = match state {
            State::Sync1 => {
                if c == 0xb5 {
                    self.sync_1 = c;
                    State::Sync2
                } else {
                    error!("bad sync1: {:x}", c);
                    return Err(Error::Parse(state));
                }
            }

            State::Sync2 => {
                if c == 0x62 {
                    self.sync_2 = c;
                    State::Class
                } else {
                    error!("bad sync2: {:x}", c);
                    return Err(Error::Parse(state));
                }
            }

            State::Class => {
                self.class = c;
                State::Id
            }

            State::Id => {
                self.id = c;
                State::LengthLow
            }

            State::LengthLow => {
                self.length = u16::from(c);
                State::LengthHigh
            }

            State::LengthHigh => {
                self.length += u16::from(c * u8::MAX);

                if self.length != 0 {
                    State::Payload
                } else {
                    State::CheckA
                }
            }

            State::Payload => {
                self.payload.push(c);

                if self.payload.len() == self.length as usize {
                    State::CheckA
                } else {
                    State::Payload
                }
            }

            State::CheckA => {
                self.check_a = c;
                State::CheckB
            }

            State::CheckB => {
                self.check_b += c;
                self.checksum_validate()?;
                State::Done
            }

            State::Done => State::Done,
        };

        Ok(state)
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, Error> {
        let mut state = State::default();
        let mut buf = vec![0];
        let mut ublox = Ublox::default();

        while state != State::Done {
            state = match reader.read_exact(&mut buf) {
                Ok(()) => ublox.parse_one(state, buf[0])?,
                Err(e) => return Err(Error::Read(e)),
            };
        }

        Ok(ublox)
    }
}

impl fmt::Display for Ublox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}",
            self.sync_1, self.sync_2, self.class, self.id
        )?;

        for c in &self.length.clone().to_le_bytes() {
            write!(f, "{:02x}", c)?;
        }

        for c in &self.payload {
            write!(f, "{:02x}", c)?;
        }

        write!(f, "{:02x}{:02x}", self.check_a, self.check_b)
    }
}

impl fmt::Debug for Ublox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}: {:x?}", self.id(), self.length, self.payload)
    }
}
