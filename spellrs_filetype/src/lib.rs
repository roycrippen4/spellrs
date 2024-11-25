mod definitions;
mod filetypes;
mod types;

pub use filetypes::{
    find_matching_file_types, get_file_types_for_ext, is_binary_ext, is_binary_file,
    is_binary_file_type, is_file_type_generated, is_generated_ext, is_generated_file,
};
pub use types::FileTypeId;
