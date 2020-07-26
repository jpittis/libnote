#[derive(Default)]
pub struct NoteSet([u8; 12]);

impl NoteSet {
    pub fn new(pitches: &[usize]) -> Self {
        let mut set = NoteSet::default();
        for &p in pitches {
            set.add(p);
        }
        set
    }

    pub fn play(&mut self, note: &midi::Note) {
        match note.status {
            midi::Status::On => self.add(note.pitch as usize),
            midi::Status::Off => self.del(note.pitch as usize),
        }
    }

    pub fn add(&mut self, pitch: usize) {
        self.0[pitch % 12] += 1;
    }

    pub fn del(&mut self, pitch: usize) {
        self.0[pitch % 12] -= 1;
    }

    pub fn snapshot(&self) -> u16 {
        let mut snapshot: u16 = 0;
        for i in 0..12 {
            if self.0[i] > 0 {
                snapshot |= 1 << i;
            }
        }
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_set() {
        let mut set = NoteSet::default();
        assert_eq!(set.snapshot(), 0x0000);
        set.add(5);
        set.add(5);
        set.add(6);
        assert_eq!(set.snapshot(), 0x0060);
        set.del(5);
        set.del(6);
        assert_eq!(set.snapshot(), 0x0020);
        set.del(5);
        assert_eq!(set.snapshot(), 0x0000);
    }
}
