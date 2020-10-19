extern crate systray;
extern crate gtk;

#[cfg(target_os = "linux")]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
   
   //need to check this. 
    app.set_icon_from_file("/home/johnny/Documents/Projects/wireguard-linux-applet/assets/wg.svg")?;

    
    app.add_menu_item("Turn On.", |_| {
        println!("Turning it on.");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Import Config File", |_| {
        println!("Importing Config.");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}
