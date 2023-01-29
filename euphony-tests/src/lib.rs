use core::future::Future;
use std::{
    io,
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
struct List(Arc<Mutex<Vec<u8>>>);

impl io::Write for List {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
use euphony::prelude::*;

pub fn start<F>(name: &str, f: F)
where
    F: 'static + Future<Output = ()> + Send,
{
    let list = List::default();
    euphony_command::api::scope::set(Some(Box::new(list.clone())));

    euphony::runtime::Runtime::new(0).block_on(f);

    let result = core::mem::take(&mut *list.0.lock().unwrap());

    std::fs::create_dir_all("target/euphony/tmp").unwrap();
    let target = Path::new("target/euphony").join(name);
    let _ = std::fs::remove_dir_all(&target);

    let contents = target.join("contents");
    std::fs::create_dir_all(&contents).unwrap();
    let timeline = target.join("main.json");
    let mut compiler = euphony_cli::compiler::Compiler::new(contents, timeline);

    let mut result = io::Cursor::new(result);
    compiler.render(&mut result).unwrap();

    let render = euphony_cli::render::Render::default();
    render.run_compilers(vec![compiler]).unwrap();
}

#[test]
fn delay_test() {
    use western::*;

    async fn synth(interval: Interval, sustain: Beat, decay: Beat) {
        let freq = interval * MAJOR * ET12;

        let attack = Beat(1, 64);

        let oscs = osc::sine().with_frequency(freq)
            + (osc::sawtooth().with_frequency(freq * 1.01) * 0.5)
                .lowpass()
                .with_cutoff(freq.0 / 2.0);

        let env = env::linear().with_duration(attack).with_target(0.2);

        let signal = oscs * &env;

        let channels = [
            (
                1.0,
                &[
                    (Beat(1, 1) + Beat(1, 7), 0.7),
                    (Beat(1, 8) + Beat(1, 14), 0.8),
                    (Beat(1, 12), 0.6),
                ][..],
            ),
            (
                -1.0,
                &[
                    (Beat(1, 1) - Beat(1, 11), 0.8),
                    (Beat(1, 9) + Beat(1, 17), 0.7),
                ][..],
            ),
        ];

        for (azimuth, channel) in channels {
            let mut recvs = vec![];
            let mut sends = vec![];
            let mut d = |d, mul| {
                let (send, recv) = delay::feedback();
                let recv = recv * mul;
                let recv = recv.lowpass().with_cutoff(freq * (recvs.len() + 1) as f64);
                recvs.push(recv.node());
                sends.push((send, d, mul));
            };

            for (delay, mul) in channel {
                d(*delay, *mul);
            }

            let recv = recvs.mix();

            let mut echos = vec![];
            for (send, d, mul) in sends {
                let signal = (&recv * 0.8 + &signal * 0.8) * mul;
                let out = send.with_signal(signal).with_delay(d);
                echos.push(out);
            }

            let echo = echos.mix();

            async move {
                let s = echo.sink().with_radius(1.0).with_azimuth(azimuth);
                delay!(8);
                s.fin();
            }
            .spawn_primary();
        }

        let sink = signal.sink();

        delay!(sustain);

        env.set_duration(decay);

        env.set_target(0.0);

        delay!(decay);

        sink.fin();
    }

    start("delay", async {
        for int in [0, 2, 4, 7] {
            synth(Interval(int, 7), Beat(1, 16), Beat(1, 2)).await;
            delay!(1 / 4);
        }
    })
}

#[test]
fn simple_noise_test() {
    use western::*;

    async fn synth(interval: Interval, sustain: Beat, decay: Beat) {
        let freq = interval * MAJOR * ET12;

        let attack = Beat(1, 16);

        let mut oscs = vec![];

        for i in 0..4 {
            let x_osc = osc::sine().with_frequency(freq.0 + i as f64) * 0.2 + 10.0;
            let y_osc = osc::sine()
                .with_frequency(freq.0 * 1.5 + i as f64)
                .with_phase(0.5)
                * 0.2
                + 10.0;
            let z_osc = osc::sine().with_frequency(100.0).with_phase(0.5) * 1.0 + 15.0;
            let z_env = env::linear()
                .with_duration(attack * 2)
                .with_value(1.0)
                .with_target(0.0);
            let z_osc = z_osc * z_env;

            let osc = noise::simplex()
                .with_seed(46 + i)
                .with_x(x_osc)
                .with_y(y_osc)
                .with_z(z_osc);

            oscs.push(osc);
        }

        let oscs = oscs.mix();

        let oscs = oscs.moog().with_cutoff(freq.0 * 3.0);

        let env = env::linear().with_duration(attack).with_target(0.7);

        let signal = oscs * &env;

        let sink = signal.sink();

        delay!(attack + sustain);

        env.set_duration(decay);

        env.set_target(0.0);

        delay!(decay);

        sink.fin();
    }

    start("simple_noise", async {
        async {
            let beats = rand::rhythm(Beat(8, 1), [Beat(3, 1), Beat(1, 1), Beat(2, 1)]);
            let interval = beats.each(|_| *[0, 1, 2, 3, 4, 7].pick());
            let sustain = beats.each(|_| *[Beat(1, 1), Beat(2, 1), Beat(1, 1)].pick());
            for _ in 0..2 {
                let mut iter = (&beats).delays().with((&interval, &sustain).zip());
                while let Some((interval, sustain)) = iter.next().await {
                    synth(Interval(*interval, 7) - 1, *sustain, Beat(1, 2)).spawn_primary();
                }
            }
        }
        .seed(42)
        .await;
    })
}
