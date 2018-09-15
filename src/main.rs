use std::process::Command;
use structopt::StructOpt;

const KEY: &'static str = "b0a99d409f4d35a";
const PYTHON: &'static str = include_str!("test.py");

#[derive(Debug, StructOpt)]
struct Opt {
    /// The deafult source of fresh dank memes
    #[structopt(
        name = "meme-src",
        default_value = "aww",
        short = "s",
        long = "meme-src"
    )]
    default_meme_src: String,
    /// Should we display the meme using feh?
    #[structopt(short = "f", long = "feh")]
    use_feh: bool,
    /// Should we display the meme using jp2a? doesn't always work
    #[structopt(short = "j", long = "jp2a")]
    use_jp2a: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    println!("Please tell me a command :D");
    println!("{}", PYTHON);
    // call python process
    let cmd = Command::new("python")
        .arg("-c")
        .arg(include_str!("test.py"))
        .output().expect("failed to run python");

    println!("{:?}", cmd.stdout);
    // Get python result
    println!("You said: {}", "stuff");
    print!("Did I get that wrong?: ");
    // loop until it works
    println!("Ok, grabbing your meme now :D");
    // grab the image from imgur
    // decide where to put it
    // call feh / jp2a
}
