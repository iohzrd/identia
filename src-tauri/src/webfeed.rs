// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chrono::{DateTime, Utc};
use feed_rs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Feed {
  // custom
  pub publisher: String,
  // provided
  pub id: String,
  pub title: Option<String>,
  pub updated: Option<DateTime<Utc>>,
  // pub authors: Vec<Person>,
  pub description: Option<String>,
  pub links: Vec<String>,
  // pub categories: Vec<String>,
  // pub contributors: Vec<Person>,
  // pub generator: Option<Generator>,
  // pub icon: Option<Image>,
  // pub language: Option<String>,
  pub logo: Option<Image>,
  pub published: Option<DateTime<Utc>>,
  // pub rating: Option<MediaRating>,
  // pub rights: Option<String>,
  // pub ttl: Option<u32>,
  pub entries: Vec<Entry>,
}

impl From<feed_rs::model::Feed> for Feed {
  fn from(f: feed_rs::model::Feed) -> Self {
    Self {
      // custom
      publisher: "".into(),
      // provided
      id: f.id,
      title: f.title.clone().map(|text| text.content),
      updated: f.updated.map(|updated| updated),
      // authors: f
      //   .authors
      //   .into_iter()
      //   .map(|person| Person::from(person))
      //   .collect(),
      description: f.description.map(|text| text.content),
      links: f.links.clone().into_iter().map(|link| link.href).collect(),
      // categories: f
      //   .categories
      //   .into_iter()
      //   .map(|categories| categories.term)
      //   .collect(),
      // contributors: f
      //   .contributors
      //   .into_iter()
      //   .map(|person| Person::from(person))
      //   .collect(),
      // generator: f.generator.map(|g| Generator::from(g)),
      // icon: f.icon.map(|i| Image::from(i)),
      // language: f.language.map(|l| l),
      logo: f.logo.map(|l| Image::from(l)),
      published: f.published.map(|published| published),
      // rating: f.rating.map(|r| MediaRating::from(r)),
      // rights: f.rights.map(|r| r.content),
      // ttl: f.ttl.map(|ttl| ttl),
      entries: f
        .entries
        .into_iter()
        .map(|entry| Entry::from(entry))
        .collect(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
  // custom
  pub cid: String,
  pub display_name: Option<String>,
  pub publisher: String,
  pub publisher_links: Vec<String>,
  pub timestamp: i64,
  // provided
  pub id: String,
  pub title: Option<String>,
  pub updated: Option<DateTime<Utc>>,
  // pub authors: Vec<Person>,
  pub content: Option<String>,
  pub links: Vec<String>,
  pub summary: Option<String>,
  // pub categories: Vec<String>,
  // pub contributors: Vec<Person>,
  pub published: Option<DateTime<Utc>>,
  // pub source: Option<String>,
  // pub rights: Option<String>,
  pub media: Vec<MediaObject>,
}

impl From<feed_rs::model::Entry> for Entry {
  fn from(e: feed_rs::model::Entry) -> Self {
    Self {
      // custom
      cid: e.id.clone(),
      display_name: None,
      publisher: "".into(),
      publisher_links: vec![],
      timestamp: match e {
        ref e if e.published.is_some() => e.published.clone().unwrap().timestamp_millis(),
        ref e if e.updated.is_some() => e.updated.clone().unwrap().timestamp_millis(),
        _ => DateTime::timestamp_millis(&Utc::now()),
      },
      // provided
      id: e.id,
      title: e.title.map(|text| text.content),
      updated: e.updated.map(|updated| updated),
      // authors: e
      //   .authors
      //   .into_iter()
      //   .map(|person| Person::from(person))
      //   .collect(),
      content: e.content.map(|content| content.body.unwrap()),
      links: e.links.into_iter().map(|link| link.href).collect(),
      summary: e.summary.map(|summary| summary.content),
      // categories: e
      //   .categories
      //   .into_iter()
      //   .map(|category| category.term)
      //   .collect(),
      // contributors: e
      //   .contributors
      //   .into_iter()
      //   .map(|person| Person::from(person))
      //   .collect(),
      published: e.published.map(|published| published),
      // source: e.source.map(|source| source.to_string()),
      // rights: e.rights.map(|rights| rights.content),
      media: e
        .media
        .into_iter()
        .map(|media| MediaObject::from(media))
        .collect(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Generator {
  pub content: String,
  pub uri: Option<String>,
  pub version: Option<String>,
}

impl From<feed_rs::model::Generator> for Generator {
  fn from(g: feed_rs::model::Generator) -> Self {
    Self {
      content: g.content,
      uri: g.uri.map(|uri| uri),
      version: g.version.map(|version| version),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
  pub uri: String,
  pub title: Option<String>,
  pub link: Option<Link>,
  pub width: Option<u32>,
  pub height: Option<u32>,
  pub description: Option<String>,
}

impl From<feed_rs::model::Image> for Image {
  fn from(i: feed_rs::model::Image) -> Self {
    Self {
      uri: i.uri,
      title: i.title.map(|title| title),
      link: i.link.map(|link| Link::from(link)),
      width: i.width.map(|width| width),
      height: i.height.map(|height| height),
      description: i.description.map(|description| description),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
  pub href: String,
  pub rel: Option<String>,
  pub media_type: Option<String>,
  pub href_lang: Option<String>,
  pub title: Option<String>,
  pub length: Option<u64>,
}

impl From<feed_rs::model::Link> for Link {
  fn from(l: feed_rs::model::Link) -> Self {
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
pub struct MediaCommunity {
  pub stars_avg: Option<f64>,
  pub stars_count: Option<u64>,
  pub stars_max: Option<u64>,
  pub stars_min: Option<u64>,
  pub stats_favorites: Option<u64>,
  pub stats_views: Option<u64>,
}

impl From<feed_rs::model::MediaCommunity> for MediaCommunity {
  fn from(m: feed_rs::model::MediaCommunity) -> Self {
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
pub struct MediaContent {
  pub url: Option<String>,
  pub content_type: Option<String>,
  pub height: Option<u32>,
  pub width: Option<u32>,
  pub duration: Option<u64>,
  pub size: Option<u64>,
  pub rating: Option<String>,
}

impl From<feed_rs::model::MediaContent> for MediaContent {
  fn from(m: feed_rs::model::MediaContent) -> Self {
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
pub struct MediaCredit {
  pub entity: String,
}

impl From<feed_rs::model::MediaCredit> for MediaCredit {
  fn from(m: feed_rs::model::MediaCredit) -> Self {
    Self { entity: m.entity }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaObject {
  pub title: Option<String>,
  pub content: Vec<MediaContent>,
  // pub duration: Option<u64>,
  pub thumbnails: Vec<MediaThumbnail>,
  // pub texts: Vec<MediaText>,
  pub description: Option<String>,
  // pub community: Option<MediaCommunity>,
  // pub credits: Vec<MediaCredit>,
}

impl From<feed_rs::model::MediaObject> for MediaObject {
  fn from(m: feed_rs::model::MediaObject) -> Self {
    Self {
      title: m.title.map(|title| title.content),
      content: m
        .content
        .into_iter()
        .map(|content| MediaContent::from(content))
        .collect(),
      // duration: m.duration.map(|duration| duration.as_secs()),
      thumbnails: m
        .thumbnails
        .into_iter()
        .map(|thumbnails| MediaThumbnail::from(thumbnails))
        .collect(),
      // texts: m
      //   .texts
      //   .into_iter()
      //   .map(|texts| MediaText::from(texts))
      //   .collect(),
      description: m.description.map(|description| description.content),
      // community: m.community.map(|community| MediaCommunity::from(community)),
      // credits: m
      //   .credits
      //   .into_iter()
      //   .map(|credits| MediaCredit::from(credits))
      //   .collect(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaRating {
  pub urn: String,
  pub value: String,
}

impl From<feed_rs::model::MediaRating> for MediaRating {
  fn from(m: feed_rs::model::MediaRating) -> Self {
    Self {
      urn: m.urn,
      value: m.value,
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaText {
  pub text: String,
  pub start_time: Option<u64>,
  pub end_time: Option<u64>,
}

impl From<feed_rs::model::MediaText> for MediaText {
  fn from(m: feed_rs::model::MediaText) -> Self {
    Self {
      text: m.text.content,
      start_time: m.start_time.map(|start_time| start_time.as_secs()),
      end_time: m.end_time.map(|end_time| end_time.as_secs()),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaThumbnail {
  pub image: Image,
  pub time: Option<u64>,
}

impl From<feed_rs::model::MediaThumbnail> for MediaThumbnail {
  fn from(m: feed_rs::model::MediaThumbnail) -> Self {
    Self {
      image: Image::from(m.image),
      time: m.time.map(|time| time.as_secs()),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
  pub name: String,
  pub uri: Option<String>,
  pub email: Option<String>,
}

impl From<feed_rs::model::Person> for Person {
  fn from(p: feed_rs::model::Person) -> Self {
    Self {
      name: p.name,
      uri: p.uri.map(|uri| uri),
      email: p.email.map(|email| email),
    }
  }
}

#[tauri::command]
pub async fn fetch_webfeed(url: String) -> Feed {
  println!("fetch_webfeed");
  let response = reqwest::get(&url)
    .await
    .unwrap();
  let data: &[u8] = &response.bytes().await.unwrap();
  let f = feed_rs::parser::parse(data).unwrap();
  println!("{:#?}", f);

  Feed {
    // custom
    publisher: url.clone(),
    // provided
    id: f.id,
    title: f.title.clone().map(|text| text.content),
    updated: f.updated.map(|updated| updated),
    // authors: f
    //   .authors
    //   .into_iter()
    //   .map(|person| Person::from(person))
    //   .collect(),
    description: f.description.map(|text| text.content),
    links: f.links.clone().into_iter().map(|link| link.href).collect(),
    // categories: f
    //   .categories
    //   .into_iter()
    //   .map(|categories| categories.term)
    //   .collect(),
    // contributors: f
    //   .contributors
    //   .into_iter()
    //   .map(|person| Person::from(person))
    //   .collect(),
    // generator: f.generator.map(|g| Generator::from(g)),
    // icon: f.icon.map(|i| Image::from(i)),
    // language: f.language.map(|l| l),
    logo: f.logo.map(|l| Image::from(l)),
    published: f.published.map(|published| published),
    // rating: f.rating.map(|r| MediaRating::from(r)),
    // rights: f.rights.map(|r| r.content),
    // ttl: f.ttl.map(|ttl| ttl),
    entries: f
      .entries
      .into_iter()
      .map(|entry| {
        let mut e = Entry::from(entry);
        e.display_name = f.title.clone().map(|title| title.content);
        e.publisher_links = f.links.clone().into_iter().map(|link| link.href).collect();
        e.publisher = url.clone();
        e
      })
      .collect(),
  }

  // let mut wf = Feed::from(feed);
  // wf.url = url;
  // wf
}
