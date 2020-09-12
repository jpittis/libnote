use std::io;

fn main() -> io::Result<()> {
    let mut iface = midi::open_file("/dev/midi1")?;
    let mut note = midi::Note::zero();
    loop {
        iface.read(&mut note)?;
        note.pitch += 7;
        iface.write(&note)?;
    }
}
