
use messages::FIXMessage;
use session::SessionId;

pub trait MessageListener {
    fn get_id(&self) -> &str;
    fn on_receive(&mut self, session_id : &SessionId, message : &FIXMessage);
    fn on_send(&mut self, session_id : &SessionId, message : &FIXMessage);
}

pub struct Engine<'a> {
    message_listeners : Vec<Box<&'a mut MessageListener>>
}

impl <'a> Engine<'a> {
    pub fn new() -> Self {
        return Engine {
            message_listeners: Vec::new()
        };
    }
    
    pub fn add_message_listener<L : MessageListener + 'a>(&mut self, listener : &'a mut L) {
        self.message_listeners.push(Box::new(listener));
    }
    
    pub fn remove_message_listener<L : MessageListener + 'a>(&mut self, listener : &'a L) {
        let position = self.message_listeners.iter().position( |_listener| (**_listener).get_id() == listener.get_id());
        
        if position.is_some() {
            self.message_listeners.remove(position.unwrap());
        }
    }
}