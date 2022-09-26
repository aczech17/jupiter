extern crate winapi;
use winapi::shared::windef::LPPOINT;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, VK_LBUTTON, VK_RBUTTON};
use winapi::um::winuser::GetKeyState;

pub(crate) struct Mouse
{

}

impl Mouse
{
    pub(crate) fn new() -> Self
    {
        Self{}
    }

    pub(crate) fn get_mouse(&self) -> (u32, u32, bool, bool)
    {
        let (x_pos, y_pos) = unsafe {
            let mut point: POINT = POINT
            {
                x: 0,
                y: 0,
            };
            let p: LPPOINT = &mut point as *mut POINT;
            GetCursorPos(p);
            let x = (*p).x as u32;
            let y = (*p).y as u32;
            (x, y)
        };

        let lmb = unsafe{GetKeyState(VK_LBUTTON) < 0};
        let rmb = unsafe{GetKeyState(VK_RBUTTON) < 0};

        (x_pos, y_pos, lmb, rmb)
    }
}