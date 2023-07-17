use crate::Error;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Default, Debug, Clone)]
pub struct Section<'a> {
    name: Option<&'a str>,
    entries: Vec<KeyValue<'a>>,
}

impl<'a> Section<'a> {
    pub fn new(name: &'a str) -> Self {
        Section {
            name: Some(name),
            entries: Vec::new(),
        }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> std::slice::Iter<KeyValue<'a>> {
        self.entries.iter()
    }

    /// Get all entries as a sequence of merged key-values into a vector (which can contain
    /// a single value).
    pub fn entries_seq(&self) -> BTreeMap<&'a str, Vec<&'a str>> {
        self.entries.iter().fold(BTreeMap::new(), |mut map, entry| {
            map.entry(entry.key).or_default().push(entry.value);
            map
        })
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.iter().any(|e| e.key == name)
    }

    pub fn extend(&mut self, other: &Self) {
        other.entries.iter().for_each(|e| self.entries.push(*e));
    }

    pub fn get<'b: 'a>(&self, key: &'a str) -> Option<&'a str> {
        self.entries
            .iter()
            .find_map(|e| if e.key == key { Some(e.value) } else { None })
    }

    pub fn get_all<'b: 'a>(&self, key: &'a str) -> Vec<&'a str> {
        self.entries
            .iter()
            .filter_map(|e| if e.key == key { Some(e.value) } else { None })
            .collect()
    }

    pub fn push(&mut self, key: &'a str, value: &'a str) {
        self.entries.push(KeyValue { key, value });
    }
}

impl<'a> From<Section<'a>> for BTreeMap<&'a str, &'a str> {
    fn from(value: Section<'a>) -> Self {
        let mut map = BTreeMap::new();
        for entry in value.entries {
            map.insert(entry.key, entry.value);
        }
        map
    }
}

impl<'a> From<Section<'a>> for BTreeMap<&'a str, Vec<&'a str>> {
    fn from(value: Section<'a>) -> Self {
        let mut map = BTreeMap::new();
        for entry in value.entries {
            map.entry(entry.key)
                .or_insert_with(Vec::new)
                .push(entry.value);
        }
        map
    }
}

#[derive(Default, Debug, Clone)]
pub struct Ini<'a> {
    root: Section<'a>,
    sections: Vec<(&'a str, Section<'a>)>,
}

impl<'a> Ini<'a> {
    /// Returns all sections (excluding the root).
    pub fn sections(&self) -> impl Iterator<Item = (&'a str, &Section<'a>)> {
        self.sections.iter().map(|(name, section)| (*name, section))
    }

    pub fn root(&self) -> &Section<'a> {
        &self.root
    }

    /// Return an iterator containing all sections.
    pub fn iter_section<'b: 'a>(&self, name: &'b str) -> impl Iterator<Item = &Section<'a>> {
        self.sections
            .iter()
            .filter(move |(n, _)| *n == name)
            .map(|(_, s)| s)
    }

    /// Return the first section matching the name.
    pub fn section<'b: 'a>(&self, name: &'b str) -> Option<&Section<'a>> {
        self.iter_section(name).next()
    }

    /// Create a new section with keys from _all_ sections with the same name.
    pub fn extended_section<'b: 'a>(&self, name: &'b str) -> Option<Section<'a>> {
        self.iter_section(name).fold(None, |mut acc, section| {
            if let Some(acc) = &mut acc {
                acc.extend(section);
            } else {
                acc = Some(section.clone());
            }
            acc
        })
    }

    pub fn is_empty(&self) -> bool {
        self.sections.is_empty() && self.root.is_empty()
    }

    pub fn get<'b: 'a>(&self, section: &'b str, key: &'b str) -> Option<&'a str> {
        self.iter_section(section)
            .find_map(|s| s.get(key))
            .or_else(|| self.root.get(key))
    }

    pub fn contains<'b: 'a>(&self, section: &'b str, key: &'b str) -> bool {
        self.iter_section(section).any(|s| s.contains(key))
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.sections.iter().any(|(_, s)| s.contains(key)) || self.root.contains(key)
    }
}

pub fn parse(input: &str) -> Result<Ini, Error> {
    let mut root_kv = Vec::new();
    let mut sections = Vec::new();
    let mut current_section: Option<(&str, Section)> = None;

    for l in input.lines() {
        // Remove comments
        let l = l.split(';').next().unwrap_or(l).trim();
        if let Some((key, value)) = l.split_once('=') {
            if let Some(section) = current_section.as_mut() {
                section.1.push(key.trim(), value.trim());
            } else {
                root_kv.push(KeyValue {
                    key: key.trim(),
                    value: value.trim(),
                });
            }
        } else if let Some(l) = l.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            if let Some(s) = current_section.take() {
                sections.push(s);
            }
            let name = l.trim();
            current_section = Some((name, Section::new(name)));
        } else {
            if l.trim() == "" {
                continue;
            }
            Err(Error::ParseError("Invalid line: ".to_string() + l))?;
        }
    }
    if let Some(s) = current_section.take() {
        sections.push(s);
    }

    Ok(Ini {
        root: Section {
            name: None,
            entries: root_kv,
        },
        sections,
    })
}
