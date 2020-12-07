use euphony::{
    midi::{codec::EncoderBuffer, key::Key, message::MIDIMessage},
    pitch::{
        interval::Interval,
        mode::{heptatonic as h, pentatonic as p, Mode},
    },
};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[macro_use]
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const MODES: [(&'static str, &'static str, Mode); 19] = [
    ("Heptatonic - Prima", "IONIAN", h::IONIAN),
    ("Heptatonic - Prima", "DORIAN", h::DORIAN),
    ("Heptatonic - Prima", "PHRYGIAN", h::PHRYGIAN),
    ("Heptatonic - Prima", "LYDIAN", h::LYDIAN),
    ("Heptatonic - Prima", "MIXOLYDIAN", h::MIXOLYDIAN),
    ("Heptatonic - Prima", "AEOLIAN", h::AEOLIAN),
    ("Heptatonic - Prima", "LOCRIAN", h::LOCRIAN),
    (
        "Heptatonic - Secunda",
        "MELODIC_ASCENDING_MINOR",
        h::MELODIC_ASCENDING_MINOR,
    ),
    (
        "Heptatonic - Secunda",
        "PHRYGIAN_RAISED_SIXTH",
        h::PHRYGIAN_RAISED_SIXTH,
    ),
    (
        "Heptatonic - Secunda",
        "LYDIAN_RAISED_FIFTH",
        h::LYDIAN_RAISED_FIFTH,
    ),
    ("Heptatonic - Secunda", "ACOUSTIC", h::ACOUSTIC),
    ("Heptatonic - Secunda", "MAJOR_MINOR", h::MAJOR_MINOR),
    (
        "Heptatonic - Secunda",
        "HALF_DIMINISHED",
        h::HALF_DIMINISHED,
    ),
    ("Heptatonic - Secunda", "ALTERED", h::ALTERED),
    ("Pentatonic", "MAJOR", p::MAJOR),
    ("Pentatonic", "EGYPTIAN", p::EGYPTIAN),
    ("Pentatonic", "BLUES_MINOR", p::BLUES_MINOR),
    ("Pentatonic", "BLUES_MAJOR", p::BLUES_MAJOR),
    ("Pentatonic", "MINOR", p::MINOR),
];

const TONICS: [&'static str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[wasm_bindgen]
pub fn mode_len() -> usize {
    MODES.len()
}

#[wasm_bindgen]
pub fn mode_system(id: usize) -> String {
    MODES[id].0.to_string()
}

#[wasm_bindgen]
pub fn mode_name(id: usize) -> String {
    MODES[id].1.to_string()
}

#[wasm_bindgen]
pub fn tonic_len() -> usize {
    TONICS.len()
}

#[wasm_bindgen]
pub fn tonic_name(id: usize) -> String {
    TONICS[id].to_string()
}

// #[wasm_bindgen]
// pub fn decode_lumina(input: &[u8]) -> JsValue {
//     for message in MIDIMessage::decode_stream(input) {
//         if let Some(msg) = euphony_lumina::Event::from_midi(message.clone()) {
//             // return JsValue::from_serde(&msg).unwrap();
//         }
//     }

//     JsValue::null()
// }

#[wasm_bindgen]
pub fn process(input: &[u8], output: &mut [u8], mode_idx: usize, tonic: usize) -> usize {
    let mut output = Cursor::new(output);
    let mode = MODES[mode_idx].2;

    for message in MIDIMessage::decode_stream(input) {
        match message {
            MIDIMessage::NoteOn {
                channel,
                key,
                velocity,
            } => {
                output
                    .encode(&MIDIMessage::NoteOn {
                        channel,
                        key: collapse(key, mode, tonic),
                        velocity,
                    })
                    .unwrap();
            }
            MIDIMessage::NoteOff {
                channel,
                key,
                velocity,
            } => {
                output
                    .encode(&MIDIMessage::NoteOff {
                        channel,
                        key: collapse(key, mode, tonic),
                        velocity,
                    })
                    .unwrap();
            }
            msg => {
                output.encode(&msg).unwrap();
            }
        }
    }

    output.position() as _
}

const MIDDLE_C: u8 = 60;

fn collapse(key: Key, mode: Mode, tonic: usize) -> Key {
    let absolute = Interval::new((**key, 12u8)).reduce();

    // expand the notes back based on the mode and tonic
    let mut tonic = Interval::new((tonic, 12));

    let (collapsed, shift) = match mode.len() {
        5 => {
            tonic -= Interval(1, 12); // compensate for starting on the black keys

            // just use the black keys
            let shift = Interval::new((MIDDLE_C + 1, 12)).reduce();
            let relative = absolute - shift;
            let collapsed: Interval = p::BLUES_MAJOR.collapse(relative, Default::default());
            (collapsed, shift)
        }
        _ => {
            // just use the white keys
            let shift = Interval::new((MIDDLE_C, 12)).reduce();
            let relative = absolute - shift;
            let collapsed: Interval = h::IONIAN.collapse(relative, Default::default());
            (collapsed, shift)
        }
    };
    let expanded: Interval = (mode * collapsed + tonic + shift) * 12;
    let whole = expanded.whole();

    Key::new(whole as u8).unwrap()
}
