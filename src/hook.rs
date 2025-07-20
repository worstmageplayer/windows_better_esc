use windows::{
    Win32::{
        Foundation::{
            LRESULT,
            WPARAM,
            LPARAM,
        },
        UI::{
            WindowsAndMessaging::{
                SetWindowsHookExA,
                CallNextHookEx,
                UnhookWindowsHookEx,
                GetMessageA,
                TranslateMessage,
                DispatchMessageA,
                WH_KEYBOARD_LL,
                KBDLLHOOKSTRUCT,
                LLKHF_INJECTED,
                WM_KEYDOWN,
                WM_SYSKEYDOWN,
                WM_KEYUP,
                WM_SYSKEYUP,
                HHOOK,
                MSG,
            }
        }
    }
};
use std::{
    sync::{
        atomic::{
            AtomicPtr,
            Ordering::SeqCst,
        },
        mpsc::{
            channel,
            Sender,
        },
        OnceLock,
    },
    thread,
    ffi::c_void,
    ptr::null_mut,
};
use crate::key::{
    better_esc, key_handler, KEY, KEY_STATE
};

pub enum KeyAction {
    KeyHandler(bool),
    BetterEsc(u32),
}

static HOOK: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static SENDER: OnceLock<Sender<KeyAction>> = OnceLock::new();

pub fn init_worker() {
    let (tx, rx) = channel::<KeyAction>();
    SENDER.set(tx).expect("Sender already set");

    thread::spawn(move || {
        for action in rx {
            match action {
                KeyAction::KeyHandler(is_down) => key_handler(is_down),
                KeyAction::BetterEsc(vk) => better_esc(vk),
            }
        }
    });
}

#[unsafe(no_mangle)]
unsafe extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    }

    let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };
    if kb.flags.contains(LLKHF_INJECTED) {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
    }

    let vk_code = kb.vkCode;
    let is_key_down = match w_param.0 as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => { true }
        WM_KEYUP | WM_SYSKEYUP => { false }
        _ => return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    };

    // f13 pressed or released
    if vk_code == KEY {
        KEY_STATE.store(is_key_down, SeqCst);
        if let Some(sender) = SENDER.get() {
            let _ = sender.send(KeyAction::KeyHandler(is_key_down));
        }
        // key_handler(is_key_down);
        return LRESULT(1);
    }

    // f13 is held down && another key is pressed
    if KEY_STATE.load(SeqCst) && is_key_down {
        if let Some(sender) = SENDER.get() {
            let _ = sender.send(KeyAction::BetterEsc(vk_code));
        }
        // better_esc(vk_code);
        return LRESULT(1)
    }

    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}

#[unsafe(no_mangle)]
pub extern "system" fn start_hook() {
    {
        let hook = match unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_proc), None, 0) } {
            Ok(result) => result,
            Err(e) => {
                eprintln!("SetWindowsHookExA failed: {e}");
                return;
            },
        };
        HOOK.store(hook.0, SeqCst);
    }

    let mut msg = MSG::default();
    while unsafe { GetMessageA(&mut msg, None, 0, 0).0 } > 0 {
        unsafe {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn stop_hook() {
    let hook_ptr = HOOK.swap(std::ptr::null_mut(), SeqCst);
    if !hook_ptr.is_null() {
        unsafe { UnhookWindowsHookEx(HHOOK(hook_ptr)) }
        .unwrap_or_else(|e| eprintln!("UnhookWindowsHookEx failed: {e}"));
    }
}
