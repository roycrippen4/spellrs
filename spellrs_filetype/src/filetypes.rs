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

// pub static LANGUAGE_IDS: Lazy<Vec<String>> =
//     Lazy::new(|| DEFINITIONS.iter().map(|d| d.id.clone()).collect());

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

#[derive(Debug)]
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
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(filename)
}

fn escape_reg_ex(s: &str) -> String {
    Regex::new(r"[|\\{}()\[\]^$+*?.]")
        .unwrap()
        .replace_all(s, "\\$0")
        .to_string()
        .replace("-", r"\x2d")
}

fn simple_glob(s: &str) -> String {
    let s = s.to_string().replace("**", "*");
    let mut pattern = "".to_string();

    s.split("").filter(|c| !c.is_empty()).for_each(|c| match c {
        "?" => pattern.push('.'),
        "*" => pattern.push_str(".*"),
        _ => pattern.push_str(escape_reg_ex(c).as_str()),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index_of() {
        let str = "i am a string".to_string();
        assert_eq!(7, str.index_of('s', None).unwrap());
        assert_eq!(2, str.index_of('a', None).unwrap());
        assert_eq!(5, str.index_of('a', Some(3)).unwrap());
    }

    #[test]
    fn validate_get_languages_for_ext() {
        let cases = vec![
            ("ts", vec!["typescript"]),
            (".tex", vec!["latex"]),
            (".jpg", vec!["image"]),
            (".jsonc", vec!["jsonc", "json"]),
            ("tex", vec!["latex"]),
            ("hs", vec!["haskell"]),
            ("PNG", vec!["image"]),
        ];

        for (ext, expected) in cases {
            let result: HashSet<_> = get_file_types_for_ext(ext).into_iter().collect();
            let expected: HashSet<String> = expected.into_iter().map(String::from).collect();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn validate_get_languages_for_basename() {
        let cases = vec![
            ("code.ts", vec!["typescript"]),
            ("base.r", vec!["r"]),
            ("base.R", vec!["r"]),
            ("doc.tex", vec!["latex"]),
            ("Dockerfile.bin", vec!["dockerfile"]),
            ("aws.Dockerfile", vec!["dockerfile"]),
            ("image.jpg", vec!["image"]),
            ("workspace.code-workspace", vec!["jsonc"]),
            (".code-workspace", vec!["jsonc"]),
            (".cspellcache", vec!["cache_files"]),
            ("Gemfile", vec!["ruby"]),
            ("path/Gemfile", vec!["ruby"]),
            ("Cargo.lock", vec!["lock", "toml"]),
            (".errors.log.2", vec!["log"]),
            ("my-cert.pem", vec!["pem"]),
            (
                "my-private-cert.private-key.pem",
                vec!["pem", "pem-private-key"],
            ),
            ("Dockerfile", vec!["dockerfile"]),
            ("Dockerfile.dev", vec!["dockerfile"]),
            ("docker.aws.compose.yaml", vec!["dockercompose"]),
            ("composer.lock", vec!["json", "lock"]),
            ("code.jl", vec!["julia"]),
            ("code.ts.map", vec!["json", "map"]),
        ];

        for (filename, expected) in cases {
            let result: HashSet<_> = find_matching_file_types(filename).into_iter().collect();
            let expected: HashSet<String> = expected.into_iter().map(String::from).collect();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn all_extensions_start_with_a_dot() {
        for def in DEFINITIONS.iter() {
            let exts_without_dot: Vec<&String> = def
                .extensions
                .iter()
                .filter(|&ext| ext.index_of('.', Some(0)).is_none())
                .collect();
            assert!(exts_without_dot.is_empty())
        }
    }

    #[test]
    fn test_is_binary_ext() {
        let cases = vec![
            (".md", false),
            (".exe", true),
            (".obj", true),
            (".dll", true),
            (".gif", true),
            (".jpeg", true),
            (".jpg", true),
            (".txt", false),
            ("md", false),
            ("exe", true),
            ("obj", true),
            (".EXE", true),
            (".bin", true),
            ("dll", true),
            ("gif", true),
            ("txt", false),
            ("unknown", false),
        ];

        for (ext, expected) in cases {
            let result = is_binary_ext(ext);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_is_binary_file() {
        let cases = vec![
            ("README.md", false),
            ("run.exe", true),
            ("lib.obj", true),
            ("lib.dll", true),
            ("lib.o", true),
            ("image.PNG", true),
            ("image.JPG", true),
            ("image.gif", true),
            ("picture.jpeg", true),
            ("picture.jpg", true),
            ("doc.txt", false),
            ("lock", false),
            ("Cargo.lock", false),
            ("Gemfile", false),
            (".cspellcache", false),
            ("my-video.webm", true),
            ("my-logo.svg", false),
        ];

        for (ext, expected) in cases {
            let result = is_binary_file(ext);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_is_generated_ext() {
        let cases = vec![
            (".md", false),
            (".exe", true),
            (".obj", true),
            (".dll", true),
            (".gif", true),
            (".jpeg", true),
            (".jpg", true),
            (".txt", false),
            ("md", false),
            ("exe", true),
            (".EXE", true),
            (".bin", true),
            ("obj", true),
            ("dll", true),
            ("gif", true),
            ("txt", false),
            ("pdf", true),
            ("lock", true),
        ];

        for (ext, expected) in cases {
            let result = is_generated_ext(ext);
            assert!(expected == result, "{ext} -> {expected} != {result}");
        }
    }

    #[test]
    fn test_is_generated_file() {
        let cases = vec![
            ("README.md", false),
            ("run.exe", true),
            ("lib.obj", true),
            ("lib.dll", true),
            ("lib.o", true),
            ("image.gif", true),
            ("picture.jpeg", true),
            ("picture.jpg", true),
            ("Cargo.lock", true),
            ("doc.txt", false),
            ("lock", false),
            ("Gemfile", false),
            (".cspellcache", true),
        ];

        for (filename, expected) in cases {
            let result = is_generated_file(filename);
            assert!(expected == result, "{filename} -> {expected} != {result}");
        }
    }

    #[test]
    fn test_is_binary_file_type() {
        let cases = vec![
            (vec!["typescript".to_string()], false),
            (vec!["gzip".to_string()], true),
            (vec!["unknown".to_string()], false),
        ];
        for (file, expected) in cases {
            let _file = file[0].clone();
            let result = is_binary_file_type(file);
            assert!(expected == result, "{_file} -> {expected} != {result}");
        }
    }
}
