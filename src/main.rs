use clap::Parser;
use draw::GraphRenderer;
use std::path::PathBuf;

mod draw;
mod layout;
mod parse;

#[derive(clap::Parser)]
struct Args {
    #[arg(required=true, num_args=1..)]
    files: Vec<PathBuf>,

    #[arg(short, long, value_name = "OUTPUT")]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut sources = Vec::with_capacity(args.files.len());

    for file in args.files {
        match std::fs::read_to_string(file) {
            Ok(source) => {
                sources.push(source);
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }

    let mut diagram = parse::ERDiagram::default();
    for source in &sources {
        diagram.include(source);
    }

    let mut layout = layout::Layout::new(&diagram);
    layout.perform();

    let mut document = svg::Document::new();
    document.render(diagram, layout);
    if let Err(e) = svg::save(args.output, &document) {
        eprintln!("{:?}", &e);
    }
}
