
pub struct SessionId {
    begin_string : String,
    sender       : Communicator,
    target       : Communicator
}

impl SessionId {
    pub fn new(begin_string : String, sender : Communicator, target : Communicator) -> Self {
        return SessionId {
            begin_string: begin_string,
            sender: sender,
            target: target
        }
    }
}

pub struct Communicator {
    comp_id     : String,
    sub_id      : Option<String>,
    location_id : Option<String>
}

impl Communicator {
    pub fn new(comp_id : String, sub_id : Option<String>, location_id : Option<String>) -> Self {
        return Communicator {
            comp_id: comp_id,
            sub_id: sub_id,
            location_id: location_id
        }
    }
}