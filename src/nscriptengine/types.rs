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
    pub fn newstring(data:String)->NscriptVariable{
        let d = NscriptString{
            value:data,
        };
        NscriptVariable{value:Box::new(d),vec:Vec::new()}
    }
    pub fn newint(data:u64)->NscriptVariable{
        let d = NscriptInt{
            value:data,
        };
        NscriptVariable{value:Box::new(d),vec:Vec::new()}
    }
    pub fn newfloat(data:f64)->NscriptVariable{
        let d = NscriptFloat{
            value:data,
        };
        NscriptVariable{value:Box::new(d),vec:Vec::new()}
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

struct NscriptInt{
    value:u64,
}
impl NscriptInt{
    pub fn new(data:u64)->NscriptInt{
        NscriptInt{
            value:data,
        }
    }
}
impl NscriptValue for NscriptInt{
    fn string(&mut self)->String{
        return self.value.to_string();
    }
    fn int(&mut self)->u64{
        return self.value.clone();
    }
    fn float(&mut self)->f64{
        return self.value as f64;
    }
}


struct NscriptFloat{
    value:f64,
}
impl NscriptFloat{
    pub fn new(data:f64)->NscriptFloat{
        NscriptFloat{
            value:data,
        }
    }
}
impl NscriptValue for NscriptFloat{
    fn string(&mut self)->String{
        return self.value.to_string();
    }
    fn int(&mut self)->u64{
        return self.value as u64;
    }
    fn float(&mut self)->f64{
        return self.value.clone();
    }
}
