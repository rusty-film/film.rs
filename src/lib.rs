#[cfg(test)]
#[macro_use]
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
    fn base_system_in_all_is_unwrap() -> Result<(), failure::Error> {
        lazy_static! {
            static ref PSEUDO_IN: Vec<u8> = {
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
