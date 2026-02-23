use crate::*;

pub struct NscriptVariable{
    pub value: Box<dyn NscriptValue>,
    pub vec:Vec<NscriptVariable>
}
impl NscriptVariable{

}
// trait for a value, allows you to add your own.
pub trait NscriptValue{
    fn string(&mut self)->String;
    fn int(&mut self)->u64;
    fn float(&mut self)->f64;
}
