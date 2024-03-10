use windows::{
    core::*,
    Storage::ApplicationData,
    Win32::{
        Foundation::{self, HWND},
        System::Threading::OpenProcess,
        UI::WindowsAndMessaging::{EnumWindows, MessageBoxW, MB_OK, MESSAGEBOX_STYLE},
    },
};

/*  This function is referred to
[Native Windows GUI](https://github.com/gabdube/native-windows-gui/tree/master?tab=readme-ov-file) of
native_windows_gui::enable_visual_styles*/
pub fn enable_visual_styles() -> Result<()> {
    let temp_folder = ApplicationData::Current()?;
    println!("{:?}", temp_folder);
    Ok(())
}

#[test]
fn temp_folder() -> Result<()> {
    let temp_folder = ApplicationData::Current()?;
    println!("{:?}", temp_folder);
    Ok(())
}
