//#![allow(unused)]
extern crate systray;

#[cfg(target_os = "linux")]
fn main() -> Result<(), systray::Error> {
    let mut app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
   
   //Get the stdout of echo $PWD and add /assets/wg.svg 
    let pwd: String = command_run("pwd");
    app.set_icon_from_file(&(pwd + "/assets/wg.svg"))?;

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

fn command_run(command: &str) -> std::string::String {
    use std::process::Command;

    let output = Command::new(&command) 
            .output()
            .expect("Failed to run command");

    let out = output.stdout;
    println!("{}", String::from_utf8_lossy(remove_slash(&out)));
    
    return String::from_utf8_lossy(remove_slash(&out)).to_string();
}

fn remove_slash(slice: &Vec<u8>) -> &[u8]{
    return &slice[0..&slice.len()-1];
}


/*
 * To do:
 *      - Solve Comments
 *      - Add in App About
 *      - Write and installation script for building the binaries
 *      - Write Docs
 */

