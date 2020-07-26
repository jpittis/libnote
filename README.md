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

## Crates

- midi/ provides a note on, note off midi parser, see midi/examples/fifths
