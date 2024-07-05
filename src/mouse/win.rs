// implement mouse api as mod

use std::mem;
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LONG;
use winapi::um::winuser::GetSystemMetrics;
use winapi::um::winuser::{
    INPUT_u, SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_WHEEL, MOUSEINPUT,
};

pub fn move_mouse(x: LONG, y: LONG) {

    let mut input_u: INPUT_u = unsafe { mem::zeroed() };
    unsafe {
        *input_u.mi_mut() = MOUSEINPUT {
            dx: x,
            dy: y,
            mouseData: 0,
            dwFlags: MOUSEEVENTF_MOVE ,
            time: 0,
            dwExtraInfo: 0,
        };
    }
    let input = INPUT {
        type_: INPUT_MOUSE,
        u: input_u,
    };
    let mut inputs = [input];
    
    unsafe {
        SendInput(1, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
    }
}
