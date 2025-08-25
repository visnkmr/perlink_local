#![windows_subsystem = "windows"]
#[allow(warnings)]
use std::{env,rc, process::{self, ExitCode}};
use opentelemetry::{trace::{TraceError, Tracer, TraceContextExt, FutureExt, SpanKind, Span, get_active_span}, sdk::{trace::Config, Resource, propagation::TraceContextPropagator}, KeyValue, global, Key as OtelKey, Context};
use tracing::{info, span, log::warn, trace};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, fmt, util::SubscriberInitExt};
use window_titles::{Connection, ConnectionTrait};
use arboard::Clipboard;
use indexmap::{IndexMap};
extern crate linkify;
// mod log;
use linkify::{LinkFinder, LinkKind};
// use std::option::Option;
use eframe::egui;
use egui::{RichText, FontId, Key, Vec2};
use egui_extras::RetainedImage;

use serde::{Deserialize, Serialize};
use std::{process::{Command,Stdio}, error::Error, time::Duration, thread};
// use execute::{Execute, command};

use isahc::prelude::*;
// extern crate preferences;
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
                prefstore::savepreference(appname, display_name, command.trim_matches('"').to_string());
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
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("perlink_main")
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(fmt::Layer::default())
        .try_init()?;

    span!(tracing::Level::INFO, "init_started")
        .in_scope(||{
            let root = span!(tracing::Level::INFO, "init_setup", work_units = 2);
            info!("setup_crashreporting");
            human_panic::setup_panic!(human_panic::Metadata {
                version: env!("CARGO_PKG_VERSION").into(),
                name: env!("CARGO_PKG_NAME").into(),
                authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
                homepage: env!("CARGO_PKG_HOMEPAGE").into(),
                path_to_save_log_to: prefstore::prefstore_directory(&appname.to_string()).unwrap(),
            });
            info!("check_for_init_args");

            let args: Vec<String> = env::args().collect();
            match args.get(1) {
                Some(val) => match val {
                    val => {
                        println!("{}----------->",val);

                        if val == "reinit"{
                            info!("reinit");
                            println!("Reinitilizing config file.");
                            reinit();
                            process::exit(0);
                        } else if val == "add"{
                            info!("add_browser");
                            println!("Added new browser.");
                            appendfile(args.get(2).unwrap().to_string(),args.get(3).unwrap().to_string());
                            process::exit(0);
                        } else if val == "clear"{
                            info!("clear_browser_list");
                            println!("Cleared browser list.");
                            prefstore::clearall(appname,"txt");
                            process::exit(0);
                        } else if val == "install"{
                            info!("install_protocol_handler");
                            println!("Installing protocol handler...");
                            if let Err(e) = register_protocol_handler() {
                                eprintln!("Failed to install protocol handler: {}", e);
                                process::exit(1);
                            }
                            process::exit(0);
                        } else if val == "uninstall"{
                            info!("uninstall_protocol_handler");
                            println!("Uninstalling protocol handler...");
                            if let Err(e) = unregister_protocol_handler() {
                                eprintln!("Failed to uninstall protocol handler: {}", e);
                                process::exit(1);
                            }
                            process::exit(0);
                        } else if val.starts_with("http://") || val.starts_with("https://") {
                            info!("url_from_protocol_handler");
                            // Launch GUI with URL
                            launch_gui(Some(val.to_string()));
                        } else {
                            println!("Unknown command: {}", val);
                            println!("Available commands: reinit, add, clear, install, uninstall");
                            process::exit(1);
                        }
                    }
                },
                None => {
                    // Launch GUI without URL
                    launch_gui(None);
                },
            }
        });

    Ok(())
}

fn launch_gui(initial_url: Option<String>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 700.0]) // Larger window to fit bigger buttons
            .with_title("Choose Browser"),
        ..Default::default()
    };

    eframe::run_native(
        "perlink",
        options,
        Box::new(|cc| {
            let mut app = PerlinkApp::new(initial_url, cc);
            Box::new(app)
        }),
    ).unwrap();
}

#[derive(Default)]
struct PerlinkApp {
    current_url: String,
    expanded_url: String,
    original_url: String,
    window_title_urls: Vec<String>,
    clipboard_urls: Vec<String>,
    browsers: Vec<(String, String)>, // (display_name, command)
    app_icon: Option<RetainedImage>, // Local app icon loaded with egui_extras
}

impl PerlinkApp {
    fn new(initial_url: Option<String>, _cc: &eframe::CreationContext) -> Self {
        let mut app = Self {
            current_url: String::new(),
            expanded_url: String::new(),
            original_url: String::new(),
            window_title_urls: Vec::new(),
            clipboard_urls: Vec::new(),
            browsers: Vec::new(),
            app_icon: None,
        };

        // Initialize browsers
        app.load_browsers();

        // Load local app icon using egui_extras
        app.load_app_icon();

        // Icons are loaded directly from URIs in the UI

        // Set initial URL if provided
        if let Some(url) = initial_url {
            app.set_url(url);
        } else {
            app.detect_urls();
        }

        app
    }

    fn load_browsers(&mut self) {
        if prefstore::getall(appname).unwrap_or(vec![]).is_empty() {
            reinit();
        }
        self.browsers = prefstore::getall(appname).unwrap_or(vec![]);
    }

    fn set_url(&mut self, url: String) {
        self.current_url = url.clone();
        self.expanded_url = url.clone();
        self.original_url = url;
    }

    fn detect_urls(&mut self) {
        self.window_title_urls.clear();
        self.clipboard_urls.clear();

        // Detect URLs from window titles
        let connection = Connection::new().unwrap();
        for title in connection.window_titles().unwrap() {
            for url in link_finder_str(&title) {
                self.window_title_urls.push(url);
            }
        }

        // Detect URLs from clipboard
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Ok(text) = clipboard.get_text() {
                for url in link_finder_str(&text) {
                    self.clipboard_urls.push(url);
                }
            }
        }
    }

    fn expand_url(&mut self) {
        if let Ok(expanded) = eurl(self.original_url.clone()) {
            if !expanded.to_lowercase().contains("invalid") {
                self.set_url(expanded);
            }
        }
    }

    fn open_in_browser(&self, browser_command: &str, url: &str) {
        if let Err(_) = open(&browser_command.to_string(), &url.to_string()) {
            eprintln!("Failed to open URL in browser");
        }
    }

    fn copy_to_clipboard(&self) {
        if let Ok(mut clipboard) = Clipboard::new() {
            #[cfg(target_os = "linux")] {
                let _ = clipboard.set().wait().text(&self.current_url);
            }
            #[cfg(not(target_os = "linux"))] {
                let _ = clipboard.set_text(&self.current_url);
            }
        }
    }

    fn get_browser_icon_url(&self, display_name: &str) -> String {
        let name_lower = display_name.to_lowercase();

        // Use Google's favicon service to get actual browser website icons
        if name_lower.contains("firefox") {
            "https://www.google.com/s2/favicons?domain=firefox.com&sz=128".to_string()
        } else if name_lower.contains("chrome") {
            "https://www.google.com/s2/favicons?domain=google.com&sz=32".to_string()
        } else if name_lower.contains("edge") {
            "https://www.google.com/s2/favicons?domain=microsoft.com&sz=32".to_string()
        } else if name_lower.contains("safari") {
            "https://www.google.com/s2/favicons?domain=apple.com&sz=32".to_string()
        } else if name_lower.contains("opera") {
            "https://www.google.com/s2/favicons?domain=opera.com&sz=32".to_string()
        } else if name_lower.contains("vivaldi") {
            "https://www.google.com/s2/favicons?domain=vivaldi.com&sz=32".to_string()
        } else if name_lower.contains("brave") {
            "https://www.google.com/s2/favicons?domain=brave.com&sz=32".to_string()
        } else if name_lower.contains("waterfox") {
            "https://www.google.com/s2/favicons?domain=waterfox.net&sz=32".to_string()
        } else if name_lower.contains("chromium") {
            "https://www.google.com/s2/favicons?domain=chromium.org&sz=32".to_string()
        } else {
            // Generic web browser icon
            "https://www.google.com/s2/favicons?domain=browser.com&sz=32".to_string()
        }
    }

    // Load local app icon using egui_extras RetainedImage
    fn load_app_icon(&mut self) {
        // Try to load the perlink_scr.png from the current directory
        if let Ok(image_bytes) = std::fs::read("perlink_scr.png") {
            // Use the image crate to decode the PNG
            if let Ok(image) = image::load_from_memory(&image_bytes) {
                let rgba_image = image.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                // Create a ColorImage from the raw pixels
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

                // Create RetainedImage from ColorImage
                let retained_image = RetainedImage::from_color_image("perlink_icon", color_image);
                self.app_icon = Some(retained_image);
            } else {
                eprintln!("Failed to decode perlink_scr.png as image");
            }
        } else {
            eprintln!("Could not read perlink_scr.png file");
        }
    }

    // Icons are loaded directly from URIs in the UI using egui::Image::from_uri

    // No longer needed - using direct image loading
}

impl eframe::App for PerlinkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        if ctx.input(|i| i.key_pressed(Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Display app icon if loaded
                if let Some(icon) = &self.app_icon {
                    ui.add_space(10.0);
                    icon.show_size(ui, Vec2::new(64.0, 64.0));
                    ui.add_space(10.0);
                }

                // URL display
                ui.add_space(20.0);
                ui.label(RichText::new(&self.current_url.chars().take(40).collect::<String>()).font(FontId::proportional(12.0)));

                ui.add_space(10.0);

                // Top buttons row
                ui.horizontal(|ui| {
                    if ui.button("expand url").clicked() {
                        self.expand_url();
                    }
                    if ui.button("All browsers").clicked() {
                        for (_, command) in &self.browsers {
                            self.open_in_browser(command, &self.current_url);
                        }
                    }
                });

                ui.add_space(10.0);

                // Browser list with icons
                // ui.label("Available browsers:");
                // ui.horizontal_wrapped(|ui| {
                //     for (display_name, _) in &self.browsers {
                //         ui.horizontal(|ui| {
                //             // Load and show icon from Google favicon API
                //             let icon_url = self.get_browser_icon_url(display_name);
                //             let image = egui::Image::from_uri(icon_url)
                //                 .fit_to_exact_size(Vec2::new(16.0, 16.0))
                //                 .rounding(2.0);
                //             ui.add(image);

                //             ui.label(RichText::new(display_name).font(FontId::proportional(10.0)));
                //         });
                //     }
                // });

                // ui.add_space(10.0);

                // Action buttons row
                ui.horizontal(|ui| {
                    if ui.button("share via web").clicked() {
                        // TODO: implement share via web
                    }
                    if ui.button("copy to clipboard").clicked() {
                        self.copy_to_clipboard();
                    }
                });

                ui.add_space(20.0);

                // URL detection buttons
                if !self.window_title_urls.is_empty() {
                    ui.label("URLs from window titles:");
                    let mut clicked_url = None;
                    for url in &self.window_title_urls {
                        let display_text: String = url.chars().take(40).collect();
                        if ui.button(&display_text).on_hover_text(url).clicked() {
                            clicked_url = Some(url.clone());
                        }
                    }
                    if let Some(url) = clicked_url {
                        self.set_url(url);
                    }
                }

                if !self.clipboard_urls.is_empty() {
                    ui.label("URLs from clipboard:");
                    let mut clicked_url = None;
                    for url in &self.clipboard_urls {
                        let display_text: String = url.chars().take(40).collect();
                        if ui.button(&display_text).on_hover_text(url).clicked() {
                            clicked_url = Some(url.clone());
                        }
                    }
                    if let Some(url) = clicked_url {
                        self.set_url(url);
                    }
                }

                ui.add_space(20.0);

                // Centered browser buttons with larger size
                ui.add_space(20.0);
                let mut clicked_browser = None;

                ui.vertical_centered(|ui| {
                    ui.heading("Choose Browser");
                    ui.add_space(15.0);

                    let browsers_per_row = 3; // Reduced to 2 per row for bigger buttons

                    for (i, (display_name, command)) in self.browsers.iter().enumerate() {
                        if i % browsers_per_row == 0 {
                            // Use centered layout for each row of buttons
                            ui.horizontal(|ui| {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 10.0; // Add spacing between buttons
                                    for j in 0..browsers_per_row {
                                        let idx = i + j;
                                        if idx < self.browsers.len() {
                                            let (btn_name, cmd) = &self.browsers[idx];
                                            let btn_text = btn_name.chars().take(15).collect::<String>(); // Allow longer names

                                            // Create larger, more prominent buttons
                                            let button = egui::Button::new(RichText::new(&btn_text).font(FontId::proportional(16.0)))
                                                .min_size(Vec2::new(140.0, 50.0)) // Larger button size
                                                .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(100, 150, 200)))
                                                .rounding(8.0);

                                            if ui.add(button).clicked() {
                                                clicked_browser = Some(cmd.clone());
                                            }
                                        }
                                    }
                                });
                                ui.add_space(20.0);
                            });
                        }
                    }
                });

                if let Some(cmd) = clicked_browser {
                    self.open_in_browser(&cmd, &self.current_url);
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

// async
fn open(v: &String, ourl: &String) -> Result<(), ()> {
    let root = span!(tracing::Level::INFO, "opening_browser", work_units = 2);

    let strings: Vec<String> = v.split_whitespace().map(str::to_string).collect();
    let mut res = Command::new(format!("{}", strings[0]));
    let slice = &strings[1..strings.len()];

    for k in slice {
        res.arg(k);
    }

    let tte = res.arg(format!("{}", ourl))
        .spawn()
        .expect("failed to execute process");
    eprintln!("{:?}", tte);
    drop(root);

    Ok(())
}