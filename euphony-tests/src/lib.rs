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
