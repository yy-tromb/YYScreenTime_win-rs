<div style="color:#10dd10">

## ***Now, I'm developping slowly, Please wait a lot***

</div>

# YYScreenTime_win-rs
Windows ScreenTime App for me.  
***This app is replacement app of [YYScreenTime_win](https://github.com/yy-tromb/YYScreenTime_win) that written in C and Win32 APIs.*** I like Rust much more than C, so I have rewrited. Developping in C was very very painful.lol Whatever it is, I'm a beginner, though.  

## For Developper
GUI crate: [iced-rs](https://github.com/iced-rs/iced/tree/master)  
Error crate: [anyhow](https://github.com/dtolnay/anyhow)  
Windows crate: [windows-rs](https://github.com/microsoft/windows-rs)

## Fonts
In `/fonts` directory, [IBM Plex Sans JP (Google Fonts Site)](https://fonts.google.com/specimen/IBM+Plex+Sans+JP/about) in [IBM Plex® project](https://github.com/IBM/plex) is used. This Font Software is licensed under the SIL Open Font License, Version 1.1.

## Thanks
- Thanks to [Gabriel Dube](https://github.com/gabdube), author of the [Native Windows GUI](https://github.com/gabdube/native-windows-gui), for my implementing [windows_tools::enable_visual_styles](https://github.com/yy-tromb/YYScreenTime_win-rs/blob/main/src/windows_tools.rs#L38), which can be found in [native_windows_gui::enable_visual_styles (in native_windows_gui/src/win32/mod.rs)](https://github.com/gabdube/native-windows-gui/blob/master/native-windows-gui/src/win32/mod.rs#L98) for reference.
