use euphony_sc::project::Project;
use std::{io, sync::Arc};

pub type Handle = Arc<dyn Output>;

pub trait Output: 'static + Project + Send + Sync {
    fn finish(&self) -> io::Result<()>;
}
