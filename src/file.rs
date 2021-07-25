use std::{fmt, path::Path};

#[derive(Clone, PartialEq, Eq, Debug)]
/// Accepted extensions for input and output
pub enum CompressionFormat {
    Gzip, // .gz
    Bzip, // .bz
    Lzma, // .lzma
    Tar,  // .tar (technically not a compression extension, but will do for now)
    Zip,  // .zip
}

use CompressionFormat::*;

pub fn extensions_from_path(path: &Path) -> Vec<CompressionFormat> {
    let (_, extensions) = separate_known_extensions_from_name(path);
    extensions
}

pub fn separate_known_extensions_from_name(mut path: &Path) -> (&Path, Vec<CompressionFormat>) {
    let mut result = vec![];

    let all = ["tar", "zip", "bz", "bz2", "gz", "xz", "lzma", "lz"];

    if path.file_name().is_some() && all.iter().any(|ext| path.file_name().unwrap() == *ext) {
        todo!("we found a extension in the path name instead, what to do with this???");
    }

    // While there is known extensions at the tail, grab them
    while let Some(extension) = path.extension() {
        let extension = match () {
            _ if extension == "tar" => Tar,
            _ if extension == "zip" => Zip,
            _ if extension == "bz" => Bzip,
            _ if extension == "gz" || extension == "bz2" => Gzip,
            _ if extension == "xz" || extension == "lzma" || extension == "lz" => Lzma,
            _ => break,
        };

        result.push(extension);

        // Update for the next iteration
        path = if let Some(stem) = path.file_stem() { Path::new(stem) } else { Path::new("") };
    }
    // Put the extensions in the correct order: left to right
    result.reverse();

    (path, result)
}

impl fmt::Display for CompressionFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Gzip => ".gz",
            Bzip => ".bz",
            Lzma => ".xz",
            Tar => ".tar",
            Zip => ".zip",
        })
    }
}

// impl CompressionFormat {
//     pub fn from(file_name: &OsStr) -> crate::Result<Self> {
//     let compression_format_from = |ext: &OsStr| match ext {
//         _ if ext == "zip" => Ok(Zip),
//         _ if ext == "tar" => Ok(Tar),
//         _ if ext == "gz" => Ok(Gzip),
//         _ if ext == "bz" || ext == "bz2" => Ok(Bzip),
//         _ if ext == "xz" || ext == "lz" || ext == "lzma" => Ok(Lzma),
//         other => Err(crate::Error::UnknownExtensionError(utils::to_utf(other))),
//     };
