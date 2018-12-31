mod enter_exit;
mod pseudo;

pub use self::enter_exit::*;

use self::pseudo::*;
use std::io::{Cursor};

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
            let mut conn = PseudoEnterExit::new(&mut enter, &mut exit)
                .for_each(&double)
                .for_each(&double);
            conn.single_run()?;
        }
        assert_eq!(output, [4, 8, 12, 16]);
    }
    Ok(())
}
