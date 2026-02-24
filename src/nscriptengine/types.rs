use crate::*;

pub struct NscriptVariable{
    pub value: Box<dyn NscriptValue>,
    pub vec:Vec<NscriptVariable>
}
impl NscriptVariable{
pub fn new()->NscriptVariable{
        NscriptVariable{
            value: Box::new(NscriptString::new("".to_string())),
            vec: Vec::new(),
        }
    }
}
// trait for a value, allows you to add your own.
pub trait NscriptValue{
    fn string(&mut self)->String;
    fn int(&mut self)->u64;
    fn float(&mut self)->f64;
}
struct NscriptString{
    value:String,
}
impl NscriptString{
    pub fn new(string:String)->NscriptString{
        NscriptString{
            value:string,
        }
    }
}
impl NscriptValue for NscriptString{
    fn string(&mut self)->String{
        return self.value.to_string();
    }
    fn int(&mut self)->u64{
        return self.value.parse::<u64>().unwrap_or(0);
    }
    fn float(&mut self)->f64{
        return self.value.parse::<f64>().unwrap_or(0.0);
    }
}
