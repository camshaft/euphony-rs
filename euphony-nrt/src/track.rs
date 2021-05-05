use crate::buffers::Buffers;
use cfg_if::cfg_if;
use codec::encode::{EncoderBuffer, LenEstimator, TypeEncoder};
use core::{mem::size_of, time::Duration};
use euphony_osc::{bundle, types::Timetag};
use euphony_runtime::time::Handle as Scheduler;
use euphony_sc::{
    buffer::{BufView, Buffer},
    osc, track,
};
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
    buffers: Buffers,
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
            scheduler,
            spawn_id: AtomicU32::new(1000),
            event_id: AtomicU32::new(0),
            buffers: Default::default(),
        }
    }

    fn event_id(&self, now: Duration) -> [u8; 12] {
        let now = Timetag::new(now);
        let event_id = self.event_id.fetch_add(1, Ordering::SeqCst).to_be_bytes();
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
        let mut output = vec![0u8; len + size_of::<u32>()];
        output[..size_of::<u32>()].copy_from_slice(&(len as u32).to_be_bytes());
        output[size_of::<u32>()..].encode(event).unwrap();
        let event_id = self.event_id(now);
        self.events.insert(event_id, output).unwrap();
    }

    pub fn dump(&self, out_dir: &Path) -> (&str, io::Result<PathBuf>) {
        let track_name = &self.name;

        let mut hash = Sha256::default();
        // make sure we don't have collisions with other tracks
        hash.update(track_name.as_bytes());
        self.write(&mut hash, out_dir).unwrap();
        let hash = hash.finalize();
        let hash = base64::encode_config(&hash, base64::URL_SAFE_NO_PAD);

        let outpath = out_dir.join("build").join(hash).join("cmd.osc");

        if outpath.exists() {
            return (track_name, Ok(outpath));
        }

        let outcome = io::Result::Ok(()).and_then(|_| {
            fs::create_dir_all(outpath.parent().unwrap())?;
            let file = File::create(&outpath)?;
            let mut buf = io::BufWriter::new(file);
            self.write(&mut buf, out_dir)?;
            Ok(outpath)
        });

        (track_name, outcome)
    }

    pub fn render(&self, build_dir: &Path, out_file: Option<&Path>) -> io::Result<PathBuf> {
        let (_, commands) = self.dump(build_dir);
        let commands = commands?;

        let output = commands.parent().unwrap().join("render.wav");

        if !output.exists() {
            crate::render::Render {
                commands: &commands,
                input: None,
                output: &output,
                channels: 2, // TODO
            }
            .render()?;
        }

        let out_file = if let Some(out_file) = out_file {
            out_file.to_owned()
        } else {
            return Ok(output);
        };

        let output = output.canonicalize()?;

        cfg_if! {
            if #[cfg(unix)] {
                let _ = std::fs::remove_file(&out_file);
                std::os::unix::fs::symlink(&output, &out_file)?;
            } else if #[cfg(windows)] {
                let _ = std::fs::remove_file(&out_file);
                std::os::windows::fs::symlink_file(&output, &out_file)?;
            } else {
                std::fs::copy(&output, out_file)?;
            }
        }

        Ok(out_file)
    }

    fn write<W: io::Write>(&self, out: &mut W, out_dir: &Path) -> io::Result<usize> {
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

        let mut tmp = vec![];
        let assets = out_dir.join("assets");
        std::fs::create_dir_all(&assets)?;
        for buffer in self.buffers.iter() {
            let msg = buffer.to_osc(&assets);

            let encoding_len = LenEstimator::encoding_len(msg, usize::MAX).unwrap();
            tmp.resize(encoding_len + size_of::<u32>(), 0u8);
            tmp[..size_of::<u32>()].copy_from_slice(&(encoding_len as u32).to_be_bytes());
            tmp[size_of::<u32>()..].encode(msg).unwrap();

            len += header!(Timetag::default().as_ref(), tmp.len());
            len += out.write(&tmp)?;
        }

        for event in self.events.iter() {
            let (key, value) = event?;
            len += header!(&key[..8], value.len());
            len += out.write(&value)?;
        }

        Ok(len)
    }
}

impl track::Track for Track {
    fn name(&self) -> &str {
        &self.name
    }

    fn load(&self, synthname: &str, synthdef: &[u8]) {
        if self.synths.contains_key(synthname).unwrap() {
            return;
        }

        self.synths
            .fetch_and_update(synthname, |prev| {
                if let Some(prev) = prev {
                    Some(prev.to_vec())
                } else {
                    let event = osc::synthdef::Receive { buffer: synthdef };
                    let len = LenEstimator::encoding_len(event, usize::MAX).unwrap();
                    let mut output = vec![0u8; len + size_of::<u32>()];
                    output[..size_of::<u32>()].copy_from_slice(&(len as u32).to_be_bytes());
                    output[size_of::<u32>()..].encode(event).unwrap();
                    Some(output)
                }
            })
            .unwrap();
    }

    fn play(
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

    fn read(&self, buffer: BufView) -> Buffer {
        self.buffers.read(buffer)
    }

    fn set(&self, id: osc::node::Id, controls: &[Option<(osc::control::Id, osc::control::Value)>]) {
        let event = osc::node::SetOptional { id, controls };
        self.insert_event(event, self.scheduler.now());
    }

    fn free(&self, id: osc::node::Id) {
        let event = osc::node::Free { id };
        self.insert_event(event, self.scheduler.now());
    }

    fn free_after(&self, id: osc::node::Id, duration: Duration) {
        let event = osc::node::Free { id };
        self.insert_event(event, self.scheduler.now() + duration);
    }
}
