// implement mouse api as mod
#[path = "types.rs"]
mod types;
pub use types::*;

use std::mem;
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LONG;
use winapi::um::winuser::GetSystemMetrics;
use winapi::um::winuser::{
    GetCursorPos, INPUT_u, SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEINPUT, WHEEL_DELTA,
};

pub fn move_mouse(x: i32, y: i32) {
    let mut input_u: INPUT_u = unsafe { mem::zeroed() };
    unsafe {
        *input_u.mi_mut() = MOUSEINPUT {
            dx: x,
            dy: y,
            mouseData: 0,
            dwFlags: MOUSEEVENTF_MOVE,
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

pub fn click_mouse(button: MouseButton) {
    let flag_for_down = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
        MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
    };

    // get current cursor position
    let mut point = winapi::shared::windef::POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point);
    }

    let mut input_u: INPUT_u = unsafe { mem::zeroed() };
    unsafe {
        *input_u.mi_mut() = MOUSEINPUT {
            dx: point.x,
            dy: point.y,
            mouseData: 0,
            dwFlags: flag_for_down,
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

    let flag_for_up = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTUP,
    };

    let mut input_u_2: INPUT_u = unsafe { mem::zeroed() };
    unsafe {
        *input_u_2.mi_mut() = MOUSEINPUT {
            dx: point.x,
            dy: point.y,
            mouseData: 0,
            dwFlags: flag_for_up,
            time: 0,
            dwExtraInfo: 0,
        };
    }
    let input2 = INPUT {
        type_: INPUT_MOUSE,
        u: input_u_2,
    };
    let mut inputs2 = [input2];
    unsafe {
        SendInput(1, inputs2.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn scroll(offset: i32) {
    if offset == 0 {
        return;
    }

    let cnt = offset.abs();

    let mouse_data = if offset > 0 {
        WHEEL_DELTA as DWORD
    } else {
        -WHEEL_DELTA as DWORD
    };

    for _ in 0..cnt {
        let mut input_u: INPUT_u = unsafe { mem::zeroed() };
        unsafe {
            *input_u.mi_mut() = MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: mouse_data,
                dwFlags: MOUSEEVENTF_WHEEL,
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
}
