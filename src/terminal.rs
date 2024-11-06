use std::io::{self, Read};

use termion::raw::IntoRawMode;

pub fn read_raw() -> Option<u16> {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut buffer = [0u8];
    io::stdin().read_exact(&mut buffer).unwrap();
    buffer[0] = match buffer[0] {
        b'\r' => b'\n',
        3 => return None, // Ctrl-C
        x => x,
    };
    Some(buffer[0] as u16)
}
