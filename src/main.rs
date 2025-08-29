#![windows_subsystem = "windows"]
#[allow(warnings)]
use std::{env,rc, process::{self, ExitCode}};
use opentelemetry::{trace::{TraceError, Tracer, TraceContextExt, FutureExt, SpanKind, Span, get_active_span}, sdk::{trace::Config, Resource, propagation::TraceContextPropagator}, KeyValue, global, Key, Context};
use tracing::{info, span, log::warn, trace};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, fmt, util::SubscriberInitExt};
use window_titles::{Connection, ConnectionTrait};
use arboard::Clipboard;
use indexmap::{IndexMap};
use shlex;
extern crate linkify;
// mod log;
use linkify::{LinkFinder, LinkKind};
// use std::option::Option;
use fltk::{
    enums::{Color, FrameType, Event, CallbackTrigger},
    app::MouseButton,
    app::{App,*},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextDisplay},
    window::Window,
    button::{Button,CheckButton},
   input::Input,
    prelude::*, frame::Frame,
};

use serde::{Deserialize, Serialize};
use std::{process::{Command,Stdio}, error::Error, time::Duration, thread};
// use execute::{Execute, command};

use isahc::prelude::*;
// extern crate preferences;
use std::collections::HashMap;
// use abserde::*;

use std::fs::create_dir_all;
// const APP_INFO: AppInfo = AppInfo{name: "Perlink", author: "visnk"};
const appname: &str = "perlink";
fn eurl(t: String) -> Result<String,()> {
    // return Ok("try".to_string());
    println!("get {} val----->{}","expanding",t);
    let mut response = isahc::get(
        format!("{}{}",prefstore::getcustom(appname, "website.su", "https://unshorten.me/s/".to_string()).unwrap(),t).as_str()
    ).map_err(|op|{
        eprintln!("Could not get expanded url. error:{}",op)
    }).unwrap();
    // println!("get {} val----->{}","expanded url",response.text()?);

    // Print some basic info about the response to standard output.
    // println!("Status: {}", response.status());
    // println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
   
    return Ok(response.text().unwrap());
}
#[derive(Serialize, Deserialize, Default, Debug)]
struct MyConfig {
    shortenusing: String,
	// window_width: usize,
	// window_height: usize,
	// window_x: usize,
	// window_y: usize,
	// theme: String,
    #[serde(with = "indexmap::serde_seq")]
	user_data: IndexMap<String, String>,
}

// #[derive(Default)]
// struct vars{
//     jas: String,
// }

// const ChosenBrowser: &str = "dbrowser";
// const Notimes: &str = "ntimes";
// const Isenb: &str = "isenb";
// const PREFERENCES_KEY: &str = "prefs";
fn appendfile(browsername:String,browsercommand:String){
    prefstore::savepreference(appname, browsername,browsercommand);
    }


#[test]
fn init_try(){
    reinit();
}


fn reinit(){
    // Clear existing preferences first
    if let Ok(existing_browsers) = prefstore::getall(appname) {
        for (key, _) in existing_browsers {
            let _ = prefstore::clearpreference(appname, &key);
        }
    }

    #[cfg(target_os = "windows")] {
        // Use registry-based browser detection on Windows
        let detected_browsers = get_installed_browsers_from_registry();

        if detected_browsers.is_empty() {
            // Fallback to common browsers if registry detection fails
            println!("No browsers found in registry, using fallback list");
            let fallback_browsers = [
                ("firefox.exe", "Firefox"),
                ("chrome.exe", "Chrome"),
                ("msedge.exe", "Edge"),
                ("iexplore.exe", "Internet Explorer"),
            ];

            for (command, display_name) in fallback_browsers.iter() {
                prefstore::savepreference(appname, display_name.to_string(), command.to_string());
            }
        } else {
            // Use detected browsers from registry
            for (command, display_name) in detected_browsers {
                // println!("{}------{}",display_name,command.trim_matches('"'));
                prefstore::savepreference(appname, display_name, command.to_string());
            }
        }
    }

    #[cfg(target_os = "linux")] {
        // Linux browser commands
        let browsers_commands = [
            "firefox --private-window",
            "firefox",
            "google-chrome",
            "chromium",
            "waterfox",
            "vivaldi-stable",
            "opera",
        ];
        let browsers_display_names = [
            "Firefox Private",
            "Firefox",
            "Chrome",
            "Chromium",
            "Waterfox",
            "Vivaldi",
            "Opera",
        ];

        // Save browser preferences
        for (i, command) in browsers_commands.iter().enumerate() {
            if let Some(display_name) = browsers_display_names.get(i) {
                prefstore::savepreference(appname, display_name.to_string(), *command);
            }
        }
    }

    #[cfg(target_os = "macos")] {
        // macOS browser commands
        let browsers_commands = [
            "open -a Firefox --args --private-window",
            "open -a Firefox --args",
            "open -a 'Google Chrome' --args",
            "open -a Safari --args",
        ];
        let browsers_display_names = [
            "Firefox Private",
            "Firefox",
            "Chrome",
            "Safari",
        ];

        // Save browser preferences
        for (i, command) in browsers_commands.iter().enumerate() {
            if let Some(display_name) = browsers_display_names.get(i) {
                prefstore::savepreference(appname, display_name.to_string(), *command);
            }
        }
    }

    prefstore::savecustom(appname,"website.su", "https://unshorten.me/s/".to_string());
}

// }

pub fn link_finder_str(input: &str) -> Vec<String> {
    let mut links_str = Vec::new();
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let links: Vec<_> = finder.links(input).collect();

    for link in links.iter() {
        links_str.push(link.as_str().to_string());
    }
    links_str
}

use dotenv::dotenv;

// Windows-specific imports
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;
#[cfg(target_os = "windows")]
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn get_exe_path() -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
    use std::env;
    let current_exe = env::current_exe()?;
    Ok(current_exe.to_string_lossy().to_string())
}

#[cfg(target_os = "windows")]
#[test]
fn getbrowsers(){
    println!("Testing browser detection functionality...");
    println!("hello");
    let browsers = get_installed_browsers_from_registry();
    println!("Test completed. Found browsers: {:?}", browsers);

    // Additional test output
    for (command, display_name) in &browsers {
        println!("Browser: {} -> {}", display_name, command.trim_matches('"'));
    }
}
#[cfg(target_os = "windows")]
fn get_installed_browsers_from_registry() -> Vec<(String, String)> {
    let mut browsers = Vec::new();
    println!("Starting browser detection from Windows registry...");

    // Try HKLM first (system-wide installations)
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(clients_key) = hklm.open_subkey("SOFTWARE\\Clients\\StartMenuInternet") {
        println!("Checking HKEY_LOCAL_MACHINE for browsers...");
        for browser_key_result in clients_key.enum_keys() {
            if let Ok(browser_key_name) = browser_key_result {
                println!("Found browser key: {}", browser_key_name);
                if let Ok(browser_key) = clients_key.open_subkey(&browser_key_name) {
                    // Get display name
                    let display_name = browser_key.get_value("")
                        .unwrap_or_else(|_| browser_key_name.clone());

                    // Get command from shell/open/command
                    if let Ok(shell_key) = browser_key.open_subkey("shell\\open\\command") {
                        if let Ok(command) = shell_key.get_value("") {
                            println!("Found browser: {} -> {}", display_name, command);
                            browsers.push((command, display_name));
                        }
                    }
                }
            }
        }
    } else {
        println!("Could not access HKEY_LOCAL_MACHINE\\SOFTWARE\\Clients\\StartMenuInternet");
    }

    // Try HKCU (user-specific installations)
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(clients_key) = hkcu.open_subkey("SOFTWARE\\Clients\\StartMenuInternet") {
        println!("Checking HKEY_CURRENT_USER for browsers...");
        for browser_key_result in clients_key.enum_keys() {
            if let Ok(browser_key_name) = browser_key_result {
                println!("Found browser key: {}", browser_key_name);
                if let Ok(browser_key) = clients_key.open_subkey(&browser_key_name) {
                    // Get display name
                    let display_name = browser_key.get_value("")
                        .unwrap_or_else(|_| browser_key_name.clone());

                    // Get command from shell/open/command
                    if let Ok(shell_key) = browser_key.open_subkey("shell\\open\\command") {
                        if let Ok(command) = shell_key.get_value("") {
                            // Avoid duplicates
                            let command_str = command;
                            if !browsers.iter().any(|(existing_cmd, _)| existing_cmd == &command_str) {
                                println!("Found browser: {} -> {}", display_name, command_str);
                                browsers.push((command_str, display_name));
                            } else {
                                println!("Skipping duplicate browser: {} -> {}", display_name, command_str);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Could not access HKEY_CURRENT_USER\\SOFTWARE\\Clients\\StartMenuInternet");
    }

    println!("Browser detection completed. Found {} browsers.", browsers.len());
    browsers
}

#[cfg(target_os = "windows")]
fn register_protocol_handler() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let exe_path = get_exe_path()?;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Get just the filename (e.g., "perlink.exe") for application registration
    let exe_name = std::path::Path::new(&exe_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("perlink.exe");

    // Define ProgID for the application
    let prog_id = "Perlink.URLHandler";

    // Step 1: Define your app's ProgID
    let (progid_key, _) = hkcu.create_subkey(&format!("Software\\Classes\\{}", prog_id))?;
    progid_key.set_value("", &"Perlink Browser Chooser")?;
    progid_key.set_value("URL Protocol", &"")?;

    // Register the command for the ProgID
    let (progid_shell_key, _) = progid_key.create_subkey("shell\\open\\command")?;
    progid_shell_key.set_value("", &format!("\"{}\" \"%1\"", exe_path))?;

    // Step 2: Register your app under RegisteredApplications
    let (reg_apps_key, _) = hkcu.create_subkey("Software\\RegisteredApplications")?;
    reg_apps_key.set_value("Perlink", &format!("Software\\Perlink\\Capabilities"))?;

    // Step 3: Define your app's capabilities
    let (capabilities_key, _) = hkcu.create_subkey("Software\\Perlink\\Capabilities")?;
    capabilities_key.set_value("ApplicationName", &"Perlink Browser Chooser")?;
    capabilities_key.set_value("ApplicationDescription", &"A browser chooser application for selecting which browser to open URLs with")?;

    // Register supported URL protocols
    let (url_protocols_key, _) = capabilities_key.create_subkey("UrlAssociations")?;
    url_protocols_key.set_value("http", &prog_id)?;
    url_protocols_key.set_value("https", &prog_id)?;

    // Step 4: Associate your app with http (and optionally https) protocols
    // This is done through the capabilities registration above

    // Step 5: Provide a command to launch your app with the URL
    // This is done in the ProgID command registration above

    // Register the application in Applications section for additional compatibility
    let (app_reg_key, _) = hkcu.create_subkey(&format!("Software\\Classes\\Applications\\{}", exe_name))?;
    app_reg_key.set_value("", &"Perlink Browser Chooser")?;

    // Register supported protocols for the application (legacy support)
    let (protocols_key, _) = app_reg_key.create_subkey("SupportedTypes")?;
    protocols_key.set_value("http", &"")?;
    protocols_key.set_value("https", &"")?;

    // Register the command for the application (legacy support)
    let (app_shell_key, _) = app_reg_key.create_subkey("shell\\open\\command")?;
    app_shell_key.set_value("", &format!("\"{}\" \"%1\"", exe_path))?;

    // Register the custom perlink protocol
    let (perlink_key, _) = hkcu.create_subkey("Software\\Classes\\perlink")?;
    perlink_key.set_value("", &"URL:perlink Protocol")?;
    perlink_key.set_value("URL Protocol", &"")?;

    let (perlink_shell_key, _) = perlink_key.create_subkey("shell\\open\\command")?;
    perlink_shell_key.set_value("", &format!("\"{}\" \"%1\"", exe_path))?;

    println!("Protocol handler registered successfully using proper Windows registration method!");
    println!("Perlink should now appear in the list of available applications for HTTP/HTTPS protocols.");
    Ok(())
}

#[cfg(target_os = "windows")]
fn unregister_protocol_handler() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Get executable path to determine the application name
    let exe_path = get_exe_path().unwrap_or_else(|_| "perlink.exe".to_string());
    let exe_name = std::path::Path::new(&exe_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("perlink.exe");

    // Remove application registration
    let _ = hkcu.delete_subkey_all(&format!("Software\\Classes\\Applications\\{}", exe_name));

    // Remove protocol associations
    let _ = hkcu.delete_subkey_all("Software\\Classes\\http\\shell\\open\\command");
    let _ = hkcu.delete_subkey_all("Software\\Classes\\https\\shell\\open\\command");
    let _ = hkcu.delete_subkey_all("Software\\Classes\\perlink");

    println!("Protocol handler unregistered successfully!");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn register_protocol_handler() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Protocol handler registration is only supported on Windows.");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn unregister_protocol_handler() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Protocol handler unregistration is only supported on Windows.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>>  {
    dotenv().ok();
    // construct a subscriber that prints formatted traces to stdout
    // let subscriber = 
    // tracing_subscriber::FmtSubscriber::new();
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("perlink_main")
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        // Continue logging to stdout
        .with(fmt::Layer::default())
        .try_init()?;
    span!(tracing::Level::INFO, "init_started")
        .in_scope(||{
    
    let root = span!(tracing::Level::INFO, "init_setup", work_units = 2);
    info!("setup_crashreporting");
    // let ac_key = env::var("APPCENTER_KEY").unwrap();
    // app_center::start!(ac_key);
    human_panic::setup_panic!(human_panic::Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        path_to_save_log_to: prefstore::prefstore_directory(&appname.to_string()).unwrap(),
    });
    // let my_abserde = Abserde {
    //     app: appname.to_string(),
    //     location: Location::Auto,
    //     format: Format::Toml,
    // };
    info!("check_for_init_args");

    let args: Vec<String> = env::args().collect();
    match args.get(1) {

        Some(val) => match val {
            val => {
                println!("{}----------->",val);

                if val == "reinit"{
                    // let mut initspan=global::tracer("perlink").start("initconfig");
                    info!("reinit");
                    println!("Reinitilizing config file.");
                    reinit();
                    // initspan.end();
                    process::exit(0);

                }if val == "add"{
                    info!("add_browser");
                    println!("Added new browser.");
                    appendfile(args.get(2).unwrap().to_string(),args.get(3).unwrap().to_string());
                    process::exit(0);

                }
                if val == "clear"{
                    info!("clear_browser_list");
                    println!("Cleared browser list.");
                    prefstore::clearall(appname,"txt");
                    process::exit(0);

                }
                if val == "install"{
                    info!("install_protocol_handler");
                    println!("Installing protocol handler...");
                    if let Err(e) = register_protocol_handler() {
                        eprintln!("Failed to install protocol handler: {}", e);
                        process::exit(1);
                    }
                    process::exit(0);

                }
                if val == "uninstall"{
                    info!("uninstall_protocol_handler");
                    println!("Uninstalling protocol handler...");
                    if let Err(e) = unregister_protocol_handler() {
                        eprintln!("Failed to uninstall protocol handler: {}", e);
                        process::exit(1);
                    }
                    process::exit(0);

                }
                // Check if it's a URL (starts with http:// or https://)
                if val.starts_with("http://") || val.starts_with("https://") {
                    // It's a URL from protocol handler, continue to GUI
                    info!("url_from_protocol_handler");
                } else {
                    // Unknown command
                    println!("Unknown command: {}", val);
                    println!("Available commands: reinit, add, clear, install, uninstall");
                    process::exit(1);
                }
            }
            _ =>{

            },
            // Message::Stop => rlist(),
        },
        None => {

        },
    }
    
    
    let mut WIDGET_PADDING: i32 = 20;
    let mut WIDGET_WIDTH: i32 = 400;

    // Dynamic window height calculation based on number of installed browsers
    // This ensures the window fits all browser buttons comfortably
    let browser_count = prefstore::getall(appname).unwrap_or(vec![(String::new(),String::new())]).len() as i32;
    let browsers_per_row = 3; // Browser buttons are arranged in rows of 3
    let button_height = 40; // Height of each browser button
    let button_spacing = 5; // Spacing between button rows

    // Calculate how many rows of browsers we need
    let browser_rows = if browser_count > 0 {
        ((browser_count as f32) / (browsers_per_row as f32)).ceil() as i32
    } else {
        1 // minimum 1 row even if no browsers
    };

    // Calculate space needed for different UI sections:
    let header_section_height = 70; // URL frame + expand button + all browsers button
    let action_buttons_height = 70; // share via web + copy to clipboard buttons
    let browser_section_height = browser_rows * button_height + (browser_rows - 1) * button_spacing;
    let padding_and_spacing = WIDGET_PADDING * 4 + 60; // padding + frame spacing

    let mut WIDGET_HEIGHT: i32 = header_section_height + action_buttons_height + browser_section_height + padding_and_spacing;

    // Set reasonable min/max window heights
    if WIDGET_HEIGHT < 350 {
        WIDGET_HEIGHT = 350; // Minimum usable height
    } else if WIDGET_HEIGHT > 800 {
        WIDGET_HEIGHT = 800; // Maximum height to keep window manageable
    }

    println!("Dynamic window sizing: {} browsers -> {} rows -> {}px height", browser_count, browser_rows, WIDGET_HEIGHT);
    let args: Vec<String> = env::args().collect();
    let mut expandedurl = "".to_string();
    let mut ourl = "".to_string();
    // let mut sourl = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    // let mut sourl= vars{jas:"".to_string()};
    
    let mut strtoshow="";
    

    // let mut ourl = args.get(1).unwrap().to_string() ;
    // expandedurl = sk;
    // let (s, r) = fltk::app::channel();
    drop(root);
    let root = span!(tracing::Level::INFO, "loading_ui", work_units = 2);

            let mut app = App::default();
            
            let mut win = Window::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT).with_label("Choose browser");
            win.handle(move |f, ev|{
                // println!("{}----->{}",ev,fltk::app::event_text());
             match ev {
                fltk::enums::Event::KeyDown => {
                     if fltk::app::event_key() == fltk::enums::Key::from_char('f') {
                        // win.fullscreen(!win.fullscreen_active());
                    } else if fltk::app::event_key() == fltk::enums::Key::from_char('q') {
                        fltk::app::quit();
                    };
        
                    true
                }
                ,
                 _ => {
                     false
                 }
             }
});
let (s, r) = fltk::app::channel();

           
            let mut vpack=fltk::group::Pack::new(WIDGET_PADDING,
                WIDGET_PADDING,
                WIDGET_WIDTH - 40,
                WIDGET_HEIGHT - 40,"");
                win.resizable(&vpack);
              
                let mut framet = fltk::frame::Frame::default()
                .with_size(800,60)
                // .center_of(&win)
                .with_label("Loading");
              
            framet.set_label_size(12);
            let cfu = span!(tracing::Level::INFO, "get_url", work_units = 2);
            match args.get(1) {
                Some(val) => match val {

                    val => {
                        info!("Found_url_in_args");

                        expandedurl=val.to_string();
                        ourl=val.to_string();
                        setframe(&mut framet,&val);
                        // rt.set_label("");
                    }
                    _ =>{
                        info!("invalid_args");

                        expandedurl=" ".to_string();
                        ourl=" ".to_string();
                        setframe(&mut framet,&"invalid url".to_string());
                    },
                    // Message::Stop => rlist(),
                },
                None => {
                let cfu = span!(tracing::Level::INFO, "no_url_in_args", work_units = 2);

                    expandedurl=" ".to_string();
                    // let k=vars{jas:"".to_string()};
                    ourl=" ".to_string();
                    println!("here");
                    info!("Checking_in_window_titles");

                    let connection = Connection::new().unwrap();
                    // let mut pref = HashMap::<String,String>::new();
                    // let mut lks = vec!["", "New York"];
                    // let mut links: Vec<_>=;
                    for i in connection.window_titles().unwrap(){
                        // println!("{}",i.to_lowercase());
                        for kj in link_finder_str(&i){
                            info!("found_window");
                            let ss: String = kj.chars().skip(0).take(40).collect();
                            let mut b = Button::default()
                                    .with_size(70, 20)
                                    .with_label(&ss)
                                    // .with_align(Align::Left | Align::Inside)
                                    ;
                                    b.set_tooltip(&kj);
                                    b.emit(s.clone(),kj);
                                b.set_down_frame(FrameType::FlatBox);
                                b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
                                b.clear_visible_focus();
                             
                                b.set_frame(FrameType::FlatBox);
           
                        }
                    }

                    info!("Checking_in_clipboard");

                    let mut clipboard = Clipboard::new().unwrap();
                    match clipboard.get_text() {
                    Ok(sk) => { 
                        for kj in link_finder_str(&sk){
                    info!("found_clip");

                            let ss: String = kj.chars().skip(0).take(40).collect();
                            let mut b = Button::default()
                                .with_size(70, 20)
                                .with_label(&ss);
                            b.emit(s.clone(),kj.to_string());
                            b.set_tooltip(&kj);
                            b.set_down_frame(FrameType::FlatBox);
                            b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
                            b.clear_visible_focus();
                            b.set_frame(FrameType::FlatBox);
                        // println!("{}",kj);
                        }
                        
                        // fltk::dialog::message(90, 90, &sk);{
                            // let mut res = std::process::Command::new(format!("/home/roger/Downloads/waterfox/waterfox {}",sk)).output();
                        // }
                        
                        // ... use sk ...
                    },
                    Err(e) => {
                    info!("error_fetch_clipboard");

                        println!("Error Clipboard");
                        // setframe(&mut framet,"Error");
                        // ... sk is not available, and e explains why ...
                    },
                }
                drop(cfu);
                                       
                }
                    
            ,
            }
            
            drop(cfu);
            // println!("{}",ourl);
                fltk::frame::Frame::default().with_size(20, 10);
           
            let mut ttb=fltk::group::Pack::default().with_size(
                10,
                40) ;
                fltk::frame::Frame::default().with_size(20, 30);
            
            let mut eub = Button::default().with_size(150,30);
            eub.set_label("expand url");
            eub.emit(s.clone(),"expandurl".to_string());
            
            fltk::frame::Frame::default().with_size(20, 10);
            // let mut bframe1 = fltk::frame::Frame::default().with_size(300, 60);
            let mut b11 = Button::default().with_size(150,30);
            b11.set_label("All browsers");
            // b1.emit(s, "refresh".to_string());
            // let mut hpack=hpack.clone();
            b11.emit(s.clone(),"all".to_string());
            

            ttb.end();
            ttb.set_type(fltk::group::PackType::Horizontal);
            fltk::frame::Frame::default().with_size(10, 10);
            let mut ttb=fltk::group::Pack::default().with_size(
                10,
                40) ;
                
                
                fltk::frame::Frame::default().with_size(20, 30);
            
                let mut svw = Button::default().with_size(150,30);
                svw.set_label("share via web");
                svw.emit(s.clone(),"svw".to_string());
                
            
            fltk::frame::Frame::default().with_size(20, 10);
            // let mut bframe1 = fltk::frame::Frame::default().with_size(300, 60);
            let mut svc = Button::default().with_size(150,30);
                svc.set_label("copy to clipboard");
                svc.emit(s.clone(),"svc".to_string());
    

            ttb.end();
            ttb.set_type(fltk::group::PackType::Horizontal);
            fltk::frame::Frame::default().with_size(20, 30);
            let mut hpack=fltk::group::Pack::default().with_size(250,40) .center_of(&win);
                // let i=0;

                // browsers=browsers.clone();
                let mut i=0;
                // let mut bl:PreferencesMap<String> = setup();
                if(prefstore::getall(appname).unwrap_or(vec![(String::new(),String::new())]).is_empty()){
                    reinit();
                }
                
                // Collect all browsers and sort them by flipped name for constant order
                let mut browsers: Vec<(String, String)> = prefstore::getall(appname)
                    .unwrap_or(vec![(String::new(), String::new())])
                    .into_iter()
                    .filter(|(k, _)| !k.is_empty())
                    .collect();

                // Sort by flipped name for consistent order
                browsers.sort_by(|(k1, _), (k2, _)| {
                    k1.to_lowercase().cmp(&k2.to_lowercase())
                });

                for (k, v) in browsers {
                    let expandedurl = expandedurl.clone();
                    fltk::frame::Frame::default().with_size(20, 10);

                    // Flip the name for display (keep original for command)
                    let flipped_name = flip_browser_name(&k);
                    let display_name: String = flipped_name.chars().skip(0).take(10).collect();

                    let mut b1 = Button::default().with_size(90, 60);
                    b1.set_label(&format!("{}", display_name));
                    b1.emit(s.clone(), v);

                    i += 1;
                    if(i % 3 == 0) {
                        hpack.end();
                        hpack.set_type(fltk::group::PackType::Horizontal);
                        fltk::frame::Frame::default().with_size(20, 10);
                        hpack = fltk::group::Pack::default().with_size(250, 40).center_of(&win);
                    }
                }
                // let browsers = "";

            hpack.end();
            hpack.set_type(fltk::group::PackType::Horizontal);
            win.make_resizable(true);
            // win.resizable(&vpack);

            vpack.end();    
            vpack.set_type(fltk::group::PackType::Vertical);
            
            win.show_with_env_args();

            win.end();
            win.show();
            drop(root);
            span!(tracing::Level::INFO, "waiting_for_input")
        .in_scope(|| {
            info!("waiting for input");
            // let mut frame1 =frame.clone();
            // get_active_span(|span|async{
                while app.wait() {
                // setframe(&mut frame, "");
                // frame=frame.clone();
                match r.recv() {
                    
                    Some(val) => 
                    match val {
                        val => {
                            // get_active_span(|span| {
                            //     span.add_event("An event!".to_string(), vec![KeyValue::new("happened", true)]);
                            // });
                            
                            // if(val == "frominput"){
                            //             ourl=url.value();
                            //     }
                            // let mut str=val;
                            if(val.contains("//")){
                                info!("expanded_url");
                                // let k= format!("{}",val);
                                // frame.set_label(&k);
                                setframe(&mut framet, &val);
                                // println!("//------------->");

                                // println!("{}",format!("{}",val));
                            ourl=format!("{}",val);
                            expandedurl=val;
                            // rt.set_label("title");
                            // frame.set_label("");
                            // setframe(&mut frame,"");
                            
                            true;
                            }
                            else if val == "expandurl"{
                                info!("expand_url");
                                match eurl(ourl.clone()) {
                                    Ok(sk) => { 
                                        if(sk.to_lowercase().contains("invalid")){
                                            setframe(&mut framet,args.get(1).unwrap());
                                            // rt.set_label("");
                                        }
                                        else{
                                            setframe(&mut framet, &sk);
                                        }
                                        
                                        // fltk::dialog::message(90, 90, &sk);{
                                            // let mut res = std::process::Command::new(format!("/home/roger/Downloads/waterfox/waterfox {}",sk)).output();
                                        // }
                                        
                                        // ... use sk ...
                                    },
                                    Err(e) => {
                                        setframe(&mut framet,"Error");
                                        // ... sk is not available, and e explains why ...
                                    },
                                }
                            }
                            else if(val == "all"){
                                // println!("all------------->");
                                // span.add_event("opening".to_string(), vec![]);
                                // if ourl==" "{
                                //     ourl=url.value(); 
                                info!("opening_in_all_browsers");
                                let root = span!(tracing::Level::INFO, "opening_in_all", work_units = 2);
                                //  }
                                if(prefstore::getall(appname).unwrap_or(vec![(String::new(),String::new())]).is_empty()){
                                    reinit();
                                }
                                let(hmap)=prefstore::getall(appname).unwrap_or(vec![(String::new(),String::new())]);
                                
    ;
                            
                                // let cx = Context::current();
                                // let span = cx.span();
                                // span.add_event("Opening in all browsers".to_string(), vec![]);
                                // span.add_event("openinall".to_string(), vec![]);
                                for (_,v) in hmap{

                                        open(&v,&ourl);
                                }
                                drop(root);
                                true;
                            }
                            else if(val == "svc"){
                                info!("fromclipboard");
                                let mut clipboard = Clipboard::new().unwrap();
                                // println!("{}",&ourl);
                                #[cfg(target_os = "linux")]{
                                    clipboard.set().wait().text(&ourl).unwrap();
                                }
                                #[cfg(not(target_os = "linux"))]{
                                    clipboard.set_text(&ourl).unwrap();
                                }
                                // clipboard.set_text("abc".to_string()).unwrap();
                                // println!("{}",clipboard.get_text().unwrap());
                            }else if(val == "svw"){
                                // ada
                            }
                            // else 
                            else{
                                let root = span!(tracing::Level::INFO, "clicked", work_units = 2);
                                info!("clicked_{val}");
                                // if ourl==" "{
                                //     ourl=url.value(); 
                                //  }
                                 
                                // println!("{}------------->r{}r",val,expandedurl);
    // let tracer = global::tracer("opentracer");

                // span.add_event(val.to_string(), vec![]);

                
                            // let tracer = global::tracer("init");
                            
                            
                            // let cx = Context::current();
                            // let span = cx.span();
                            // span.add_event("opening in browser".to_string(), vec![]);
                                open(&val,&expandedurl);
                                // .with_context(cx).await;
                                // println!("opening----->{}",expandedurl);
                                drop(root);
                                
                                fltk::app::quit();
                                                true;
                            }
                            
                            // frame.set_label(&val);
                            
                        },
                        // Message::Stop => rlist(),
                    },
                    None => ({
                        // println!("stop");
                    })
                }
                
                // let frame=win.frame.clone();
                // frame.set_label("&val");
            }
        });
            warn!("Exiting");
        });
            // .with_context(cx);
            // });
        // global::shutdown_tracer_provider();
    process::exit(0);
            // app.run().unwrap();    
            Ok(())
}
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
const DAEMONIZE_ARG: &str = "__internal_daemonize";

fn flip_browser_name(name: &str) -> String {
    // Flip the name by reversing word order, e.g., "The firefox beta" -> "beta firefox the"
    let words: Vec<&str> = name.split_whitespace().collect();
    if words.is_empty() {
        return name.to_string();
    }
    words.into_iter().rev().collect::<Vec<&str>>().join(" ")
}

fn setframe(f:&mut Frame,s: &str){
    let ss: String = s.chars().skip(0).take(40).collect();
    f.set_label(&ss);
}
// async 

use std::path::Path;

fn open(v: &String, ourl: &String) -> Result<(), ()> {
    let root = span!(tracing::Level::INFO, "opening_browser", work_units = 2);

    // Split the input string into executable and arguments
    let parts: Vec<String> = shlex::split(v).ok_or_else(|| {
        eprintln!("Failed to parse browser path and arguments");
        ()
    })?;

    // If no executable is provided, use platform-specific default browser opener
    if parts.is_empty() {
        eprintln!("No executable provided, falling back to default browser");
        let (cmd, args): (&str, Vec<&str>) = if cfg!(target_os = "windows") {
            ("cmd.exe", vec!["/c", "start", "", ourl.as_str()])
        } else if cfg!(target_os = "macos") {
            ("open", vec![ourl.as_str()])
        } else if cfg!(target_os = "linux") {
            ("xdg-open", vec![ourl.as_str()])
        } else {
            eprintln!("Unsupported platform");
            return Err(());
        };

        let mut res = Command::new(cmd);
        res.args(&args);

        println!("Executing: {:?}", res);
        let tte = res
            .spawn()
            .map_err(|e| {
                eprintln!("Failed to execute default browser command: {:?}", e);
                ()
            })?;

        eprintln!("Process: {:?}", tte);
        drop(root);
        return Ok(());
    }
    println!("Executing: {:?}", parts);
    // Validate the executable path
    let executable = &parts[0];
    if !Path::new(executable).exists() {
        eprintln!("Executable does not exist: {:?}", executable);
        // return Err(());
    }

    // Use the provided browser executable and arguments
    let mut res = Command::new(executable);
    println!("Executing: {:?}", executable);

    // Add any additional arguments from the input (if any)
    if parts.len() > 1 {
        res.args(&parts[1..]);
    }

    // Add the URL as the final argument
    res.arg(ourl);

    match res.spawn() {
        Ok(tte) => {
            eprintln!("Process: {:?}", tte);
        }
        Err(e) => {
            eprintln!("Failed to execute process: {:?}", e);
        }
    }
    drop(root);

    Ok(())
}

#[test]
fn trybopen() {
    // Test with browser path and arguments
    let browser_path = if cfg!(target_os = "windows") {
        // Use double backslashes in raw string to ensure correct parsing
        r#""C:\Program Files\Google\Chrome\Application\chrome.exe" --incognito --new-window"#.to_string()
    } else if cfg!(target_os = "macos") {
        r#""/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --incognito --new-window"#.to_string()
    } else {
        r#"chromium --incognito --new-window"#.to_string()
    };

    // Test parsing and basic functionality (don't expect browser to exist)
    open(&browser_path, &"https://google.com".to_string()).unwrap();

    // The test passes as long as no panic occurs during parsing
}