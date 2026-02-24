use crate::*;

// Variable type used in nscriptcode, holds a trait NscriptValue

pub struct NscriptVariable{
    pub value: Box<dyn NscriptValue>,
    pub vec:Vec<NscriptVariable>
}
impl NscriptVariable{
    pub fn clone(&self)->NscriptVariable{
        let mut vec = Vec::new();
        for x in self.vec{
            vec.push(self.value.clone());
        }
        NscriptVariable{value:self.value.clone(),vec:self.vec}
    }
    pub fn new()->NscriptVariable{
        NscriptVariable{
            value: Box::new(NscriptString::new("".to_string())),
            vec: Vec::new(),
        }
    }
    pub fn newstring(data:String)->NscriptVariable{
        let d = NscriptString::new(data);
        NscriptVariable{value:Box::new(d),vec:Vec::new()}
    }
    pub fn newint(data:u64)->NscriptVariable{
        let d = NscriptInt::new(data);
        NscriptVariable{value:Box::new(d),vec:Vec::new()}
    }
    pub fn newfloat(data:f64)->NscriptVariable{
        let d = NscriptFloat::new(data);
        NscriptVariable{value:Box::new(d),vec:Vec::new()}
    }

}

// trait for a value, allows you to add your own.
pub trait NscriptValue{

    fn clone<T>(T:&self) ->T;
    fn string(&mut self)->String;
    fn int(&mut self)->u64;
    fn float(&mut self)->f64;
    fn object(&mut self)->Option<NscriptClass>;
}

// default NscriptValue for holding strings
#[derive(Clone)]
pub struct NscriptString{
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
    fn clone(&self) ->Self{
        NscriptString{
            value:self.value.clone()
        }
    }
    fn string(&mut self)->String{
        return self.value.to_string();
    }
    fn int(&mut self)->u64{
        return self.value.parse::<u64>().unwrap_or(0);
    }
    fn float(&mut self)->f64{
        return self.value.parse::<f64>().unwrap_or(0.0);
    }
    fn object(&mut self)->Option<NscriptClass>{
        return None;
    }
}

// default NscriptValue for holding int
#[derive(Clone)]
pub struct NscriptInt{
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
    fn clone(&self) ->Self{
        NscriptInt{
            value:self.value.clone()
        }
    }
    fn string(&mut self)->String{
        return self.value.to_string();
    }
    fn int(&mut self)->u64{
        return self.value.clone();
    }
    fn float(&mut self)->f64{
        return self.value as f64;
    }
    fn object(&mut self)->Option<NscriptClass>{
        return None;
    }
}


// default NscriptValue for holding float
//#[derive(Clone)]
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
    fn clone(&self) ->Self{
        NscriptFloat{
            value:self.value.clone()
        }
    }
    fn string(&mut self)->String{
        return self.value.to_string();
    }
    fn int(&mut self)->u64{
        return self.value as u64;
    }
    fn float(&mut self)->f64{
        return self.value.clone();
    }
    fn object(&mut self)->Option<NscriptClass>{
        return None;
    }
}
//#[derive(Clone)]
struct NscriptClass{
    value:String,
    variables:HashMap<Box<str>,NscriptVariable>,
}
impl NscriptValue for NscriptClass{
    fn clone(&self) ->Self{
        NscriptClass{
            value:self.value.clone(),
            variables:self.variables.clone()
        }
    }
    fn string(&mut self)->String{
        return self.value.to_string();
    }
    fn int(&mut self)->u64{
        return 0;
    }
    fn float(&mut self)->f64{
        return 0.0;
    }
    fn object(&mut self)->Option<NscriptClass>{
        return Some(self.clone());
    }
}
