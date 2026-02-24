use crate::*;
pub struct NscriptStorage{
globals: HashMap<Box<str>,NscriptVariable>
}
impl NscriptStorage{
    pub fn new()->NscriptStorage{
        NscriptStorage { globals:HashMap::new() }
    }
}
