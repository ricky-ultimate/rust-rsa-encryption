use crate::win1::StartWindow;
use crate::win2::EncryptionWindow;
use crate::win3::DecryptionWindow;
use crate::win4::KeyGenWindow;
use slint::ComponentHandle;

mod core;
mod display;
mod utils;

mod win1 {
    include!(env!("SLINT_INCLUDE_START"));
}
mod win2 {
    include!(env!("SLINT_INCLUDE_ENCRYPT"));
}

mod win3 {
    include!(env!("SLINT_INCLUDE_DECRYPT"));
}

mod win4 {
    include!(env!("SLINT_INCLUDE_KEYGEN"));
}

fn main() -> Result<(), slint::PlatformError> {
    let start_win = StartWindow::new()?;
    let key_gen_win = KeyGenWindow::new()?;
    let encrypt_win = EncryptionWindow::new()?;
    let decrypt_win = DecryptionWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    start_win.run()
}
