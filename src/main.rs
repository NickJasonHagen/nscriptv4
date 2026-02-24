mod nscriptengine {
    pub mod core;
    pub mod parser;
    pub mod storage;
    pub mod types;

}
pub use nscriptengine::core::*;
pub use nscriptengine::parser::*;
pub use nscriptengine::storage::*;
pub use nscriptengine::types::*;
pub use std::collections::HashMap;
fn test_nscriptvariable() {
    let  vars = vec!(NscriptVariable::newstring("test".to_string()),NscriptVariable::newfloat(0.0),NscriptVariable::newfloat(10.123123),NscriptVariable::newint(100));
    for mut xvar in vars{
        println!("dyn: {}",xvar.value.string())
    }
}
fn main() {
    test_nscriptvariable();
}
