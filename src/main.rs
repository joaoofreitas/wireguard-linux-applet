//#![allow(unused)]
extern crate systray;
use std::process::Command;

#[cfg(target_os = "linux")]
fn main() -> Result<(), systray::Error> {
    let mut app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
   
    let pwd: String = pwd_get();
    app.set_icon_from_file(&(pwd + "/assets/wg.svg"))?;

    app.add_menu_item("Turn On", |_| {
        println!("Turning it on.");
        turn_vpn_on();
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Turn Off", |_| {
        println!("Turning it off.");
        turn_vpn_off();
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Drag/Select Wireguard Configuration File", |_| {
        println!("Import Configuration");
        open_config();
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;
    
    app.add_menu_item("About", |_| {
        open_url("https://www.github.com/joaoofreitas");
        Ok::<_, systray::Error>(())
    })?;
    
    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

fn open_url(url: &str){ 
    Command::new("xdg-open")
            .arg(url)
            .spawn()
            .expect("Failed to open your browser. Exiting");
}

fn pwd_get() -> std::string::String {
    let output = Command::new("pwd") 
            .output()
            .expect("Failed to get the location of the installation");

    let out = output.stdout;
    println!("{}", String::from_utf8_lossy(remove_slash(&out)));
    
    return String::from_utf8_lossy(remove_slash(&out)).to_string();
}

fn remove_slash(slice: &Vec<u8>) -> &[u8]{
    return &slice[0..&slice.len()-1];
}

fn open_config(){
    Command::new("xdg-open")
            .arg(pwd_get() + "/config")
            .spawn()
            .expect("Failed to open your file manager.");
}

//Try to change this hardcoded part
fn turn_vpn_on() {
    Command::new("tunsafe")
            .arg(pwd_get() + "/config/config.conf")
            .spawn()
            .expect("Failed Connecting to VPN");
}

fn turn_vpn_off() {
    Command::new("pkill")
            .arg("tunsafe")
            .spawn()
            .expect("Failed Killing VPN");
}

/*
 * To do:
 *      - Solve Comments
 *      - Add in App About
 *      - Write and installation script for building the binaries
 *      - Write Docs
 */

