
use std::any::Any;

pub trait FIXMessageField<T : Any> {
    fn get_tag_num(&self) -> u32;
    fn get_name(&self) -> &str;
    fn get_value(&self, raw : Vec<u8>) -> Result<T, ()>;
}

pub struct StringField(pub u32, pub &'static str);

impl FIXMessageField<String> for StringField {
    fn get_tag_num(&self) -> u32 {
        return self.0;
    }
    
    fn get_name(&self) -> &str {
        return self.1;
    }
    
    fn get_value(&self, raw : Vec<u8>) -> Result<String, ()> {
        return Err(());
    }
}

pub trait FIXMessageFields {
    fn get_value<T : Any, F : FIXMessageField<T>>(&self, field : &F) -> Result<Option<&T>, ()>;
    fn get_value_at<T : Any, F : FIXMessageField<T>>(&self, field : &F, index : usize) -> Result<Option<&T>, ()>;
    
    fn get_tag_value(&self, tag_num : u32) -> Result<Option<&Any>, ()>;
    fn get_tag_value_at(&self, tag_num : u32, index : usize) -> Result<Option<&Any>, ()>;
}