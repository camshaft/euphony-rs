use euphony_units::time::Beat;
use std::io;

use super::Reader;

const TICKS_PER_BEAT: u64 = 1 << 13;

const MAX_DELTA: u64 = (1 << 28) - 1;

pub fn write<R: io::Read, W: io::Write + io::Seek>(
    reader: &mut Reader<R>,
    mut out: W,
) -> io::Result<()> {
    // "MThd" 4 bytes
    //     the literal string MThd, or in hexadecimal notation: 0x4d546864. These four characters at the start of the MIDI file indicate that this is a MIDI file.
    out.write_all(b"MThd")?;
    // <header_length> 4 bytes
    //     length of the header chunk (always 6 bytes long&emdash;the size of the next three fields).
    out.write_all(&6u32.to_be_bytes())?;
    // <format> 2 bytes
    //     0 = single track file format
    //     1 = multiple track file format
    //     2 = multiple song file format
    out.write_all(&0u16.to_be_bytes())?;
    // <n> 2 bytes
    //     number of tracks that follow
    out.write_all(&1u16.to_be_bytes())?;
    // <division> 2 bytes
    //     unit of time for delta timing. If the value is positive, then it represents the units per beat. For example, +96 would mean 96 ticks per beat. If the value is negative, delta times are in SMPTE compatible units.
    out.write_all(&(TICKS_PER_BEAT as u16).to_be_bytes())?;

    // "MTrk" 4 bytes
    //     the literal string MTrk. This marks the beginning of a track.
    out.write_all(b"MTrk")?;
    // <length> 4 bytes
    //     the number of bytes in the track chunk following this number.
    let len_pos = out.stream_position()?;
    out.write_all(&0u32.to_be_bytes())?;
    // <track_event>
    //     a sequenced track event.

    let mut beats = Beat(0, 1);

    while let Some((_offset, beat, data)) = reader.try_next()? {
        let delta = seek(&mut beats, beat, &mut out)?;
        write_message(delta, data, &mut out)?;
    }

    let end_pos = out.stream_position()?;
    out.seek(io::SeekFrom::Start(len_pos))?;

    let track_len = (end_pos - len_pos) as u32;
    out.write_all(&track_len.to_be_bytes())?;

    Ok(())
}

fn seek<W: io::Write>(beats: &mut Beat, target: Beat, out: &mut W) -> io::Result<u32> {
    let mut diff = ((target.as_ratio() - beats.as_ratio()) * TICKS_PER_BEAT).whole();

    while let Some(delta) = diff.checked_sub(MAX_DELTA) {
        // write a filler event here

        write_varlen(MAX_DELTA as _, out)?;
        // FF 20 01 cc
        //
        // cc is a byte specifying the MIDI channel (0-15).
        //
        // This optional event is used to associate any subsequent SysEx and Meta events with a particular MIDI channel, and will remain in effect until the next MIDI Channel Prefix Meta event or the next MIDI event.
        out.write_all(&[0xff, 0x20, 0x01, 0x00])?;

        diff = delta;
    }

    *beats = target;
    Ok(diff as _)
}

fn write_message<W: io::Write>(delta: u32, data: [u8; 3], out: &mut W) -> io::Result<()> {
    write_varlen(delta, out)?;
    out.write_all(&data)?;
    Ok(())
}

fn write_varlen<W: io::Write>(value: u32, out: &mut W) -> io::Result<()> {
    let mut writing = false;

    for i in (0..4).rev() {
        let mut byte = ((value >> (i * 7)) & 0x7F) as u8;

        // skip leading zeros
        if !writing && byte == 0 && i != 0 {
            continue;
        }

        // mark that we've started writing bytes
        writing = true;

        // mark leading bytes
        if i != 0 {
            byte |= 0x80;
        }

        out.write_all(&[byte])?;
    }

    Ok(())
}
