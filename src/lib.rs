#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read, Write};
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn base_system_in_all_is_unwrap() {
        let mut input = Cursor::new(vec!['H' as u8, 'i' as u8]);
        let output = Arc::new(Mutex::new(Cursor::new(Vec::new())));

        {
            let output = output.clone();

            let event = move || {
                let mut data = vec![0; 2];
                assert_eq!(input.read(&mut data).unwrap(), 2);
                let mut output = output.lock().unwrap();
                let data = data;
                assert_eq!(output.write(&data).unwrap(), 2);
            };

            let handle = thread::spawn(event);
            handle.join().unwrap();
        }

        {
            let output = output.clone();
            let output = output.lock().unwrap();
            assert_eq!(std::str::from_uft8(output.get_ref()).unwrap(), "Hi");
        }
    }
}
