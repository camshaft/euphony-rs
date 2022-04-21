#![allow(dead_code)]

use crate::{manifest::Manifest, Result};
use std::{
    self,
    collections::HashSet,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Publish {
    #[structopt(long)]
    manifest_path: Option<PathBuf>,
}

impl Publish {
    pub fn run(&self) -> Result<()> {
        let mut compiler = Manifest::new(self.manifest_path.as_deref())?;

        compiler.compile()?;

        //let mut projects = create_file(compiler.root.join("target/euphony/projects.json"))?;
        //serde_json::to_writer(&mut projects, &compiler.projects)?;

        // let mut index = create_file(compiler.root.join("target/euphony/index.html"))?;
        //write_main_index(&mut index, &compiler.projects)?;

        /*
        for project in compiler.projects.keys() {
            let mut index = create_file(
                compiler
                    .root
                    .join("target/euphony")
                    .join(project)
                    .join("index.html"),
            )?;
            write_project_index(&mut index)?;
        }
        */

        Ok(())
    }
}

pub fn write_project_index<W: std::io::Write>(w: &mut W) -> Result<()> {
    macro_rules! w {
        ($arg:expr) => {
            write!(w, "{}", $arg)?
        };
    }

    w!("<!DOCTYPE html>\n");
    w!("<html>");
    w!("<head>");
    w!(r#"<meta charset="utf-8">"#);
    w!(
        r#"<link rel=stylesheet href=https://raw.githubusercontent.com/naomiaro/waveform-playlist/master/dist/waveform-playlist/css/main.css>"#
    );
    w!("<title>");
    w!("Euphony Viewer");
    w!("</title>");
    w!("</head>");
    w!("<body>");
    w!("<div id=euphony-viewer></div>");
    w!(r#"<script src="/main.js"></script>"#);
    w!(r#"<script>EuphonyViewer(document.getElementById("euphony-viewer"))</script>"#);
    w!("</body>");
    w!("</html>");

    Ok(())
}

pub fn write_main_index<W: std::io::Write>(w: &mut W, _projects: &HashSet<String>) -> Result<()> {
    macro_rules! w {
        ($arg:expr) => {
            write!(w, "{}", $arg)?
        };
    }

    w!("<!DOCTYPE html>\n");
    w!("<html>");
    w!("<head>");
    w!(r#"<meta charset="utf-8">"#);
    w!("<title>");
    w!("Euphony Projects");
    w!("</title>");
    w!("</head>");
    w!("<body>");
    // TODO list projects and links
    w!("</body>");
    w!("</html>");

    Ok(())
}

fn create_file<P: AsRef<Path>>(path: P) -> Result<std::io::BufWriter<std::fs::File>> {
    let file = std::fs::File::create(path)?;
    Ok(std::io::BufWriter::new(file))
}
