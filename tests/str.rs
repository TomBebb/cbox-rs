extern crate cbox;
extern crate libc;

use cbox::CBox;

static PRINT_TEXT:&'static [u8] = b"%s\n\0";

extern {
    fn printf(fmt: *const u8, text: *mut libc::c_char);
}

extern fn steal_print(text: *mut libc::c_char) {
    unsafe {
        printf(PRINT_TEXT.as_ptr(), text);
        libc::free(text as *mut libc::c_void);
    }
}

#[test]
fn test_to_c() {
    unsafe {
        let text = CBox::from("Hello, world!");
    	steal_print(text.unwrap());
    }
}

#[test]
fn test_clone() {
    unsafe {
        let text = CBox::from("Hello, world!");
        let texts = (0..10).map(|_| text.clone());
        for text in texts {
        	steal_print(text.unwrap());
        }
    }
}
