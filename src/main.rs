use std::path::Path;

use rimd::{self, MidiMessage, SMFBuilder, SMFWriter, MetaEvent};

fn main() {
    let mut builder = SMFBuilder::new();
    builder.add_track();
    builder.add_midi_rel(0, 0, MidiMessage::program_change(69, 0));
    builder.add_meta_rel(0, 0, MetaEvent::tempo_setting(400000));
    builder.add_meta_rel(0, 0, MetaEvent::time_signature(4, 2, 24, 8));

    for note in 40..100 {
        builder.add_midi_rel(0, 0, MidiMessage::note_on(note, 100, 0));
        builder.add_midi_rel(0, 10, MidiMessage::pitch_bend(0, 127, 0));
        builder.add_midi_rel(0, 20, MidiMessage::pitch_bend(127, 0, 0));
        builder.add_midi_rel(0, 30, MidiMessage::note_off(note, 100, 0));
    }
    let mut smf = builder.result();
    smf.division = 120;

    let writer = SMFWriter::from_smf(smf);
    match writer.write_to_file(Path::new("test.mid")) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}