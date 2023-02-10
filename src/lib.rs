
extern crate libc;

#[repr(C)]
pub struct Amazons {

}

impl Amazons{

    #[no_mangle]
    pub extern "C" fn amazons_make() -> Self {
        Self {
        }
    }

}