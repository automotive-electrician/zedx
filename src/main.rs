#[allow(unused)]
mod consts;
#[allow(unused)]
mod pids;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.run()
}
