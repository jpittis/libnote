use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub status: Status,
    pub channel: u8,
    pub pitch: u8,
    pub velocity: u8,
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    UnknownStatus,
    InvalidPitch,
    InvalidVelocity,
}

pub type DecodeResult<T> = Result<T, DecodeError>;

impl Note {
    pub fn zero() -> Note {
        Note {
            status: Status::On,
            channel: 0,
            pitch: 0,
            velocity: 0,
        }
    }

    pub fn new(packet: &[u8; 3]) -> DecodeResult<Note> {
        let mut note = Note::zero();
        Note::decode(packet, &mut note)?;
        Ok(note)
    }

    pub fn decode(packet: &[u8; 3], note: &mut Note) -> DecodeResult<()> {
        let mut status = match packet[0] >> 4 {
            0b1001 => Status::On,
            0b1000 => Status::Off,
            _ => return Err(DecodeError::UnknownStatus),
        };
        if packet[1] >> 7 != 0 {
            return Err(DecodeError::InvalidPitch);
        }
        if packet[2] >> 7 != 0 {
            return Err(DecodeError::InvalidVelocity);
        }
        let channel = packet[0] & 0b00001111;
        let pitch = packet[1] & 0b01111111;
        let velocity = packet[2] & 0b01111111;
        if velocity == 0 {
            status = Status::Off;
        }
        note.status = status;
        note.channel = channel;
        note.pitch = pitch;
        note.velocity = velocity;
        Ok(())
    }

    pub fn encode(note: &Note, packet: &mut [u8; 3]) {
        let status = match note.status {
            Status::On => 0b1001,
            Status::Off => 0b1000,
        };
        packet[0] = 0b11110000 & status << 4;
        packet[0] |= 0b00001111 & note.channel;
        packet[1] = 0b01111111 & note.pitch;
        packet[2] = 0b01111111 & note.velocity;
    }
}

pub struct IO {
    file: fs::File,
    packet: [u8; 3],
}

impl IO {
    pub fn open(path: &str) -> io::Result<Self> {
        let f = fs::OpenOptions::new().write(true).read(true).open(path)?;
        Ok(Self {
            file: f,
            packet: [0; 3],
        })
    }

    pub fn read(&mut self, note: &mut Note) -> io::Result<()> {
        self.file.read_exact(&mut self.packet)?;
        Note::decode(&self.packet, note)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))
    }

    pub fn write(&mut self, note: &Note) -> io::Result<()> {
        Note::encode(note, &mut self.packet);
        self.file.write_all(&self.packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let note = Note {
            status: Status::On,
            channel: 1,
            pitch: 60,
            velocity: 127,
        };
        let mut packet = [0; 3];
        Note::encode(&note, &mut packet);
        let decoded = Note::new(&packet);
        assert_eq!(Ok(note), decoded);
    }
}
