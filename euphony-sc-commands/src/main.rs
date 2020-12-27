pub fn main() {}

fn parse(reference: &str) -> Vec<Command> {
    let mut commands = vec![];

    let mut command = None;

    for line in reference.split('\n') {
        if let Some(address) = line
            .strip_prefix("subsection:: ")
            .filter(|l| l.starts_with('/'))
        {
            eprintln!("{:?}", address);
        }
    }

    commands
}

enum Parser<'a> {
    Init,
}

#[derive(Clone, Debug)]
pub struct Command<'a> {
    address: &'a str,
    arguments: Vec<Argument<'a>>,
}

#[derive(Clone, Copy, Debug)]
pub struct Argument<'a> {
    description: &'a str,
    ty: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    static REFERENCE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/Server-Command-Reference.schelp"
    ));

    #[test]
    fn parse_test() {
        dbg!(parse(REFERENCE));
    }
}
