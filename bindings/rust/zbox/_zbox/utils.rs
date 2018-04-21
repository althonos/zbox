#![feature(specialization)]

pub trait QuickFind {
    fn quickfind(&self, needle: u8) -> Option<usize>;
    fn quickrfind(&self, needle: u8) -> Option<usize>;
}

impl<T> QuickFind for T
where
    T: AsRef<str>,
{
    #[cfg(not(feature = "memchr"))]
    default fn quickfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().find(char::from(needle))
    }

    #[cfg(not(feature = "memchr"))]
    default fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().rfind(char::from(needle))
    }

    #[cfg(feature = "memchr")]
    default fn quickfind(&self, needle: u8) -> Option<usize> {
        memchr(needle, self.as_ref().as_bytes())
    }

    #[cfg(feature = "memchr")]
    default fn quickrfind(&self, needle: u8) -> Option<usize> {
        memrchr(needle, self.as_ref().as_bytes())
    }
}

impl<'a> QuickFind for [u8] {
    #[cfg(not(feature = "memchr"))]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        self.iter().position(|&c| c == needle)
    }

    #[cfg(not(feature = "memchr"))]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.iter().rposition(|&c| c == needle)
    }

    #[cfg(feature = "memchr")]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        memchr(needle, self)
    }

    #[cfg(feature = "memchr")]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        memrchr(needle, self)
    }
}

pub trait Tell {
    fn tell(&mut self) -> ::std::io::Result<u64>;
}

impl<S> Tell for S
where
    S: ::std::io::Seek,
{
    default fn tell(&mut self) -> ::std::io::Result<u64> {
        self.seek(::std::io::SeekFrom::Current(0))
    }
}
