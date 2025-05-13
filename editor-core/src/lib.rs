// SPDX-License-Identifier: Apache-2.0

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// A simple in-memory text buffer.
pub struct Buffer {
    pub lines: Vec<String>,
    pub cursor: (usize, usize),
}

impl Buffer {
    /// Load the entire file at `path` into a new Buffer.
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(Buffer {
            lines,
            cursor: (0, 0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn from_file_loads_all_lines() {
        // create temp file with two lines
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "first").unwrap();
        writeln!(tmp, "second").unwrap();

        let buf = Buffer::from_file(tmp.path()).unwrap();
        assert_eq!(buf.lines, vec!["first", "second"]);
        assert_eq!(buf.cursor, (0, 0));
    }
}
