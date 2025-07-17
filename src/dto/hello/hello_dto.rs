use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetHelloDto<'a>{
    pub c_name : &'a str,
    pub c_alias : &'a str,
    pub b_enabled : i32,
}