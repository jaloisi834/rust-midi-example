# MIDI Test

A simple project to test generating MIDI files with Rust!

I started this project primarily to help me understand the MIDI file format. The choice to use Rust was made because I am also trying to get used to it.

My journey from thinking "programming MIDI files would be cool" to actually understanding the essentials of interacting with MIDI files was a little rough, and I hope the documentation of my findings can help others in their journeys.

## Resources

Some reference material that I found useful while I worked on this application:

- Very high level overview of MIDI file structure: https://ccrma.stanford.edu/~craig/14q/midifile/MidiFileFormat.html
- More detailed information on MIDI file structure plus some info about meta event types: https://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
- Info on primary MIDI event types: https://www.cs.cmu.edu/~music/cmsip/readings/MIDI%20tutorial%20for%20programmers.html
- The official MIDI 1.0 specification: https://www.midi.org/specifications/midi1-specifications

Tools:

- Online MIDI player: https://midiplayer.ehubsoft.net/

## Setup

- Install Rust - https://www.rust-lang.org/tools/install

## Running

Run the application and output a MIDI file to `result.mid`.

```sh
cargo run
```

## Understanding the Output

Using `hexdump` (a tool in bash) on the result file we can its contents in hexadecimal format:

```
$ hexdump -C result.mid
00000000  4d 54 68 64 00 00 00 06  00 01 00 01 00 60 4d 54  |MThd.........`MT|
00000010  72 6b 00 00 00 93 00 ff  51 03 07 a1 20 00 ff 58  |rk......Q... ..X|
00000020  04 04 02 18 08 00 90 3c  64 60 80 3c 64 00 90 3d  |.......<d`.<d..=|
00000030  64 60 80 3d 64 00 90 3e  64 60 80 3e 64 00 90 3f  |d`.=d..>d`.>d..?|
00000040  64 60 80 3f 64 00 90 40  64 60 80 40 64 00 90 41  |d`.?d..@d`.@d..A|
00000050  64 60 80 41 64 00 90 42  64 60 80 42 64 00 90 43  |d`.Ad..Bd`.Bd..C|
00000060  64 60 80 43 64 00 90 44  64 60 80 44 64 00 90 45  |d`.Cd..Dd`.Dd..E|
00000070  64 60 80 45 64 00 90 46  64 60 80 46 64 00 90 47  |d`.Ed..Fd`.Fd..G|
00000080  64 60 80 47 64 00 90 48  64 60 80 48 64 00 90 49  |d`.Gd..Hd`.Hd..I|
00000090  64 60 80 49 64 00 90 4a  64 60 80 4a 64 00 90 4b  |d`.Id..Jd`.Jd..K|
000000a0  64 60 80 4b 64 00 ff 2f  00                       |d`.Kd../.|
```

I know this is pretty gross and low level, but it's not actually that hard to understand once you know what you are looking at. A MIDI file is just a sequence of bytes representing different instructions. Let's break it down into chunks so we can understand the instructions in this file!

The following chunks are formatted with `|` between each logical sub-chunk, and each bullet point corresponds to a sub-chunk. Also when interpreting numbers remember that they are in hexadecimal (e.g. 0x60 = 96).

### Header

The first chunk is always the file header:

```
4d 54 68 64 | 00 00 00 06 | 00 01 | 00 01 | 00 60
```

- The `MThd` marker.
- Six bytes following in the header.
- File format is "multi-track". Note: we could/should have set this to zero but this is the library default.
- One track.
- 96 ticks per quarter-note.

### Track

The next chunk starts a track:

```
4d 54 72 6b | 00 00 00 93
```

- The `MTrk` marker.
- 32 bit `147` indicating that there are 147 bytes following in the track.


Following the track header will be a series of meta and MIDI events. The first sets the tempo.

```
00 | ff | 51 | 03 | 07 a1 20
```

- Relative offset 0.
- Meta event.
- "Set tempo" type
- Three data bytes following.
- 500,000 microseconds per quarter note. *More on this in code comments*.

Next we set the time signature:

```
00 | ff | 58 | 04 | 04 | 02 | 18 | 08
```

- Relative offset 0.
- Meta event.
- "Set time signature" type.
- Four data bytes following.
- Time signature numerator is four.
- Time signature denominator is four (2^2). *More on this in code comments*.
- 24 clocks per tick. *More on this in code comments*.
- Eight 32nd notes per 24 clocks. *More on this in code comments*.

Now we have a bunch of note events:

```
00 | 90 | 3c | 64
```

- Relative offset 0.
- "Note on" type on channel one.
- Note 60.
- Velocity 100.

```
60 | 80 | 3c | 64
```

- Relative offset 96.
- "Note off" type on channel one.
- Note 60.
- Velocity 100.

This pattern continues starting and ending increasing notes:

```
00 90 3d 64
60 80 3d 64
...
00 90 4b 64
60 80 4b 64
```

Finally there is a track end chunk:

```
00 | ff | 2f | 00
```

- Relative offset 0.
- Meta event.
- "Track end" type.
- Zero bytes following.
