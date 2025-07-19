use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput,
    INPUT,
    INPUT_0,
    INPUT_KEYBOARD,
    KEYBDINPUT,
    KEYEVENTF_KEYUP,
    VK_LCONTROL,
    VK_ESCAPE,
    VIRTUAL_KEY,
};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::mem::size_of;

pub const KEY: u32 = 124; // f13
static OTHER_KEY_PRESSED: AtomicBool = AtomicBool::new(false);
pub static KEY_STATE: AtomicBool = AtomicBool::new(false);

pub fn key_handler(is_key_down: bool) {
    if is_key_down {
        // println!("KEY pressed");
        OTHER_KEY_PRESSED.store(false, SeqCst);
    } else if OTHER_KEY_PRESSED.load(SeqCst) {
        // println!("KEY released, but other key was pressed");
    } else {
        // println!("KEY released, simulating Esc");
        send_esc();
    }
}

pub fn better_esc(vk_code: u32) {
    // println!("better_esc");
    OTHER_KEY_PRESSED.store(true, SeqCst);
    // println!("Simulating Ctrl + {vk_code}");
    send_ctrl_combo(vk_code);
}

fn send_ctrl_combo(vk_code: u32) {
    let inputs = &[
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    ..Default::default()
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk_code as u16),
                    ..Default::default()
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk_code as u16),
                    dwFlags: KEYEVENTF_KEYUP,
                    ..Default::default()
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    dwFlags: KEYEVENTF_KEYUP,
                    ..Default::default()
                }
            },
        },
        ];
    let sent = unsafe { SendInput(inputs, size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        eprintln!("SendInput failed for Ctrl+{}: sent {} of {}", vk_code, sent, inputs.len());
    }
}

pub fn send_esc() {
    let inputs = &[
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_ESCAPE,
                    ..Default::default()
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_ESCAPE,
                    dwFlags: KEYEVENTF_KEYUP,
                    ..Default::default()
                }
            },
        },
    ];
    let sent = unsafe { SendInput(inputs, size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        eprintln!("SendInput failed for Esc: sent {} of {}", sent, inputs.len());
    }
}
