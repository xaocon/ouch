use std::{
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use tar;
use utils::colors;

use crate::{dialogs::Confirmation, oof, utils};

use walkdir::WalkDir;

// TODO: check if this is adding empty folders correctly to the final archive
pub fn build_archive_from_paths<W>(input_filenames: &[PathBuf], writer: W) -> crate::Result<W>
where
    W: Write,
{
    let mut builder = tar::Builder::new(writer);

    for filename in input_filenames {
        let previous_location = utils::cd_into_same_dir_as(filename)?;

        // Safe unwrap, input shall be treated before
        let filename = filename.file_name().unwrap();

        for entry in WalkDir::new(&filename) {
            let entry = entry?;
            let path = entry.path();

            println!("Compressing '{}'.", utils::to_utf(path));
            if !path.is_dir() {
                let mut file = fs::File::open(path)?;
                builder.append_file(path, &mut file)?;
            }
        }
        env::set_current_dir(previous_location)?;
    }

    Ok(builder.into_inner()?)
}

pub fn unpack_archive(
    reader: Box<dyn Read>,
    output_folder: &Path,
    flags: &oof::Flags,
) -> crate::Result<Vec<PathBuf>> {
    // TODO: move this printing to the caller.
    // println!(
    //     "{}[INFO]{} attempting to decompress {:?}",
    //     colors::blue(),
    //     colors::reset(),
    //     &input_path
    // );
    let confirm = Confirmation::new("Do you want to overwrite 'FILE'?", Some("FILE"));

    let mut archive = tar::Archive::new(reader);

    let mut files_unpacked = vec![];
    for file in archive.entries()? {
        let mut file = file?;

        let file_path = output_folder.join(file.path()?);
        if file_path.exists() && !utils::permission_for_overwriting(&file_path, flags, &confirm)? {
            // The user does not want to overwrite the file
            continue;
        }

        file.unpack_in(output_folder)?;

        println!(
            "{}[INFO]{} {:?} extracted. ({})",
            colors::yellow(),
            colors::reset(),
            output_folder.join(file.path()?),
            utils::Bytes::new(file.size())
        );

        files_unpacked.push(file_path);
    }

    Ok(files_unpacked)
}
