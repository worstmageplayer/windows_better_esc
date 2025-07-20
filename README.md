# Windows Better Key
- Uses the f13 key.
- When the f13 key is pressed, it checks for other key presses.
- If other keys are pressed while the f13 key is held down, it would send Ctrl + the other key.
- If no other keys are pressed while the f13 key is held down and the f13 key is released, it would send Esc.
- Does not work with f13(Ctrl) + Shift + _. It would just send Ctrl + Shift and Ctrl + _.
- However, Shift + f13(Ctrl) + _ does work.
## Only works in Windows

### Remap Caps Lock to F13
1. Press `Win + R`, type `regedit`, and press Enter.
2. Navigate to: `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Keyboard Layout`.
3. Right-click and choose: `New > Binary Value`, name it: `Scancode Map`.
4. Set its value to: `00 00 00 00 00 00 00 00
02 00 00 00 64 00 3A 00
00 00 00 00`
