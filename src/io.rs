
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    // Need BufRead trait for lines(), and Read for read() !
    use std::io::{BufRead, BufReader, Cursor, ErrorKind, Read};
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use encoding_rs::Encoding;
    use encoding_rs_io::DecodeReaderBytesBuilder;

    static DATA: LazyLock<PathBuf, fn() -> PathBuf> =
        LazyLock::new(|| std::env::current_dir().unwrap().join("data"));

    const FILE_CONTENTS: &[u8] = include_bytes!("../data/file.txt");

    #[test]
    fn read_lines() -> Result<(), io::Error> {
        do_read_lines("file.txt")
    }

    #[test]
    fn read_lines_crlf() -> Result<(), io::Error> {
        do_read_lines("file-crlf.txt")
    }

    #[test]
    fn read_lines_no_end_nl() -> Result<(), io::Error> {
        do_read_lines("file-no-end-nl.txt")
    }

    fn do_read_lines(s: &str) -> Result<(), io::Error> {
        let lines = {
            let p = DATA.join(s);
            let f = File::open(p)?;
            let result: Result<Vec<_>, _> = BufReader::new(f).lines().collect();
            result?
        };

        // Trailing CRLF or LF stripped.
        assert_eq!(lines, &["Here is the first line", "This is a second line"]);
        Ok(())
    }

    #[test]
    fn read_loop() -> Result<(), io::Error> {
        let p = DATA.join("file.txt");
        let mut f = File::open(p)?;
        let mut buf = [0; 4096];

        let mut count = 0;
        while count < buf.len() {
            match f.read(&mut buf[count..]) {
                Ok(0) => break,
                Ok(n) => count += n,
                Err(e) if e.kind() == ErrorKind::Interrupted => { /* continue */ }
                Err(e) => return Err(e),
            }
        }

        assert_eq!(&buf[..count], FILE_CONTENTS);
        Ok(())
    }

    #[test]
    fn read_all() -> Result<(), io::Error> {
        let p = DATA.join("file.txt");
        let mut f = File::open(p)?;
        let mut buf = vec![];

        let count = f.read_to_end(&mut buf)?;
        assert_eq!(&buf[..count], FILE_CONTENTS);
        Ok(())
    }

    #[test]
    fn read_bytes_exact() -> Result<(), io::Error> {
        let p = DATA.join("file.txt");
        let mut f = File::open(p)?;
        let mut buf = [0; 1024];

        // Doesn't stop just because of EOF, must fill the buffer
        assert!(f.read_exact(&mut buf).is_err());
        Ok(())
    }

    #[test]
    fn read_string_1252() -> Result<(), io::Error> {
        let p = DATA.join("file-1252.txt");
        let f = File::open(p)?;

        let encoding = Encoding::for_label_no_replacement(b"windows-1252");
        let mut d = DecodeReaderBytesBuilder::new()
            .encoding(encoding)
            .bom_sniffing(false)
            .build(f);
        let mut buf = String::new();

        let expected = "I am Windows-1252 encoded Euro \\x80: €\n";
        let count = d.read_to_string(&mut buf)?;
        assert_eq!(count, expected.len());
        assert_eq!(buf, expected);

        // Not UTF-8
        let p = DATA.join("file-1252.txt");
        let mut f = File::open(p)?;
        let mut buf = String::new();

        let count = f.read_to_string(&mut buf);
        assert_eq!(count.map_err(|e| e.kind()), Err(ErrorKind::InvalidData));
        Ok(())
    }

    #[test]
    fn read_string_utf8() -> Result<(), io::Error> {
        let p = DATA.join("file-utf8.txt");
        let mut f = File::open(p)?;
        let mut buf = String::new();

        let expected = "I am UTF-8 encoded Euro \\xE2\\x82\\xAC: €\n";
        let count = f.read_to_string(&mut buf)?;
        // Byte len
        assert_eq!(count, expected.len());
        assert_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn read_in_memory() -> Result<(), io::Error> {
        let buffer_in = "Hello world!";
        let mut c: Cursor<&[u8]> = Cursor::new(buffer_in.as_bytes());

        let mut buffer_out = String::new();
        let count = c.read_to_string(&mut buffer_out)?;
        // Byte len
        assert_eq!(count, buffer_in.len());
        assert_eq!(buffer_in, buffer_out);
        Ok(())
    }
}
