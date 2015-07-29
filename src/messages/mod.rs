
use fields::{FIXMessageField, FIXMessageFields};
use std::any::Any;
use std::collections::HashMap;

pub trait FIXMessage : FIXMessageFields {}

pub struct RawFIXMessage {
    field_values  : Vec<Box<Any>>,
    field_indices : HashMap<u32, Vec<usize>>
}

impl FIXMessageFields for RawFIXMessage {
    fn get_value<T : Any, F : FIXMessageField<T>>(&self, field : &F) -> Result<Option<&T>, ()> {
        return self.get_value_at(field, 0);
    }
    
    fn get_value_at<T : Any, F : FIXMessageField<T>>(&self, field : &F, index : usize) -> Result<Option<&T>, ()> {
        return self.get_tag_value_at(field.get_tag_num(), index).and_then(|option|
            match option {
                Some(value) => {
                    match value.downcast_ref::<T>() {
                        Some(value) => Ok(Some(value)),
                        None => Err(())
                    }
                },
                None => Ok(None)
            });
    }
    
    fn get_tag_value(&self, tag_num : u32) -> Result<Option<&Any>, ()> {
        return self.get_tag_value_at(tag_num, 0);
    }
    
    fn get_tag_value_at(&self, tag_num : u32, index : usize) -> Result<Option<&Any>, ()> {
        return self.field_indices.get(&tag_num).map_or(Err(()), |field_indices|
            field_indices.get(index).map_or(Err(()), |field_index|
                self.field_values.get(*field_index).map_or(Err(()), |value| Ok(Some(&**value)))));
    }
}

impl FIXMessage for RawFIXMessage {}

pub struct RawFIXMessageFields {
    field_values  : Vec<Box<Any>>,
    field_indices : HashMap<u32, Vec<usize>>
}

impl RawFIXMessageFields {
    pub fn new() -> Self {
        return RawFIXMessageFields { field_values: Vec::new(), field_indices: HashMap::new() };
    }
    
    pub fn add_value<T : Any, F : FIXMessageField<T>>(mut self, field : &F, value : T) -> Self {
        return self.add_tag_value(field.get_tag_num(), value);
    }
    
    pub fn add_tag_value<T : Any>(mut self, tag_num : u32, value : T) -> Self {
        // isolate borrows of self before owned return
        {
            let mut field_indices = self.field_indices.entry(tag_num).or_insert(Vec::new());
            
            field_indices.push(self.field_values.len());
            
            self.field_values.push(Box::new(value));
        }
        
        return self;
    }
    
    pub fn into_message(self) -> RawFIXMessage {
        return RawFIXMessage { field_values: self.field_values, field_indices: self.field_indices };
    }
}

#[cfg(test)]
mod tests {
    use super::{RawFIXMessageFields, RawFIXMessage, FIXMessage};
    use fix::fields::{FIXMessageField, StringField};
    use std;
    use std::any::Any;
    
    #[test]
    fn test_raw_fix_message() {
        let field1 : StringField = StringField(1, "Field1");
        let field2 : StringField = StringField(2, "Field2");
        let field3 : StringField = StringField(3, "Field3");
        
        let fields : RawFIXMessageFields = RawFIXMessageFields::new()
        .add_value(&field1, String::from("abc1"))
        .add_value(&field2, String::from("def2"))
        .add_value(&field3, String::from("ghi3"))
        .add_value(&field2, String::from("jkl4"))
        .add_value(&field3, String::from("mno5"))
        .add_value(&field1, String::from("pqr6"))
        .add_value(&field3, String::from("stu7"))
        .add_value(&field1, String::from("vwx8"))
        .add_value(&field2, String::from("yz90"));
        
        let message : RawFIXMessage = fields.into_message();
        
        assert_field_value(&message, &field1, String::from("abc1"));
        assert_field_value_at(&message, &field1, 1, String::from("pqr6"));
        assert_field_value_at(&message, &field1, 2, String::from("vwx8"));
        assert_field_value(&message, &field2, String::from("def2"));
        assert_field_value_at(&message, &field2, 1, String::from("jkl4"));
        assert_field_value_at(&message, &field2, 2, String::from("yz90"));
        assert_field_value(&message, &field3, String::from("ghi3"));
        assert_field_value_at(&message, &field3, 1, String::from("mno5"));
        assert_field_value_at(&message, &field3, 2, String::from("stu7"));
    }
    
    fn assert_field_value<M : FIXMessage, T : Any + std::fmt::Debug + std::cmp::Eq, F : FIXMessageField<T>>(message : &M, field : &F, expected : T) {
        assert_field_result_eq(expected, message.get_value(field));
    }
    
    fn assert_field_value_at<M : FIXMessage, T : Any + std::fmt::Debug + std::cmp::Eq, F : FIXMessageField<T>>(message : &M, field : &F, index : usize, expected : T) {
        assert_field_result_eq(expected, message.get_value_at(field, index));
    }
    
    fn assert_field_result_eq<T : Any + std::fmt::Debug + std::cmp::Eq>(expected : T, result : Result<Option<&T>, ()>) {
        assert!(result.is_ok());
        
        let option = result.unwrap();
        
        assert!(option.is_some());
        
        let actual = option.unwrap();
        
        assert_eq!(expected, *actual);
    }
}