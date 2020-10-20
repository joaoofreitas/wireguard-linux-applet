extern crate systray;

#[cfg(target_os = "linux")]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
   
   //Get the stdout of echo $PWD and add /assets/wg.svg 
    app.set_icon_from_file("$PWD/assets/wg.svg")?;

   //Run: tunsafe wg.conf & 
    app.add_menu_item("Turn On", |_| {
        println!("Turning it on.");
        Ok::<_, systray::Error>(())
    })?;
    
    //Run: xdg-open $PWD
    app.add_menu_item("Place confiiguration file", |_| {
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

/*
 * To do:
 *      - Solve Comments
 *      - Add in App About
 *      - Write and installation script for building the binaries
 *      - Write Docs
 *
 *
 *
 *
 *
 */


