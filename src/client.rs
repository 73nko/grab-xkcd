use std::{
    convert::{TryFrom, TryInto},
};

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

use crate::{
    cli::{Args, OutFormat},
    BASE_URL,
};
use std::time::Duration;


pub struct XkcdClient {
    args: Args,
}

impl XkcdClient {
    pub fn new(args: Args) -> Self {
        XkcdClient { args }
    }

    pub fn run(&self) -> Result<()> {
        let url = if let Some(n) = self.args.num {
            format!("{}/{}/info.0.json", BASE_URL, n)
        } else {
            format!("{}/info.0.json", BASE_URL)
        };

        let http_client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(self.args.timeout))
            .build()?;

        let resp: ComicResponse = http_client.get(&url).send()?.text()?.try_into()?;
        let comic: Comic = resp.into();

        if self.args.save {
            comic.save()?;
        }

        comic.print(self.args.output)?;

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