use flate2::Compression;
use flate2::write::GzEncoder;
use std::io;
use std::io::Write;
use xz2::write::XzEncoder;
use zstd;

#[derive(PartialEq)]
pub enum CompressionAlgorithm {
    Gzip,
    Lzma,
    //Zstd
}

pub trait Encode<W: Write>: Write {
    fn finish(self) -> io::Result<W>;
}

pub struct Encoder<W: Write> {
    inner: Box<W>
}

impl<W: Write> Encode<W> for Encoder<GzEncoder<W>> {
    fn finish(self) -> io::Result<W> {
        return self.inner.finish();
    }
}

impl<W: Write> Encode<W> for Encoder<XzEncoder<W>> {
    fn finish(self) -> io::Result<W> {
        return self.inner.finish();
    }
}

impl<W: Write> Write for Encoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }
    
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<W: Write + 'static> Encoder<W> {

/*    pub fn new<T: Write>(w: T, algorithm: CompressionAlgorithm) -> Encoder<W> {
        if algorithm == CompressionAlgorithm::Gzip {
            return Encoder { inner: Box::new(GzEncoder::new(w, Compression::Default)) as Box<W> };
        }
        else {
            return Encoder { inner: Box::new(XzEncoder::new(w, 6)) as Box<W> };
        }
        /*
        let encoder = match algorithm {
            CompressionAlgorithm::Gzip => Box::new(GzEncoder::new(w, Compression::Default)) as Box<W>,
            CompressionAlgorithm::Lzma => Box::new(XzEncoder::new(w, 6)) as Box<W>,
    //        CompressionAlgorithm::Zstd => Encoder { inner: zstd::stream::Encoder::new(w, 3).unwrap() }
        };

        return Encoder { inner: Box::into_raw(encoder) };
        */
    }
i*/
}
