use std::process::Command;
use structopt::StructOpt;

const URL: &'static str = "https://api.imgur.com/3/gallery/r/";
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
    let _opt = Opt::from_args();

    println!("Please tell me a command :D");
    // call python process
    let voice = get_voice();

    // Get python result
    println!("You said: {}", voice);
    // loop until it works
    // print!("Did I get that wrong?: ");
    println!("Ok, grabbing your meme now :D");
    // grab the image from imgur
    let url = get_image_url();
    // decide where to put it
    // call feh / jp2a
}

fn get_voice() -> String {
    String::from_utf8(
        Command::new("python")
            .arg("-c")
            .arg(PYTHON)
            .output()
            .expect("failed to run python")
            .stdout,
    ).expect("Not UTF8 Text!")
}

fn get_image_url() -> String {
    use reqwest::header;
    let mut headers = header::Headers::new();
    headers.set(header::Authorization(String::from(KEY)));

    let client = reqwest::Client::builder().default_headers(headers).build().expect("Unable to build client");

    let res = client.get(&(String::from(URL) + "aww")).send().expect("Didn't get a response! ;(");

    println!("{:?}", res);

    String::new()
}
