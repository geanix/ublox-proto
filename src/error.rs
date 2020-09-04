#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Parse(crate::ublox::State),
    Checksum((u8, u8), (u8, u8)),
}
