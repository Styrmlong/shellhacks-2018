#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::{self, StdinLock};
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

const URL: &'static str = "https://api.imgur.com/3/gallery/r/";
const KEY: &'static str = "Client-ID b0a99d409f4d35a";
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
    /// Where to store the memes
    #[structopt(
        parse(from_os_str),
        name = "storage",
        default_value = ".",
        short = "p",
        long = "path"
    )]
    storage: PathBuf,
    /// How should we display the meme?
    #[structopt(
        default_value = "google-chrome-stable",
        short = "d",
        long = "display"
    )]
    display: String,
    /// matcher location
    #[structopt(
        parse(from_os_str),
        name = "matcher",
        default_value = "matcher.json",
        short = "m",
        long = "match"
    )]
    matcher: PathBuf,
    /// switch to use the mac `open` program
    #[structopt(short = "c", long = "mac")]
    mac: bool,
}

#[derive(Serialize, Deserialize)]
struct Match {
    subreddit: String,
    keywords: Vec<String>,
}

impl Match {
    fn is_match(&self, s: &str) -> bool {
        for m in &self.keywords {
            if s.contains(m) {
                return true;
            }
        }
        false
    }

    fn sub(&self) -> String {
        self.subreddit.clone()
    }
}

fn main() {
    let opt = Opt::from_args();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // grab what the user wants to see
    let meme = process_meme(get_meme(&mut handle), &opt);

    println!("Ok, grabbing your meme now :D");
    // grab the image from imgur
    let url = get_image_url(&meme);
    let file = get_file(&url);

    // show it
    if opt.mac {
        Command::new("open")
            .arg("-a")
            .arg(&opt.display)
            .arg(&file)
            .spawn()
            .expect("Unable to display meme");
    } else {
        Command::new(&opt.display)
            .arg(&file)
            .spawn()
            .expect("Unable to display meme");
    }
}

fn get_voice() -> String {
    String::from_utf8(
        Command::new("python3")
            .arg("-c")
            .arg(PYTHON)
            .output()
            .expect("failed to run python")
            .stdout,
    ).expect("Not UTF8 Text!")
}

fn get_meme(_stdin: &mut StdinLock) -> String {
    //let mut buffer = String::new();
    println!("Please tell me a command :D");
    // call python process
    let voice = get_voice();
    /*
    // Get python result
    println!("You said: {}", voice);
    // loop until it works
    print!("Did I get that wrong?: (Y/n)");
    stdin
        .read_to_string(&mut buffer)
        .expect("unable to read input");

    if buffer.to_lowercase() == "n" {*/
    voice /*
    } else {
        get_meme(stdin)
    }*/
}

fn process_meme(usr: String, opt: &Opt) -> String {
    let matcher = File::open(&opt.matcher).expect("unable to find matcher file");
    let matcher: Vec<Match> =
        serde_json::from_reader(matcher).expect("unable to read matcher file");
    let usr = usr.to_lowercase();

    for m in matcher {
        if m.is_match(&usr) {
            return m.sub();
        }
    }

    opt.default_meme_src.clone()
}

fn get_image_url(sub_reddit: &str) -> String {
    use reqwest::header;
    let mut headers = header::Headers::new();
    headers.set(header::Authorization(String::from(KEY)));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Unable to build client");

    let mut res = client
        .get(&(String::from(URL) + sub_reddit))
        .send()
        .expect("Didn't get a response! ;(");

    let json: serde_json::Value = res.json().expect("Unable to make JSON");

    random_link(json)
}

fn random_link(val: serde_json::Value) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(
        0,
        val["data"].as_array().expect("data was not an array").len(),
    );

    String::from(
        val["data"][n]["link"]
            .as_str()
            .expect("unable to find link"),
    )
}

fn get_file(url: &str) -> PathBuf {
    let name = reqwest::Url::parse(url).expect("unable to URL a URL");
    let name = name
        .path_segments()
        .expect("No segments in the URL")
        .last()
        .expect("no last in a path");
    let path = PathBuf::from(&name);
    let mut file = File::create(&path).expect("unable to make file");
    let mut resp = reqwest::get(url).expect("unable to get image");
    resp.copy_to(&mut file).expect("unable to copy data");
    path
}
