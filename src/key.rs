use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput,
    INPUT,
    INPUT_0,
    INPUT_KEYBOARD,
    KEYBDINPUT,
    KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP,
    VK_LCONTROL,
    VK_ESCAPE,
    VIRTUAL_KEY,
};
use std::{
    sync::atomic::{AtomicBool, Ordering::SeqCst},
    mem::size_of,
};

pub const KEY: u32 = 124; // f13
static OTHER_KEY_PRESSED: AtomicBool = AtomicBool::new(false);
pub static KEY_STATE: AtomicBool = AtomicBool::new(false);
const INPUT_SIZE: i32 = size_of::<INPUT>() as i32;

pub fn key_handler(is_key_down: bool) {
    if is_key_down {
        OTHER_KEY_PRESSED.store(false, SeqCst);
    } else if OTHER_KEY_PRESSED.load(SeqCst) {
    } else {
        send_esc();
    }
}

pub fn better_esc(vk_code: u32) {
    OTHER_KEY_PRESSED.store(true, SeqCst);
    send_ctrl_combo(vk_code);
}

fn send_ctrl_combo(vk_code: u32) {
    let inputs = &[
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk_code as u16),
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk_code as u16),
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        }
    ];
    let sent = unsafe { SendInput(inputs, INPUT_SIZE) };
    if sent != inputs.len() as u32 {
        eprintln!("SendInput failed for Ctrl+{}: sent {} of {}", vk_code, sent, inputs.len());
    }
}

static ESC_INPUTS: [INPUT; 2] = [
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_ESCAPE,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            }
        },
    },
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_ESCAPE,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            }
        },
    }
];
const ESC_INPUTS_LEN: u32 = ESC_INPUTS.len() as u32;

pub fn send_esc() {
    let sent = unsafe { SendInput(&ESC_INPUTS, INPUT_SIZE) };
    if sent != ESC_INPUTS_LEN {
        eprintln!("SendInput failed for Esc: sent {sent} of {ESC_INPUTS_LEN}");
    }
}
