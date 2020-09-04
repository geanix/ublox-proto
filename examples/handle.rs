use ublox_proto::{id, Error, Id, Ublox};

fn main() -> Result<(), Error> {
    let data: Vec<u8> = vec![0xb5, 0x62, 0x13, 0x80, 0x00, 0x00, 0x93, 0xcc];
    let ublox = Ublox::from_reader(&data[..])?;

    match ublox.id() {
        Id::MON(id) => match id {
            id::MON::VER => {
                let sw = String::from_utf8_lossy(&ublox.payload[..30]);
                let hw = String::from_utf8_lossy(&ublox.payload[30..40]);

                println!("mon-ver: sw: {}, hw: {}", sw, hw);

                for item in ublox.payload[40..].chunks(30) {
                    let s = String::from_utf8_lossy(item);

                    println!("mon-ver: {}", s);
                }
            }
            _ => println!("{:x?}", ublox),
        },

        Id::MGA(id) => match id {
            id::MGA::ACKDATA0 => {
                if ublox.payload[0] == 0 {
                    println!(
                        "MGA-ACK: not used: {:x}: {:x}",
                        ublox.payload[3], ublox.payload[2]
                    );
                } else {
                    let bytes = [
                        ublox.payload[4],
                        ublox.payload[5],
                        ublox.payload[6],
                        ublox.payload[7],
                    ];
                    let count = u32::from_le_bytes(bytes);

                    println!("MGA-ACK: used: {}: {}", ublox.payload[3], count);
                }
            }
            id::MGA::DBD => println!("{:?}", ublox),
            _ => println!("{:x?}", ublox),
        },

        Id::ACK(id) => match id {
            id::ACK::ACK => println!(
                "ACK-ACK: class: {:x}, id: {:x}",
                ublox.payload[0], ublox.payload[1]
            ),
            id::ACK::NAK => println!(
                "ACK-NAK: class: {:x}, id: {:x}",
                ublox.payload[0], ublox.payload[1]
            ),
            _ => println!("{:x?}", ublox),
        },

        Id::NAV(_) => {}

        _ => println!("{:x?}", ublox),
    }

    Ok(())
}
