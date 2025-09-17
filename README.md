 <img height="200px" src="https://raw.githubusercontent.com/penguinify/leto/main/docs/images/banner.png" />

# leto
rust wrapper for discord


## goals
- [x] Non native discord features
- [X] Video + Screenshare
- [ ] RPC (rust implementation)
- [X] Window features
- [ ] OS Tray
- [ ] Notifications
- [X] Sound
- [ ] Global keybinds
- [ ] Support for ARM, Linux, and Windows (Only OSX ARM + x86 for now since I don't have a Linux or Windows machine)
- [x] Faster and lighter than Discord + OpenASAR (Currently is)
- [X] VC Support
- [X] Link support
- [X] Multiple input and output devices (only input atm)
- [ ] Support for vencord, betterdiscord, equicord, and replugged.
- [X] Support for custom js

## how performance is improved
- rust backend creates lower overhead compared to electron
- no checking for updates unnecessarily
- in my testing the webview is more efficient than electrons, but that is dependent on the hardware and webview implementation

## patching
leto uses a mix of js scripts and shelter plugins, just like Dorion

## leto vs dorion    
- dorion uses tauri, while leto uses tao, wry, global-hotkeys, and muda which are the underlying libraries used by tauri.
- dorion is focused on compatibility and performance, while leto is focused on being a drop-in replacement for discord without auto-updates and without electron.

## contributing
- to launch use `cargo run`. This runs webpack aswell, im sure that windows and linux aren't working since I haven't added their custom implemnentations yet. If you have a windows or linux machine please open a PR and add platform specific code.
- Scripts are hot trash right now, and I plan to move most of them into shelter plugins, similar to Dorion

