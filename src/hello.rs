///////////////////////////////////////////////////////////////////////////////
// NAME:            hello.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logical data types used by the application.
//
// CREATED:         09/05/2021
//
// LAST EDITED:     09/05/2021
////

pub struct Hello { called_count: u32 }

impl Hello {
    pub fn new() -> Hello {
        Hello { called_count: 0 }
    }

    pub fn increment(&mut self) {
        self.called_count += 1;
    }

    pub fn get(&self) -> u32 {
        self.called_count
    }
}

///////////////////////////////////////////////////////////////////////////////
