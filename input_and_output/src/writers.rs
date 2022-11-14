use serde_json;
use std::path::Path;
use std::ffi::OsStr;

/// Like readers, writers are closed automatically when they are dropped. Just as BufReader::new(reader)
/// adds a buffer to any reader, BufWriter::new(writer) adds a buffer to any writer.
/// When a BufWriter is dropped, all remaining buffered data is written to the underlying writer.
/// However, if an error occurs during this write, the error is ignored. Since this happens inside
/// BufWriter's .drop() method, there is no useful place to report the error. To make sure your
/// application notices all output errors, manually .flush() buffered writers before dropping them.
///
/// Seeking
/// File also implements the Seek trait, which means you can hop around within a File rather than
/// reading or writing in a single pass from the beginning to the end. Seek is defined like this:
///
///     pub trait Seek {
///         fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
///     }
///
///     pub enum SeekFrom {
///         Start(u64),
///         End(i64),
///         Current(i64)
///     }
///
/// Thanks to the enum, the seek method is nicely expressive: use file.seek(SeekFrom::Start(0)) to
/// rewind to the beginning and the file.seek(SeekFrom::Current(-8)) to go back a few bytes, and
/// so on.
/// Seeking within a file is slow. Whether you're using a hard disk or a solid-state drive(SSD), a
/// seek takes as long as reading several megabytes of data.
pub(crate) fn write_json() {
    use std::collections::HashMap;

    type RoomId = String;
    type RoomExits = Vec<(char, RoomId)>;
    type RoomMap = HashMap<RoomId, RoomExits>;

    let mut map = RoomMap::new();
    map.insert("Cobble Crawl".to_string(),
            vec![('W', "Debris Room".to_string())]);
    map.insert("Debris Room".to_string(),
                vec![('W', "Slopping Canyon".to_string())]);

    serde_json::to_writer(&mut std::io::stdout(), &map).unwrap();
}

/// OsStr is a string type that's a superset of UTF-8. Its job is to be able to represent all filenames,
/// command-line arguments, and environment variables on the current system, whether they're valid
/// Unicode or not. On Unix, as OsStr can hold any sequence of bytes. On Windows, an OsStr is stored
/// using an extension of UTF-8 that can encode any sequence of 16-bit values, including unmatched
/// surrogates.
/// path.components()
/// Returns an iterator over the components of the given path, from left to right. The item type of
/// this iterator is std::path::Component, an enum that can represent all the different pieces that
/// can appear in filenames:
///
///     pub enum Component<'a> {
///         Prefix(PrefixComponent<'a>),    // a drive letter or share (on Windows)
///         RootDir,                        // the root directory, `/` or `\`
///         CurDir,                         // the `.` special directory
///         ParentDir,                      // the `..` special directory
///         Normal(&'a OsStr)               // plain file and directory names
pub(crate) fn paths() {
    assert_eq!(Path::new("/home/fwolfe/program.txt").parent(), Some(Path::new("/home/fwolfe")));

    assert_eq!(Path::new("/home/fwolfe/program.txt").file_name(), Some(OsStr::new("program.txt")));

    let path1 = Path::new("/usr/share/dict");

    assert_eq!(path1.join("words"), Path::new("/usr/share/dict/words"));

    let file = Path::new("/home/jimb/calendars/calendar-18x18.pdf");
    assert_eq!(file.ancestors().collect::<Vec<_>>(),
                vec![Path::new("/home/jimb/calendars/calendar-18x18.pdf"),
                     Path::new("/home/jimb/calendars"),
                     Path::new("/home/jimb"),
                     Path::new("/home"),
                     Path::new("/")]);
}


use std::{fs, io};

/// Copy the existing directory `src` to the target path `dst`.
pub(crate) fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }

    Ok(())
}

/// Copy whatever is at `src` to the target path `dst`.
pub(crate) fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::Other,
                                  format!("don't know how to copy: {}", src.display())));
    }

    Ok(())
}


#[cfg(unix)]
use std::os::unix::fs::symlink;

/// Stub implementation of `symlink` for platforms that don't provide it.
#[cfg(not(unix))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst:Q) -> std::io::Result<()>
{
    Err(io::Error::new(io::ErrorKind::Other,
                        format!("can't copy symbolic link: {}", src.as_ref().display())))
}
