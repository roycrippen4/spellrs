use crate::{
    definitions::DEFINITIONS,
    types::{FileTypeDefinition, FileTypeFormat, FileTypeId},
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

type ExtensionToFileTypeIdMapSet = HashMap<String, HashSet<String>>;
type ExtensionToFileTypeIdMap = HashMap<String, Vec<String>>;

static BINARY_FORMAT_IDS: Lazy<Vec<String>> = Lazy::new(|| {
    DEFINITIONS
        .iter()
        .filter(|d| d.format == Some(FileTypeFormat::Binary))
        .map(|d| d.id.clone())
        .collect()
});

pub static BINARY_LANGUAGES: Lazy<HashSet<String>> = Lazy::new(|| {
    vec![
        "binary".into(),
        "image".into(),
        "video".into(),
        "fonts".into(),
    ]
    .into_iter()
    .chain(BINARY_FORMAT_IDS.clone())
    .collect()
});

pub static GENERATED_FILES: Lazy<HashSet<String>> = Lazy::new(|| {
    vec![
        "map".into(),
        "lock".into(),
        "pdf".into(),
        "cache_files".into(),
        "rsa".into(),
        "pem".into(),
        "trie".into(),
        "log".into(),
    ]
    .into_iter()
    .chain(BINARY_FORMAT_IDS.clone())
    .collect()
});

pub static LANGUAGE_IDS: Lazy<Vec<String>> =
    Lazy::new(|| DEFINITIONS.iter().map(|d| d.id.clone()).collect());

static MAP_EXTENSION_TO_SET_OF_LANGUAGE_IDS: Lazy<ExtensionToFileTypeIdMapSet> = Lazy::new(|| {
    let mut map: ExtensionToFileTypeIdMapSet = HashMap::new();

    for def in DEFINITIONS.iter() {
        def.extensions.iter().for_each(|v| {
            map.entry(v.into()).or_default().insert(def.id.clone());
        });

        if let Some(ref filenames) = def.filenames {
            filenames.iter().for_each(|f| {
                map.entry(f.into()).or_default().insert(def.id.clone());
            });
        }
    }

    map
});

static MAP_EXTENSION_TO_LANGUAGE_IDS: Lazy<ExtensionToFileTypeIdMap> = Lazy::new(|| {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (k, s) in MAP_EXTENSION_TO_SET_OF_LANGUAGE_IDS.iter() {
        map.insert(
            k.into(),
            s.iter().map(String::from).collect::<Vec<String>>(),
        );
    }

    map
});

struct RegExpMatchToFileTypeId {
    pub regexp: Regex,
    pub id: FileTypeId,
}

static IDS_WITH_REGEXP: Lazy<Vec<RegExpMatchToFileTypeId>> =
    Lazy::new(|| DEFINITIONS.iter().filter_map(def_to_reg_exp).collect());

/// Checks to see if a filetype is considered to be a binary file type
pub fn is_binary_ext(ext: &str) -> bool {
    is_binary_file_type(get_file_types_for_ext(ext))
}

/// Checks to see if a file type is considered to be a binary file type
pub fn is_binary_file(filename: &str) -> bool {
    let filename = basename(filename);
    is_binary_file_type(find_matching_file_types(filename))
}

/// Checks to see if a file type is considered to be a binary file type
pub fn is_binary_file_type<I>(file_type_id: I) -> bool
where
    I: IntoIterator<Item = FileTypeId>,
{
    file_type_id
        .into_iter()
        .any(|id| BINARY_LANGUAGES.contains(&id))
}

/// Check if a file extension is associated with a genereated file. Generated files are not
/// typically edited by a human.
pub fn is_generated_ext(ext: &str) -> bool {
    is_file_type_generated(get_file_types_for_ext(ext))
}

/// Check if a file is auto generated. Generated files are not typically edited by a human
pub fn is_generated_file(filename: &str) -> bool {
    is_file_type_generated(find_matching_file_types(filename))
}

/// Check if a file type is auto generated. Generated files are not typically edited by a human
pub fn is_file_type_generated<I>(file_type_id: I) -> bool
where
    I: IntoIterator<Item = FileTypeId>,
{
    file_type_id
        .into_iter()
        .any(|id| GENERATED_FILES.contains(&id))
}

fn does_set_contain_any_of<I>(set_of_ids: Lazy<HashSet<FileTypeId>>, file_type_id: I) -> bool
where
    I: IntoIterator<Item = FileTypeId>,
{
    file_type_id.into_iter().any(|id| set_of_ids.contains(&id))
}

fn _get_languages_for_ext(ext: &str) -> Option<Vec<FileTypeId>> {
    if let Some(langs) = MAP_EXTENSION_TO_LANGUAGE_IDS.get(ext) {
        return Some(langs.clone());
    }

    if let Some(langs) = MAP_EXTENSION_TO_LANGUAGE_IDS.get(&format!(".{ext}")) {
        return Some(langs.clone());
    }

    None
}

/// Tries to find a matching language for a given filetype
pub fn get_file_types_for_ext(ext: &str) -> Vec<FileTypeId> {
    let mut langs = _get_languages_for_ext(ext);

    if let Some(langs) = langs {
        return langs;
    }

    langs = _get_languages_for_ext(ext.to_lowercase().as_str());

    if let Some(langs) = langs {
        return langs;
    }

    vec![]
}

fn match_patterns_to_filename(basename: &str) -> Vec<FileTypeId> {
    IDS_WITH_REGEXP
        .iter()
        .filter_map(|re| match re.regexp.is_match(basename) {
            true => Some(re.id.clone()),
            false => None,
        })
        .collect()
}

fn _get_languages_for_basename(basename: &str) -> Option<Vec<String>> {
    if let Some(f) = MAP_EXTENSION_TO_LANGUAGE_IDS.get(basename) {
        return Some(f.clone());
    }

    let pattern_matches = match_patterns_to_filename(basename);
    if !pattern_matches.is_empty() {
        return Some(pattern_matches);
    }

    let mut pos = basename.to_string().index_of('.', None);
    while pos.is_some() {
        let ids = MAP_EXTENSION_TO_LANGUAGE_IDS.get(basename.split_at(pos.unwrap()).1);

        if let Some(ids) = ids {
            return Some(ids.clone());
        }

        pos = basename.to_string().index_of('.', Some(pos.unwrap() + 1))
    }

    None
}

/// Find the matching file types for a given filename
pub fn find_matching_file_types(filename: &str) -> Vec<FileTypeId> {
    let fname = basename(filename);
    if let Some(matches) = _get_languages_for_basename(fname) {
        return matches;
    }

    if let Some(matches) = _get_languages_for_basename(fname.to_string().to_lowercase().as_str()) {
        return matches;
    }

    vec![]
}

fn basename(filename: &str) -> &str {
    Path::new(filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(filename)
}

pub fn auto_resolve<K, V, F>(mut map: HashMap<K, V>, key: K, mut resolve: F) -> V
where
    K: std::cmp::Eq,
    K: std::hash::Hash,
    V: Clone,
    F: FnMut(&K) -> V,
{
    match map.get(&key) {
        Some(found) => found.clone(),
        None => {
            let value = resolve(&key);
            map.insert(key, value.clone());
            value
        }
    }
}

fn escape_reg_ex(s: &str) -> String {
    Regex::new(r"[|\{}()\[\]^$+*?.]/g")
        .unwrap()
        .replace_all(s, "\\$&")
        .to_string()
        .replace("-", "\\x2d")
}

fn simple_glob(s: &str) -> String {
    let s = s.to_string().replace("**", "*");
    let mut pattern = "".to_string();

    s.split("").filter(|c| c.is_empty()).for_each(|c| match c {
        "?" => pattern += ".",
        "*" => pattern += ".*",
        _ => pattern += escape_reg_ex(c).as_str(),
    });

    pattern
}

fn def_to_reg_exp(def: &FileTypeDefinition) -> Option<RegExpMatchToFileTypeId> {
    def.filenames.as_ref()?;

    // Above check makes unwrap safe enough for me
    let reg_exps: Vec<String> = def
        .filenames
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|filename| match filename.contains('*') {
            true => Some(simple_glob(filename)),
            false => None,
        })
        .collect();

    if reg_exps.is_empty() {
        return None;
    }

    Some(RegExpMatchToFileTypeId {
        regexp: Regex::new(&reg_exps.join("|")).unwrap(),
        id: def.id.clone(),
    })
}

trait IndexOf {
    fn index_of(&self, pat: char, position: Option<usize>) -> Option<usize>;
}

impl IndexOf for String {
    fn index_of(&self, pat: char, position: Option<usize>) -> Option<usize> {
        self.to_string()
            .chars()
            .enumerate()
            .position(|(i, c)| c == pat && i >= position.unwrap_or(0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index_of() {
        let str = "i am a string".to_string();
        let skip = 3;
        assert_eq!(7, str.index_of('s', None).unwrap());
        assert_eq!(2, str.index_of('a', None).unwrap());
        assert_eq!(5, str.index_of('a', Some(3)).unwrap());
    }
}
