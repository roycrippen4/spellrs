pub type FileTypeId = String;
pub struct FileTypeExtensionDefinition {
    pub id: FileTypeId,
    /// List of extensions starting with '.'
    pub extensions: Vec<String>,
    /// Filenames that do not have an extension or have a different type than their implied
    /// extension
    pub filenames: Option<Vec<String>>,
    /// Indicates that it is a `Text` or `Binary` file type
    pub format: Option<FileTypeFormat>,
    /// Optional Description
    pub description: Option<String>,
    /// Optional Comment
    pub comment: Option<String>,
}

impl FileTypeExtensionDefinition {
    pub fn new(
        id: String,
        extensions: Vec<String>,
        filenames: Option<Vec<String>>,
        format: Option<FileTypeFormat>,
        description: Option<String>,
        comment: Option<String>,
    ) -> Self {
        Self {
            id,
            extensions,
            filenames,
            format,
            description,
            comment,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileTypeFormat {
    Text,
    Binary,
}

pub type FileTypeDefinition = FileTypeExtensionDefinition;
pub type FileTypeDefinitions = Vec<FileTypeDefinition>;
