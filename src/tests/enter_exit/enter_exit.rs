pub trait EnterExit<'a> {
    type InUnit;
    fn for_each(self, func: &'a Fn(&mut Self::InUnit)) -> Self;

    fn single_run(&mut self) -> Result<(), failure::Error>;
}
