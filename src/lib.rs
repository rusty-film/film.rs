#[cfg(test)]
extern crate failure;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read, Write};
    use std::sync::Mutex;
    use std::thread;

    pub fn event(mut input: impl Read, mut output: impl Write) -> Result<(), failure::Error> {
        let mut data = vec![0; 2];
        assert_eq!(input.read(&mut data)?, 2);
        let data = data;
        assert_eq!(output.write(&data)?, 2);
        Ok(())
    }

    #[test]
    fn base_system() -> Result<(), failure::Error> {
        lazy_static! {
            static ref PSEUDO_IN: Vec<u8> = {
                let input = vec!['H' as u8, 'i' as u8];
                input
            };
            static ref PSEUDO_OUT: Mutex<Vec<u8>> = {
                let output = Mutex::new(Vec::new());
                output
            };
        }

        let handle = thread::spawn(move || {
            let input = Cursor::new(&*PSEUDO_IN);
            let mut output = PSEUDO_OUT.lock().expect("Must be locked");
            let output = Cursor::new(&mut *output);
            event(input, output)
        });
        handle
            .join()
            .map_err(|_| failure::err_msg("Thread Error"))??;

        let output = PSEUDO_OUT.lock().expect("Must be locked");
        assert_eq!(std::str::from_utf8(&*output)?, "Hi");
        Ok(())
    }

    struct PseudoEnterExit<'a, R, W>
    where
        R: Read,
        W: Write,
    {
        enter: &'a mut R,
        exit: &'a mut W,
    }

    impl<'a, W, R> PseudoEnterExit<'a, R, W>
    where
        R: Read,
        W: Write,
    {
        fn new(enter: &'a mut R, exit: &'a mut W) -> Self {
            PseudoEnterExit { enter, exit }
        }

        fn single_run(&mut self) -> Result<(), failure::Error> {
            let mut buf = [0; 1];
            while let Ok(1) = self.enter.read(&mut buf) {
                self.exit.write(&buf)?;
            }
            Ok(())
        }
    }

    #[test]
    fn connection() -> Result<(), failure::Error> {
        let hello = "Hello World";
        let enter_length = hello.len();
        let mut output = vec![0; enter_length];
        {
            let mut enter = Cursor::new(hello.as_bytes());
            let mut exit = Cursor::new(&mut output);
            let mut conn = PseudoEnterExit::new(&mut enter, &mut exit);
            conn.single_run()?;
        }
        assert_eq!(std::str::from_utf8(&output)?, hello);
        Ok(())
    }
}
