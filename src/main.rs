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
    app.set_icon_from_file("$PWD/assets/wg.svg")?;

   //Run: tunsafe wg.conf & 
    app.add_menu_item("Turn On", |_| {
        println!("Turning it on.");
        getFolder();
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

fn getFolder(){
    use std::process::Command;
    let output = Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output(); 
    //return String::from_utf8_lossy(&out.stdout);
    //
    let out = output.stdout;
    println!("{:?}", String::from_utf8_lossy(&out));
}

/*
 * To do:
 *      - Solve Comments
 *      - Add in App About
 *      - Write and installation script for building the binaries
 *      - Write Docs
 *
 *
 * NOTES:
 
#![allow(unused)]
fn main() {
use std::process::Command;

let output = Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("failed to execute process");


let hello = output.stdout;
println!("{:?}", remove_slash(&hello));
println!("{:?}", String::from_utf8_lossy(remove_slash(&hello)));
}

fn remove_slash(mut slice: &Vec<u8>) -> &[u8]{
    return &slice[0..&slice.len()-1];
} 


