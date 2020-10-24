# Wireguard Linux Applet

###### A GTK3-based Wireguard VPN applet that helps you connect and disconnect easily to your server.

##### On development... 🚧


### What is this? 🤔
Wireguard Linux Applet is a user-friendly GUI VPN Client for Linux Users. It relies on Tunsafe to run connection commands in the background.

### Purpose 🤠
What motivated me to build this project is the fact that there's no minimalist VPN client applet for Wireguard VPN servers. I didn't need a GUI but it would be useful to have a one in my Linux workflow.

### Why in Rust? ⚙️
Actually making it in C++ or Go would be way easier. But I wanted to learn Rust, and this one of the challenges I purpose myself. Even though the system tray library used it's still on early development.

### How was it build and how does it work? 🔨
This applet uses GTK based library called systray-rs to create a simple applet GUI easily and uses Tunsafe terminal program to run the connect to the Wireguard VPN. Basically, I added an icon, buttons and dumb-proof config locations so anyone minimally used to GNU/Linux could install it and use it freely.

### Installation 📝
##### On development... 🚧

### Usage 🖥️
##### On development... 🚧


### Maintenance and Future Development ℹ️
This program is being in constant development depending also in the development of:

- [Wireguard](https://www.wireguard.com/)
- [systray-rs](https://github.com/qdot/systray-rs)
- [Tunsafe](https://tunsafe.com/)

If you have any concerns, errors, or if you want more features feel free to open an issue, or even contact me on Twitter.

