# leto
rust wrapper for discord


# goals
- [x] Non native discord features
- [ ] RPC (rust implementation)
- [ ] OS Tray + window features
- [ ] Notifications and sound
- [ ] Global keybinds
- [ ] Support for ARM, Linux, and Windows (Only OSX ARM + x86 for now since I don't have a Linux or Windows machine)
- [x] Faster and lighter than Discord + OpenASAR (9s launch time) (this will always be ongoing but currently it starts in a few seconds)
- [ ] No Jank
- [ ] VC Support
- [X] Link support
- [ ] Multiple input and output devices
- [ ] Support for vencord, betterdiscord, equicord, and replugged.
- [X] Support for custom js
- [ ] As close to default discord as possible

# how performance is improved
- rust backend creates lower overhead compared to electron
- no checking for updates unnecessarily
- in my testing the webview is more efficient than electrons, but that is dependent on the hardware and webview implementation

# leto vs dorion    
- dorion uses tauri, while leto uses tao, wry, global-hotkeys, and muda which are the underlying libraries used by tauri.
- dorion is focused on compatibility and performance, while leto is focused on being a drop-in replacement for discord without auto-updates and without electron.

# contributing
- to launch use `cargo run`. This runs webpack aswell, im sure that windows and linux aren't working since I haven't added their custom implemnentations yet. If you have a windows or linux machine please open a PR and add platform specific code.
