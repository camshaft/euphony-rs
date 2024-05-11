/*

use std::collections::HashMap;

use euphony::prelude::*;
use euphony_live::{
    transform::{builder, Config},
    *,
};

fn prog() -> Config {
    use builder::*;
    use transform::*;
    build(|| {
        let event = argument(0);

        let input = input("input");
        let out = output("output");
        let (replay_in, replay_out) = bus();
        let (tick_in, tick_out) = bus();

        let tick = cell(0);
        let tempo = cell(120);
        let target_mode_index = cell(0);
        // let tonic_index = cell(0);
        let interval_index = cell(0);
        let conf_table = table();
        let mode_group = constant(0);
        let tonic_group = constant(1);
        let interval_group = constant(2);

        let divisions = constant(8);
        let length = constant(64);

        let replay_table = table();

        let send_tick = || {
            let delay = constant(1).div(divisions).theory(Theory::BeatDuration {
                tempo: tempo.load(),
            });
            tick_out.send_after(constant(0), delay);
        };

        once(|| {
            conf_table.push([mode_group], constant(western::MAJOR));
            conf_table.push([mode_group], constant(western::DORIAN));

            conf_table.push([tonic_group], constant(Interval(0, 12)));
            conf_table.push([tonic_group], constant(Interval(1, 12)));

            conf_table.push([interval_group], constant(Interval(0, 7)));
            conf_table.push([interval_group], constant(Interval(4, 7)));
            conf_table.push([interval_group], constant(Interval(3, 7)));
            conf_table.push([interval_group], constant(Interval(2, 7)));

            send_tick();
            constant(1).inspect("TICK: ");
        });

        cond(tick_in.is_from(), || {
            let count = tick.load().add(constant(1)).rem(length);
            cond(count.rem(divisions).eq(constant(0)), || {
                count.div(divisions).add(constant(1)).inspect("TICK: ");
            });
            // count.inspect("TICK: ");
            tick.store(count);

            replay_table.send_all([count], replay_out);

            send_tick();
        });

        cond(replay_in.is_from().or(input.is_from()), || {
            let source_mode = constant(western::MINOR);

            let key = event.select(Select::Key);

            let channel = event.select(Select::Channel);

            let is_note = channel.eq(constant(8)).not();

            let is_note_on = event.filter(Filter::NoteOn);

            let mode_table = |idx: Local| conf_table.select_index(mode_group, idx);

            let interval_table = |idx: Local| conf_table.select_index(interval_group, idx);

            cond(is_note, || {
                // record notes
                if std::env::var("RECORD").is_ok() {
                    cond(input.is_from(), || {
                        let t = tick.load();

                        event.inspect(" REC: ");

                        replay_table.push([t], event);
                    });
                }

                let key = key.sub(constant(3));
                let key = key.theory(Theory::FromMidi);
                let key = key.theory(Theory::Collapse { mode: source_mode });
                let key = key.add(interval_table(interval_index.load()));
                let key = key.theory(Theory::Expand {
                    mode: mode_table(target_mode_index.load()),
                });
                let key = key.theory(Theory::ToMidi);
                // let key = key.add(tonic.load());

                let velocity = event.select(Select::Velocity);

                cond(is_note_on, || {
                    let event = midi(Midi::NoteOn {
                        channel,
                        key,
                        velocity,
                    });
                    event.inspect("  ON: ");
                    out.send(event);
                });

                cond(event.filter(Filter::NoteOff), || {
                    let event = midi(Midi::NoteOff {
                        channel,
                        key,
                        velocity,
                    });
                    event.inspect(" OFF: ");
                    out.send(event);
                });
            });

            cond(is_note.not(), || {
                cond(is_note_on, || {
                    let idx = target_mode_index.load();
                    let idx = idx.add(constant(1)).rem(conf_table.group_len(mode_group));
                    mode_table(idx).inspect("MODE: ");
                    target_mode_index.store(idx);

                    let v = interval_index.load();
                    let v = v.add(constant(1)).rem(conf_table.group_len(interval_group));
                    v.inspect(" INT: ");
                    interval_index.store(v);
                });
            });
        });
    })
}

#[tokio::main]
async fn main() {
    // let mut input = connection::Input::open("euphony-stage", "", false).unwrap();
    let config = prog();

    let input_overrides = HashMap::new();
    let output_overrides = HashMap::new();

    // input_overrides.insert("input".into(), "USB2.0-MIDI Port 1".into());
    // input_overrides.insert("input".into(), "euphony (bass)".into());

    let mut state = config
        .init("euphony", &input_overrides, &output_overrides)
        .unwrap();

    loop {
        state.apply_inputs().await;
    }
}
*/

fn main() {}
