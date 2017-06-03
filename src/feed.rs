use std::io::BufRead;
use std::str::FromStr;

use quick_xml::events::Event;
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;

use category::Category;
use error::Error;
use entry::Entry;
use fromxml::FromXml;
use generator::Generator;
use link::Link;
use person::Person;
use util::atom_text;

/// Represents an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Feed {
    /// A human-readable title for the feed.
    title: String,
    /// A universally unique and permanent URI.
    id: String,
    /// The last time the feed was modified in a significant way.
    updated: String,
    /// The authors of the feed.
    authors: Vec<Person>,
    /// The categories that the feed belongs to.
    categories: Vec<Category>,
    /// The contributors to the feed.
    contributors: Vec<Person>,
    /// The software used to generate the feed.
    generator: Option<Generator>,
    /// A small image which provides visual identification for the feed.
    icon: Option<String>,
    /// The Web pages related to the feed.
    links: Vec<Link>,
    /// A larger image which provides visual identification for the feed.
    logo: Option<String>,
    /// Information about rights held in and over the feed.
    rights: Option<String>,
    /// A human-readable description or subtitle for the feed.
    subtitle: Option<String>,
    /// The entries contained in this feed.
    entries: Vec<Entry>,
}

impl Feed {
    /// Attempt to read an Atom feed from the reader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::BufReader;
    /// use std::fs::File;
    /// use atom_syndication::Feed;
    ///
    /// let file = File::open("example.xml").unwrap();
    /// let feed = Feed::read_from(BufReader::new(file)).unwrap();
    /// ```
    pub fn read_from<B: BufRead>(reader: B) -> Result<Feed, Error> {
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true).expand_empty_elements(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    if element.name() == b"feed" {
                        return Ok(Feed::from_xml(&mut reader, element.attributes())?);
                    } else {
                        return Err(Error::InvalidStartTag);
                    }
                }
                Event::Eof => break,
                _ => {}
            }

            buf.clear();
        }

        Err(Error::Eof)
    }

    /// Return the title of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_title("Feed Title");
    /// assert_eq!(feed.title(), "Feed Title");
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Set the title of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_title("Feed Title");
    /// ```
    pub fn set_title<V>(&mut self, title: V)
        where V: Into<String>
    {
        self.title = title.into();
    }

    /// Return the unique URI of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// assert_eq!(feed.id(), "urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Set the unique URI of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn set_id<V>(&mut self, id: V)
        where V: Into<String>
    {
        self.id = id.into();
    }

    /// Return the last time that this feed was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_updated("2017-06-03T15:15:44-05:00");
    /// assert_eq!(feed.updated(), "2017-06-03T15:15:44-05:00");
    /// ```
    pub fn updated(&self) -> &str {
        self.updated.as_str()
    }

    /// Set the last time that this feed was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_updated("2017-06-03T15:15:44-05:00");
    /// ```
    pub fn set_updated<V>(&mut self, updated: V)
        where V: Into<String>
    {
        self.updated = updated.into();
    }

    /// Return the authors of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Person};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_authors(vec![Person::default()]);
    /// assert_eq!(feed.authors().len(), 1);
    /// ```
    pub fn authors(&self) -> &[Person] {
        self.authors.as_slice()
    }

    /// Set the authors of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Person};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_authors(vec![Person::default()]);
    /// ```
    pub fn set_authors<V>(&mut self, authors: V)
        where V: Into<Vec<Person>>
    {
        self.authors = authors.into();
    }

    /// Return the categories this feed belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Category};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_categories(vec![Category::default()]);
    /// assert_eq!(feed.categories().len(), 1);
    /// ```
    pub fn categories(&self) -> &[Category] {
        self.categories.as_slice()
    }

    /// Set the categories this feed belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Category};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_categories(vec![Category::default()]);
    /// ```
    pub fn set_categories<V>(&mut self, categories: V)
        where V: Into<Vec<Category>>
    {
        self.categories = categories.into();
    }

    /// Return the contributors to this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Person};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_contributors(vec![Person::default()]);
    /// assert_eq!(feed.contributors().len(), 1);
    /// ```
    pub fn contributors(&self) -> &[Person] {
        self.contributors.as_slice()
    }

    /// Set the contributors to this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Person};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_contributors(vec![Person::default()]);
    /// ```
    pub fn set_contributors<V>(&mut self, contributors: V)
        where V: Into<Vec<Person>>
    {
        self.contributors = contributors.into();
    }

    /// Return the name of the software used to generate this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Generator};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_generator(Generator::default());
    /// assert!(feed.generator().is_some());
    /// ```
    pub fn generator(&self) -> Option<&Generator> {
        self.generator.as_ref()
    }

    /// Set the name of the software used to generate this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Generator};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_generator(Generator::default());
    /// ```
    pub fn set_generator<V>(&mut self, generator: V)
        where V: Into<Option<Generator>>
    {
        self.generator = generator.into()
    }

    /// Return the icon for this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_icon("http://example.com/icon.png".to_string());
    /// assert_eq!(feed.icon(), Some("http://example.com/icon.png"));
    /// ```
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_ref().map(|s| s.as_str())
    }

    /// Set the icon for this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_icon("http://example.com/icon.png".to_string());
    /// ```
    pub fn set_icon<V>(&mut self, icon: V)
        where V: Into<Option<String>>
    {
        self.icon = icon.into()
    }

    /// Return the Web pages related to this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Link};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_links(vec![Link::default()]);
    /// assert_eq!(feed.links().len(), 1);
    /// ```
    pub fn links(&self) -> &[Link] {
        self.links.as_slice()
    }

    /// Set the Web pages related to this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Link};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_links(vec![Link::default()]);
    /// ```
    pub fn set_links<V>(&mut self, links: V)
        where V: Into<Vec<Link>>
    {
        self.links = links.into();
    }

    /// Return the logo for this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_logo("http://example.com/logo.png".to_string());
    /// assert_eq!(feed.logo(), Some("http://example.com/logo.png"));
    /// ```
    pub fn logo(&self) -> Option<&str> {
        self.logo.as_ref().map(|s| s.as_str())
    }

    /// Set the logo for this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_logo("http://example.com/logo.png".to_string());
    /// ```
    pub fn set_logo<V>(&mut self, logo: V)
        where V: Into<Option<String>>
    {
        self.logo = logo.into()
    }

    /// Return the information about the rights held in and over this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_rights("© 2017 John Doe".to_string());
    /// assert_eq!(feed.rights(), Some("© 2017 John Doe"));
    /// ```
    pub fn rights(&self) -> Option<&str> {
        self.rights.as_ref().map(|s| s.as_str())
    }

    /// Set the information about the rights held in and over this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_rights("© 2017 John Doe".to_string());
    /// ```
    pub fn set_rights<V>(&mut self, rights: V)
        where V: Into<Option<String>>
    {
        self.rights = rights.into()
    }

    /// Return the description or subtitle of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_subtitle("Feed subtitle".to_string());
    /// assert_eq!(feed.subtitle(), Some("Feed subtitle"));
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(|s| s.as_str())
    }

    /// Set the description or subtitle of this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Feed;
    ///
    /// let mut feed = Feed::default();
    /// feed.set_subtitle("Feed subtitle".to_string());
    /// ```
    pub fn set_subtitle<V>(&mut self, subtitle: V)
        where V: Into<Option<String>>
    {
        self.subtitle = subtitle.into()
    }

    /// Return the entries in this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Entry};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_entries(vec![Entry::default()]);
    /// assert_eq!(feed.entries().len(), 1);
    /// ```
    pub fn entries(&self) -> &[Entry] {
        self.entries.as_slice()
    }

    /// Set the entries in this feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Feed, Entry};
    ///
    /// let mut feed = Feed::default();
    /// feed.set_entries(vec![Entry::default()]);
    /// ```
    pub fn set_entries<V>(&mut self, entries: V)
        where V: Into<Vec<Entry>>
    {
        self.entries = entries.into();
    }
}

impl FromXml for Feed {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, _: Attributes) -> Result<Self, Error> {
        let mut feed = Feed::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"id" => feed.id = atom_text(reader)?.unwrap_or_default(),
                        b"title" => feed.title = atom_text(reader)?.unwrap_or_default(),
                        b"updated" => feed.updated = atom_text(reader)?.unwrap_or_default(),
                        b"author" => {
                            feed.authors
                                .push(Person::from_xml(reader, element.attributes())?)
                        }
                        b"category" => {
                            feed.categories
                                .push(Category::from_xml(reader, element.attributes())?)
                        }
                        b"contributor" => {
                            feed.contributors
                                .push(Person::from_xml(reader, element.attributes())?)
                        }
                        b"generator" => {
                            feed.generator = Some(Generator::from_xml(reader,
                                                                      element.attributes())?)
                        }
                        b"icon" => feed.icon = atom_text(reader)?,
                        b"link" => {
                            feed.links
                                .push(Link::from_xml(reader, element.attributes())?)
                        }
                        b"logo" => feed.logo = atom_text(reader)?,
                        b"rights" => feed.rights = atom_text(reader)?,
                        b"subtitle" => feed.subtitle = atom_text(reader)?,
                        b"entry" => {
                            feed.entries
                                .push(Entry::from_xml(reader, element.attributes())?)
                        }
                        n => reader.read_to_end(n, &mut Vec::new())?,
                    }
                }
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(feed)
    }
}

impl FromStr for Feed {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Feed::read_from(s.as_bytes())
    }
}
