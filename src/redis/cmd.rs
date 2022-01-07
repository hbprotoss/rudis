use super::proto::Proto;

pub const CMD_COMMAND: &[u8] = b"COMMAND";

pub struct Command<'a> {
    pub req: &'a mut Proto,
    pub reply: &'a mut Proto,
}

impl<'a> Command<'a> {
    pub fn new(req: &'a mut Proto, reply: &'a mut Proto) -> Self { Self { req, reply } }

    pub fn name(&self) -> Option<&Vec<u8>> {
        if self.req.arr.len() == 0 {
            return None;
        }
        Some(&self.req.arr[0].data)
    }

    pub fn key(&self) -> Option<&Vec<u8>> {
        if self.req.arr.len() <= 1 {
            return None;
        }
        Some(&self.req.arr[1].data)
    }
}