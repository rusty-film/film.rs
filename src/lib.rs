#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

#[cfg(test)]
mod tests {
    use failure::{Backtrace, Context, Fail};
    use std::fmt::{self, Display};
    use std::io::{self, Cursor, Read, Write};
    use std::sync::{self, Arc, Mutex};
    use std::thread;

    #[derive(Fail, Debug)]
    pub enum ErrorKind {
        #[fail(display = "IO error")]
        Io,
        #[fail(display = "Sync error")]
        Sync,
    }

    #[derive(Debug)]
    pub struct Error {
        inner: Context<ErrorKind>,
    }

    impl Fail for Error {
        fn cause(&self) -> Option<&Fail> {
            self.inner.cause()
        }

        fn backtrace(&self) -> Option<&Backtrace> {
            self.inner.backtrace()
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            Display::fmt(&self.inner, f)
        }
    }

    impl Error {
        pub fn new(inner: Context<ErrorKind>) -> Error {
            Error { inner }
        }

        pub fn kind(&self) -> &ErrorKind {
            self.inner.get_context()
        }
    }

    impl From<ErrorKind> for Error {
        fn from(kind: ErrorKind) -> Error {
            Error {
                inner: Context::new(kind),
            }
        }
    }

    impl From<Context<ErrorKind>> for Error {
        fn from(inner: Context<ErrorKind>) -> Error {
            Error { inner }
        }
    }

    impl From<io::Error> for Error {
        fn from(error: io::Error) -> Error {
            Error {
                inner: error.context(ErrorKind::Io),
            }
        }
    }

    pub fn event(mut input: impl Read, mut output: impl Write) -> Result<(), Error> {
        let mut data = vec![0; 2];
        assert_eq!(input.read(&mut data)?, 2);
        let data = data;
        assert_eq!(output.write(&data)?, 2);
        Ok(())
    }

    #[test]
    fn base_system_in_all_is_unwrap() -> Result<(), Error> {
        lazy_static! {
            static ref PSEUDO_IN:  Vec<u8> = {
                let input = vec!['H' as u8, 'i' as u8];
                input
            };
            static ref PSEUDO_OUT: Mutex<Vec<u8>> = {
                let mut output = Mutex::new(Vec::new());
                output
            };
        }

        let handle = thread::spawn(move || {
            let input = Cursor::new(&*PSEUDO_IN);
            let mut output = PSEUDO_OUT.lock().unwrap();
            let output = Cursor::new(&mut *output);
            event(input, output)
        });
        handle.join().unwrap()?;

        let output = PSEUDO_OUT.lock().unwrap();
        assert_eq!(std::str::from_utf8(&*output).unwrap(), "Hi");
        Ok(())
    }
}
