# libnote

I discovered I can read and write midi packets to and from my digital piano!

> Everything here assumes you're running Linux with ALSA and are able to read
> and write to /dev/midi1 or equivalent. It may work with OSX or Windows,
> I have no idea.

## Fifths Example

Plug in your midi device, and try out the following fifths example! Every time
you play a note on your keyboard, the program will simultaneously play five
semi-tones above it.

```
cargo run --example fifths
```

## Diminished Example

Plug in your midi device, and try playing a diminished 7th chord like C,
D sharp, F sharp, A! The program will play a base note that sounds good!

## Crates

- midi/ provides a note on, note off midi parser, see midi/examples/fifths
- scale/ provides working with sets of notes, see scale/examples/diminished
