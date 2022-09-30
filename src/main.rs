use std::path::Path;

use rimd::{MetaEvent, MidiMessage, SMFBuilder, SMFWriter};

const OUT_FILE: &str = "result.mid";

fn main() {
    let mut builder = SMFBuilder::new();

    builder.add_track();

    // Set the tempo to 120. This is expressed as microseconds per quarter note.
    //
    // We want `120 beats / 1 minute`, so:
    //  = 120 beats / 60 seconds
    //  = 2 beats / 1 second
    //
    // As we will set in the next line, `1 quarter note = 1 beat`, so we want `2 quarter notes / 1 second`.
    //
    // There are x microseconds in a beat such that `2 quarter_notes * x = 1 seconds`.
    // x = 1 second / 2 quarter notes
    //   = 1,000,000 microseconds / 2 quarter notes
    //   = 500,000 microseconds / 1 quarter note
    //
    builder.add_meta_rel(0, 0, MetaEvent::tempo_setting(500000));

    // Set the time signature to 4/4.
    //
    // Parameters used here:
    // - The time signature numerator. This is expressed normally, so `4=4`.
    // - The time signature denominator. This is expressed as `2^actual_denominator`, so `2^2=4`.
    // - The number of clocks per tick. Here we are saying that 24 clocks is a beat which is the typical value.
    // - The number of 32nd notes per 24 clocks. Think of this as `32nd_notes_per_clock/8=quarter_notes_per_beat`, so `8/8=1` or one quarter note per beat.
    builder.add_meta_rel(0, 0, MetaEvent::time_signature(4, 2, 24, 8));

    // Add four measures of quarter notes increasing by half steps.
    for note in 60..76 {
        builder.add_midi_rel(0, 0, MidiMessage::note_on(note, 100, 0));
        builder.add_midi_rel(0, 96, MidiMessage::note_off(note, 100, 0));
    }

    let mut smf = builder.result();

    // Set the relative timing division, also know as the resultion. This defines how long a quarter note is, expressed as ticks.
    //
    // It is sort of arbitrary and only impacts how the MIDI timings are interpreted, not the actual tempo (i.e. setting 96
    // here and using a delta time of 96 is effectively the same as setting 10,000 here and using a delta time of 10,000).
    smf.division = 96;

    let writer = SMFWriter::from_smf(smf);
    match writer.write_to_file(Path::new(OUT_FILE)) {
        Ok(_) => println!("Wrote output to {}", OUT_FILE),
        Err(err) => println!("Error writing output to {}: {}", OUT_FILE, err),
    }
}
