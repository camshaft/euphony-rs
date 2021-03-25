use codec::encode::{EncoderBuffer, LenEstimator, TypeEncoder};
use core::{mem::size_of, time::Duration};
use euphony_osc::{bundle, types::Timetag};
use euphony_runtime::time::Handle as Scheduler;
use euphony_sc::{osc, track};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Debug)]
pub struct Track {
    name: String,
    events: sled::Tree,
    synths: sled::Tree,
    scheduler: Scheduler,
    spawn_id: AtomicU32,
    event_id: AtomicU32,
}

impl Track {
    pub(crate) fn new(
        events: sled::Tree,
        synths: sled::Tree,
        scheduler: Scheduler,
        name: String,
    ) -> Self {
        Track {
            name,
            events,
            synths,
            scheduler: scheduler.clone(),
            spawn_id: AtomicU32::new(1000),
            event_id: AtomicU32::new(0),
        }
    }

    fn event_id(&self, now: Duration) -> [u8; 12] {
        let now = Timetag::new(now);
        let event_id = self.event_id.fetch_add(1, Ordering::Relaxed).to_be_bytes();
        let mut id = [0u8; 12];
        id[..8].copy_from_slice(now.as_ref());
        id[8..].copy_from_slice(&event_id);
        id
    }

    fn insert_event<T>(&self, event: T, now: Duration)
    where
        T: Copy + for<'a> TypeEncoder<&'a mut [u8]> + TypeEncoder<LenEstimator>,
    {
        let len = LenEstimator::encoding_len(event, usize::MAX).unwrap();
        let mut output = vec![0u8; len];
        output.encode(event).unwrap();
        let event_id = self.event_id(now);
        self.events.insert(event_id, output).unwrap();
    }

    pub fn dump(&self, outdir: &Path, padding: Duration) -> (String, io::Result<PathBuf>) {
        let track_name = self.name.to_string();

        let mut hash = Sha256::default();
        self.write(&mut hash, padding).unwrap();
        let hash = hash.finalize();
        let hash = base64::encode_config(&hash, base64::URL_SAFE_NO_PAD);

        let outpath = outdir.join(hash).join("cmd.osc");

        if outpath.exists() {
            return (track_name, Ok(outpath));
        }

        let outcome = io::Result::Ok(()).and_then(|_| {
            fs::create_dir_all(outpath.parent().unwrap())?;
            let file = File::create(&outpath)?;
            let mut buf = io::BufWriter::new(file);
            self.write(&mut buf, padding)?;
            Ok(outpath)
        });

        (track_name, outcome)
    }

    fn write<W: io::Write>(&self, out: &mut W, padding: Duration) -> io::Result<usize> {
        let mut len = 0;

        macro_rules! header {
            ($timetag:expr, $len:expr) => {{
                let message_len = bundle::TAG.len() + size_of::<Timetag>() + $len;
                let mut len = 0;
                len += out.write(&(message_len as u32).to_be_bytes())?;
                len += out.write(&bundle::TAG)?;
                len += out.write($timetag)?;
                len
            }};
        }

        for synth in self.synths.iter().values() {
            let synth = synth?;
            len += header!(Timetag::default().as_ref(), synth.len());
            len += out.write(&synth)?;
        }

        for event in self.events.iter() {
            let (key, value) = event?;
            len += header!(&key[..8], value.len());
            len += out.write(&value)?;
        }

        // add padding and quit
        let end = Timetag::new(self.scheduler.now() + padding);
        let msg = "/quit\0\0\0";
        len += header!(end.as_ref(), msg.len());
        len += out.write(msg.as_ref())?;

        Ok(len)
    }
}

impl track::Track for Track {
    fn name(&self) -> &str {
        &self.name
    }

    fn load(&self, synthname: &str, synthdef: &[u8]) {
        self.synths
            .fetch_and_update(synthname, |prev| {
                if prev.is_some() {
                    None
                } else {
                    let event = osc::synthdef::Receive { buffer: synthdef };
                    let len = LenEstimator::encoding_len(event, usize::MAX).unwrap();
                    let mut output = vec![0u8; len];
                    output.encode(event).unwrap();
                    Some(output)
                }
            })
            .unwrap();
    }

    fn new(
        &self,
        synthname: &str,
        action: Option<osc::group::Action>,
        target: Option<osc::node::Id>,
        values: &[Option<(osc::control::Id, osc::control::Value)>],
    ) -> osc::node::Id {
        let id = self.spawn_id.fetch_add(1, Ordering::Relaxed) as i32;
        debug_assert!(id > 0, "ran out of node ids");

        let id = osc::node::Id(id);

        let event = osc::synth::NewOptional {
            name: synthname,
            id,
            action: action.unwrap_or(osc::group::Action::Tail),
            target: target.unwrap_or(osc::node::Id(0)),
            values,
        };

        self.insert_event(event, self.scheduler.now());

        id
    }

    fn set(&self, id: osc::node::Id, controls: &[Option<(osc::control::Id, osc::control::Value)>]) {
        let event = osc::node::SetOptional { id, controls };
        self.insert_event(event, self.scheduler.now());
    }

    fn free(&self, id: osc::node::Id) {
        let event = osc::node::Free { id };
        self.insert_event(event, self.scheduler.now());
    }
}
