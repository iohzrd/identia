// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use chrono::{offset::Utc, DateTime};
use feed_rs::model::{
  Entry, Feed, Image, Link, MediaCommunity, MediaContent, MediaCredit, MediaObject, MediaText,
  MediaThumbnail, Person,
};
use feed_rs::parser;
use serde::{Deserialize, Serialize};
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebFeed {
  // custom
  pub url: String,
  // provided
  pub id: String,
  pub title: Option<String>,
  pub description: Option<String>,
  pub links: Vec<String>,
  pub entries: Vec<FilteredEntry>,
}

impl From<Feed> for WebFeed {
  fn from(f: Feed) -> Self {
    Self {
      // custom
      url: String::from(""),
      // provided
      id: f.id,
      title: f.title.clone().map(|text| text.content),
      description: f.description.map(|text| text.content),
      links: f.links.into_iter().map(|link| link.href).collect(),
      entries: f
        .entries
        .into_iter()
        .map(|entry| {
          let mut e = FilteredEntry::from(entry);
          e.display_name = f
            .title
            .clone()
            .map_or(String::from(""), |title| title.content);
          e.publisher = f
            .title
            .clone()
            .map_or(String::from(""), |title| title.content);
          e
        })
        .collect(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredEntry {
  // custom
  pub cid: String,
  pub display_name: String,
  pub publisher: String,
  pub publisher_links: Vec<String>,
  pub timestamp: i64,
  // provided
  pub authors: Vec<FilteredPerson>,
  pub categories: Vec<String>,
  pub content: String,
  pub contributors: Vec<FilteredPerson>,
  pub id: String,
  pub links: Vec<String>,
  pub media: Vec<FilteredMediaObject>,
  pub published: Option<String>,
  pub rights: Option<String>,
  pub source: Option<String>,
  pub summary: String,
  pub title: Option<String>,
  pub updated: Option<String>,
}

impl From<Entry> for FilteredEntry {
  fn from(e: Entry) -> Self {
    Self {
      // custom
      cid: e.id.clone(),
      display_name: String::from(""),
      publisher: String::from(""),
      publisher_links: vec![],
      timestamp: match e {
        ref e if e.published.is_some() => e.published.clone().unwrap().timestamp_millis(),
        ref e if e.updated.is_some() => e.updated.clone().unwrap().timestamp_millis(),
        _ => 0,
      },
      // provided
      authors: e
        .authors
        .into_iter()
        .map(|person| FilteredPerson::from(person))
        .collect(),
      categories: e
        .categories
        .into_iter()
        .map(|category| category.term)
        .collect(),
      id: e.id,
      content: e
        .content
        .map_or(String::from(""), |content| content.body.unwrap()),
      contributors: e
        .contributors
        .into_iter()
        .map(|person| FilteredPerson::from(person))
        .collect(),
      links: e.links.into_iter().map(|link| link.href).collect(),
      media: e
        .media
        .into_iter()
        .map(|media| FilteredMediaObject::from(media))
        .collect(),
      published: e.published.map(|published| published.to_string()),
      rights: e.rights.map(|rights| rights.content),
      source: e.source.map(|source| source.to_string()),
      summary: e
        .summary
        .map_or(String::from(""), |summary| summary.content),
      title: e.title.map(|text| text.content),
      updated: e.updated.map(|updated| updated.to_string()),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredImage {
  pub uri: String,
  pub title: Option<String>,
  pub link: Option<FilteredLink>,
  pub width: Option<u32>,
  pub height: Option<u32>,
  pub description: Option<String>,
}

impl From<Image> for FilteredImage {
  fn from(i: Image) -> Self {
    Self {
      uri: i.uri,
      title: i.title.map(|title| title),
      link: i.link.map(|link| FilteredLink::from(link)),
      width: i.width.map(|width| width),
      height: i.height.map(|height| height),
      description: i.description.map(|description| description),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredLink {
  pub href: String,
  pub rel: Option<String>,
  pub media_type: Option<String>,
  pub href_lang: Option<String>,
  pub title: Option<String>,
  pub length: Option<u64>,
}

impl From<Link> for FilteredLink {
  fn from(l: Link) -> Self {
    Self {
      href: l.href,
      rel: l.rel.map(|href| href),
      media_type: l.media_type.map(|media_type| media_type),
      href_lang: l.href_lang.map(|href_lang| href_lang),
      title: l.title.map(|title| title),
      length: l.length.map(|length| length),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredMediaCommunity {
  pub stars_avg: Option<f64>,
  pub stars_count: Option<u64>,
  pub stars_max: Option<u64>,
  pub stars_min: Option<u64>,
  pub stats_favorites: Option<u64>,
  pub stats_views: Option<u64>,
}

impl From<MediaCommunity> for FilteredMediaCommunity {
  fn from(m: MediaCommunity) -> Self {
    Self {
      stars_avg: m.stars_avg.map(|stars_avg| stars_avg),
      stars_count: m.stars_count.map(|stars_count| stars_count),
      stars_max: m.stars_max.map(|stars_max| stars_max),
      stars_min: m.stars_min.map(|stars_min| stars_min),
      stats_favorites: m.stats_favorites.map(|stats_favorites| stats_favorites),
      stats_views: m.stats_views.map(|stats_views| stats_views),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredMediaContent {
  pub url: Option<String>,
  pub content_type: Option<String>,
  pub height: Option<u32>,
  pub width: Option<u32>,
  pub duration: Option<u64>,
  pub size: Option<u64>,
  pub rating: Option<String>,
}

impl From<MediaContent> for FilteredMediaContent {
  fn from(m: MediaContent) -> Self {
    Self {
      url: m.url.map(|url| url.to_string()),
      content_type: m.content_type.map(|content_type| content_type.to_string()),
      height: m.height.map(|height| height),
      width: m.width.map(|width| width),
      duration: m.duration.map(|duration| duration.as_secs()),
      size: m.size.map(|size| size),
      rating: m.rating.map(|rating| rating.value),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredMediaCredit {
  pub entity: String,
}

impl From<MediaCredit> for FilteredMediaCredit {
  fn from(m: MediaCredit) -> Self {
    Self { entity: m.entity }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredMediaObject {
  pub title: Option<String>,
  pub content: Vec<FilteredMediaContent>,
  pub duration: Option<u64>,
  pub thumbnails: Vec<FilteredMediaThumbnail>,
  pub texts: Vec<FilteredMediaText>,
  pub description: Option<String>,
  pub community: Option<FilteredMediaCommunity>,
  pub credits: Vec<FilteredMediaCredit>,
}

impl From<MediaObject> for FilteredMediaObject {
  fn from(m: MediaObject) -> Self {
    Self {
      title: m.title.map(|title| title.content),
      content: m
        .content
        .into_iter()
        .map(|content| FilteredMediaContent::from(content))
        .collect(),
      duration: m.duration.map(|duration| duration.as_secs()),
      thumbnails: m
        .thumbnails
        .into_iter()
        .map(|thumbnails| FilteredMediaThumbnail::from(thumbnails))
        .collect(),
      texts: m
        .texts
        .into_iter()
        .map(|texts| FilteredMediaText::from(texts))
        .collect(),
      description: m.description.map(|description| description.content),
      community: m
        .community
        .map(|community| FilteredMediaCommunity::from(community)),
      credits: m
        .credits
        .into_iter()
        .map(|credits| FilteredMediaCredit::from(credits))
        .collect(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredMediaText {
  pub text: String,
  pub start_time: Option<u64>,
  pub end_time: Option<u64>,
}

impl From<MediaText> for FilteredMediaText {
  fn from(i: MediaText) -> Self {
    Self {
      text: i.text.content,
      start_time: i.start_time.map(|start_time| start_time.as_secs()),
      end_time: i.end_time.map(|end_time| end_time.as_secs()),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredMediaThumbnail {
  // pub image: FilteredLink,
  pub time: Option<u64>,
}

impl From<MediaThumbnail> for FilteredMediaThumbnail {
  fn from(m: MediaThumbnail) -> Self {
    Self {
      // image: m.image.map(|image| FilteredImage::from(image)),
      time: m.time.map(|time| time.as_secs()),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilteredPerson {
  pub name: String,
  pub uri: Option<String>,
  pub email: Option<String>,
}

impl From<Person> for FilteredPerson {
  fn from(p: Person) -> Self {
    Self {
      name: p.name,
      uri: p.uri.map(|uri| uri),
      email: p.email.map(|email| email),
    }
  }
}

#[tauri::command]
pub async fn fetch_webfeed(url: String) -> WebFeed {
  println!("fetch_webfeed");
  let client = ClientBuilder::new().build().unwrap();
  let response = client
    .send(
      HttpRequestBuilder::new("GET", &url)
        .unwrap()
        .response_type(ResponseType::Text),
    )
    .await
    .unwrap();
  let data: &[u8] = &response.bytes().await.unwrap().data;
  let feed = parser::parse(data).unwrap();
  println!("{:#?}", feed);

  WebFeed {
    url: url.clone(),
    id: feed.id,
    title: feed.title.clone().map(|text| text.content),
    description: feed.description.map(|text| text.content),
    links: feed
      .links
      .clone()
      .into_iter()
      .map(|link| link.href)
      .collect(),
    entries: feed
      .entries
      .into_iter()
      .map(|entry| {
        let mut e = FilteredEntry::from(entry);
        e.display_name = feed
          .title
          .clone()
          .map_or(String::from(""), |title| title.content);
        e.publisher_links = feed
          .links
          .clone()
          .into_iter()
          .map(|link| link.href)
          .collect();
        e.publisher = url.clone();
        e
      })
      .collect(),
  }

  // let mut wf = WebFeed::from(feed);
  // wf.url = url;
  // wf
}
