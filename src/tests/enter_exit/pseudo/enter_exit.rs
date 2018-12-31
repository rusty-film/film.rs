use crate::tests::enter_exit::EnterExit;
use std::io::{Read, Write};

pub struct PseudoEnterExit<'a, R, W>
where
    R: Read,
    W: Write,
{
    enter: &'a mut R,
    func: Vec<&'a Fn(&mut u8)>,
    exit: &'a mut W,
}

impl<'a, R, W> PseudoEnterExit<'a, R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(enter: &'a mut R, exit: &'a mut W) -> Self {
        PseudoEnterExit {
            enter,
            func: Vec::new(),
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

    fn for_each(mut self, func: &'a Fn(&mut Self::InUnit)) -> Self {
        self.func.push(func);
        PseudoEnterExit {
            enter: self.enter,
            func: self.func,
            exit: self.exit,
        }
    }

    fn single_run(&mut self) -> Result<(), failure::Error> {
        let mut buf = [0; 1];
        while let Ok(1) = self.enter.read(&mut buf) {
            for func in &self.func {
                buf.iter_mut().for_each(func);
            }
            self.exit.write(&buf)?;
        }
        Ok(())
    }
}
