#[derive(Debug, PartialEq)]
pub enum Status {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
pub struct Note {
    status: Status,
    channel: u8,
    pitch: u8,
    velocity: u8,
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    UnknownStatus,
    InvalidPitch,
    InvalidVelocity,
}

pub type DecodeResult<T> = Result<T, DecodeError>;

impl Note {
    pub fn new(packet: &[u8; 3]) -> DecodeResult<Note> {
        let mut note = Note {
            status: Status::On,
            channel: 0,
            pitch: 0,
            velocity: 0,
        };
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
