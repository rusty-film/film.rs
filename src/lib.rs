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

    trait EnterExit<'a> {
        type InUnit;
        fn for_each(self, func: &'a Fn(&mut Self::InUnit)) -> Self;

        fn single_run(&mut self) -> Result<(), failure::Error>;
    }

    struct PseudoEnterExit<'a, R, W>
    where
        R: Read,
        W: Write,
    {
        enter: &'a mut R,
        func: Option<&'a Fn(&mut u8)>,
        exit: &'a mut W,
    }

    impl<'a, R, W> PseudoEnterExit<'a, R, W>
    where
        R: Read,
        W: Write,
    {
        fn new(enter: &'a mut R, exit: &'a mut W) -> Self {
            PseudoEnterExit {
                enter,
                func: None,
                exit,
            }
        }
    }

    impl<'a, W, R> EnterExit<'a> for PseudoEnterExit<'a, R, W>
    where
        R: Read,
        W: Write,
    {
        type InUnit = u8;

        fn for_each(self, func: &'a Fn(&mut Self::InUnit)) -> Self {
            PseudoEnterExit {
                enter: self.enter,
                func: Some(func),
                exit: self.exit,
            }
        }

        fn single_run(&mut self) -> Result<(), failure::Error> {
            let mut buf = [0; 1];
            while let Ok(1) = self.enter.read(&mut buf) {
                if let Some(func) = self.func {
                    buf.iter_mut().for_each(func);
                }
                self.exit.write(&buf)?;
            }
            Ok(())
        }
    }

    #[test]
    fn connection() -> Result<(), failure::Error> {
        {
            let hello = [1, 2, 3, 4];
            let enter_length = hello.len();
            let mut output = vec![0; enter_length];
            {
                let mut enter = Cursor::new(&hello);
                let mut exit = Cursor::new(&mut output);

                let mut conn = PseudoEnterExit::new(&mut enter, &mut exit);
                conn.single_run()?;
            }
            assert_eq!(output, [1, 2, 3, 4]);
        }
        {
            let hello = [1, 2, 3, 4];
            let enter_length = hello.len();
            let mut output = vec![0; enter_length];
            {
                let mut enter = Cursor::new(&hello);
                let mut exit = Cursor::new(&mut output);

                fn double<'tmp>(x: &'tmp mut u8) {
                    *x *= 2u8;
                }

                let mut conn = PseudoEnterExit::new(&mut enter, &mut exit).for_each(&double);
                conn.single_run()?;
            }
            assert_eq!(output, [2, 4, 6, 8]);
        }
        Ok(())
    }
}
