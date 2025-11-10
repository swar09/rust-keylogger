// extern crate winapi;
// use winapi::um::winuser::{
//     DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT
// };
use winapi::um::winuser::{GetClipboardData};
// use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT};
// // use winapi::shared::windef::HHOOK;
// use std::io::Error;
// use std::mem::zeroed;
// use std::ptr::null_mut;
// use std::fs::{File, OpenOptions};
// use std::io::Write;


pub mod Clip {
    pub fn cpyclp{
        // Write code here to copy the clip board contents when it is getiing updated 
        // Also to save tha tin a file 

        // HANDLE GetClipboardData(
        //     [in] UINT uFormat
        // );

        // GetClipboardData	Retrieves data from the clipboard in a specified format. 
        // The clipboard must have been opened previously.
    }

    pub fn psnclp{
        // write here code to modify the clip board 
        // so we can modify the clipoboard
    }
}