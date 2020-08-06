
use std::{
    convert::{TryFrom, TryInto},
    env, fmt,
    fs::File,
    io::Write,
    time::Duration,
};

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use url::Url;

use crate::{
    cli::{Args, OutFormat},
    BASE_URL,
};

pub struct XkcdClient {
    args: Args,
}

impl XkcdClient {
    pub fn new(args: Args) -> Self {
        XkcdClient { args }
    }

    pub fn run(&self) -> Result<()> {

        // We build a request URL based on if the user requested a particular comic, if not we get the latest comic
        let url = if let Some(n) = self.args.num {
            format!("{}/{}/info.0.json", BASE_URL, n)
        } else {
            format!("{}/info.0.json", BASE_URL)
        };

        // We build an HTTP client with a custom timeout based on --timeout or the default
        let http_client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(self.args.timeout))
            .build()?;

        // We make the GET request, convert it to text (JSON), then attempt to convert to a ComicResponse
        let resp: ComicResponse = http_client.get(&url).send()?.text()?.try_into()?;

        // We convert the ComicResponse into a Comic
        let comic: Comic = resp.into();

        // If the user wants to save the image, we stub out save call
        if self.args.save {
            comic.save()?;
        }

        // Prints out the Comic representation in the format requested, or the default
        comic.print(self.args.output)?;

        // Returns no errors
        Ok(())
    }
}

#[derive(Deserialize)]
struct Comic {
    title: String,
    num: usize,
    date: String,
    desc: String,
    img_url: String
}

impl Comic {
    fn print(&self, of: OutFormat) -> Result<()> {
        match of {
            OutFormat::Text => println!("{}", self),
            OutFormat::Json => println!("{}", serde_json::to_string(self)?),
        }

        Ok(())
    }

    fn save(&self) -> Result<()> {
        use std::io::Read;

        let url = Url::parse(&*self.img_url)?;
        let img_name = url.path_segments().unwrap().last().unwrap();
        let p = env::current_dir()?;
        let p = p.join(img_name);
        let mut file = File::create(p)?;

        let body = reqwest::blocking::get(&self.img_url)?;
        file.write_all(&*body.bytes()?).map_err(|e| e.into())
    }
}

impl From<ComicResponse> for Comic {
    fn from(cr: ComicResponse) -> Self {
        Comic {
            title: cr.title,
            num: cr.num,
            date: format!("{}-{}-{}", cr.day, cr.month, cr.year)
            desc: cr.alt,
            img_url: cr.img
        }
    }
}

impl fmt::Display for Comic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "Title: {}\n\
        Comic No: {}\n\
        Date: {}\n\
        Description: {}\n\
        Image: {}\n",
        self.title, self.num, self.date, self.desc, self.img_url
    )
}
}


#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ComicResponse {
    month: String,
    num: usize,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

impl TryFrom<String> for ComicResponse {
    type Error = anyhow::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json).map_err(|e| e.into())
    }
}