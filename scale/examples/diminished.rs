use rand::Rng;
use std::io;

const C_DIM_7: [usize; 4] = [0, 3, 6, 9];
const CSHARP_DIM_7: [usize; 4] = [1, 4, 7, 10];
const D_DIM_7: [usize; 4] = [2, 5, 8, 11];

fn main() -> io::Result<()> {
    let mut iface = midi::open_file("/dev/midi1")?;
    let mut note = midi::Note::zero();
    let mut active = scale::NoteSet::default();
    let mut sent_pitch = None;

    let c_dim_7 = scale::NoteSet::new(&C_DIM_7).snapshot();
    let csharp_dim_7 = scale::NoteSet::new(&CSHARP_DIM_7).snapshot();
    let d_dim_7 = scale::NoteSet::new(&D_DIM_7).snapshot();
    let mut rng = rand::thread_rng();

    loop {
        iface.read(&mut note)?;
        active.play(&note);

        if let Some(pitch) = sent_pitch {
            iface.write(&midi::Note {
                status: midi::Status::Off,
                channel: note.channel,
                pitch,
                velocity: 0,
            })?;
            sent_pitch = None;
        }

        let diminished = match active.snapshot() {
            s if s == c_dim_7 => Some(D_DIM_7),
            s if s == csharp_dim_7 => Some(C_DIM_7),
            s if s == d_dim_7 => Some(CSHARP_DIM_7),
            _ => None,
        };

        if let Some(tonics) = diminished {
            let tonic = tonics[rng.gen_range(0, 4)];
            let pitch = 36 + tonic as u8;
            iface.write(&midi::Note {
                status: midi::Status::On,
                channel: note.channel,
                pitch,
                velocity: 60,
            })?;
            sent_pitch = Some(pitch);
        }
    }
}
