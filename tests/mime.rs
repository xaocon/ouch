#[macro_use]
mod utils;

use rand::{rngs::SmallRng, SeedableRng};
use tempfile::NamedTempFile;

use crate::utils::write_random_content;

#[test]
/// Makes sure that the files ouch produces are what they claim to be, checking their
/// types through MIME sniffing.
fn sanity_check_through_mime() {
    let temp_dir = tempfile::tempdir().expect("to build a temporary directory");
    let temp_dir_path = temp_dir.path();

    let test_file = &mut NamedTempFile::new_in(temp_dir_path).expect("to be able to build a temporary file");
    write_random_content(test_file, &mut SmallRng::from_entropy());

    let formats_and_expected_mimes = [
        ("tar", Some("application/x-tar")),
        ("zip", Some("application/zip")),
        ("tar.gz", Some("application/gzip")),
        ("tgz", Some("application/gzip")),
        ("tbz", Some("application/x-bzip2")),
        ("tbz2", Some("application/x-bzip2")),
        ("txz", Some("application/x-xz")),
        ("tlzma", Some("application/x-xz")),
        ("tzst", Some("application/zstd")),
        ("tar.bz", Some("application/x-bzip2")),
        ("tar.bz2", Some("application/x-bzip2")),
        ("tar.lzma", Some("application/x-xz")),
        ("tar.xz", Some("application/x-xz")),
        ("tar.zst", Some("application/zstd")),
        ("tar.bz3", None),
    ];

    for (format, expected_mime) in formats_and_expected_mimes {
        let path_to_compress = test_file.path();

        let compressed_file_path = &format!("{}.{}", path_to_compress.display(), format);
        ouch!("c", path_to_compress, compressed_file_path);

        let sniffed = infer::get_from_path(compressed_file_path).expect("Missing file to check mime from");

        let mime = sniffed.as_ref().map(|sniffed| sniffed.mime_type());
        assert_eq!(
            mime, expected_mime,
            "Expected the mime {expected_mime:?} for the format {format}, but got {mime:?} instead",
        );
    }
}
