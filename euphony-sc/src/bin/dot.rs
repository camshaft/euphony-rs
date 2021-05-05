use euphony_sc::{codec::decode::DecoderBuffer, synthdef::Container};
use std::io;

fn main() -> io::Result<()> {
    let input = std::env::args().nth(1).expect("missing synthdef path");
    let output = std::env::args().nth(2);

    let input = std::fs::read(input)?;

    let (container, _) = input
        .decode::<Container>()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

    let synth = &container.defs[0];

    if let Some(output) = output {
        let file = std::fs::File::create(output)?;
        let mut file = io::BufWriter::new(file);
        synth.dot(&mut file)?;
    } else {
        synth.dot(&mut std::io::stdout())?;
    };

    Ok(())
}
