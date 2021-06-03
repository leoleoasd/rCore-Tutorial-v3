use log::error;
use crate::batch::validate_app_address;
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            // let str = core::str::from_utf8(slice).unwrap();
            // print!("{}", str);
            // len as isize
            if !(validate_app_address(buf) && validate_app_address((buf as usize + len) as *const u8)) {
                error!("Illegal address {} in sys_write!", buf as usize);
                -1 as isize
            } else {
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            }
        },
        _ => {
            error!("Unsupported fd {} in sys_write!", fd);
            -1 as isize
            //panic!("Unsupported fd in sys_write!");
        }
    }
}