use crate::*;

pub struct Nscript{

}
impl Nscript{
pub fn new()->Nscript{
        Nscript{}
    }
}
// each line instruction for execution.
pub trait NscriptLine{
    fn exec(&mut self) ->Option<NscriptVariable>;
}
// holds the tokenized code which the interpreter uses to exectue
pub struct NscriptExecutableBlock{
    pub codevec:Vec<Box<dyn NscriptLine>>
}

