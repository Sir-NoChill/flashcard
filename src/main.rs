pub use app::App;
use clap::Parser;
use flashcard::Flashcard;
use rand::seq::SliceRandom;

pub mod app;
pub mod flashcard;

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    let mut flashcards: Vec<Flashcard> = Flashcard::from_csv(args.filename);
    if args.shuffle {
        let mut rng = rand::thread_rng();
        flashcards.shuffle(&mut rng);
    }

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(flashcards).run(terminal);
    ratatui::restore();
    result
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to use for flashcards (.csv)
    filename: String,

    /// whether or not to shuffle the flashcards
    #[arg(short)]
    shuffle: bool,
}
