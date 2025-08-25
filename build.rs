#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();

    // Set executable metadata
    res.set("FileDescription", "Perlink Browser Chooser");
    res.set("ProductName", "Perlink");
    res.set("CompanyName", "visnk");

    // Check for custom icon file (optional)
    // To add a custom icon:
    // 1. Create a perlink.ico file (16x16, 32x32, 48x48, 256x256 recommended)
    // 2. Place it in the project root directory
    // 3. The build script will automatically use it
    if std::path::Path::new("perlink.ico").exists() {
        println!("cargo:warning=Using custom icon: perlink.ico");
        res.set_icon("perlink.ico");
    } else {
        println!("cargo:warning=No custom icon found. Using default Windows icon.");
        println!("cargo:warning=To add a custom icon, create a perlink.ico file in the project root.");
    }

    if let Err(e) = res.compile() {
        eprintln!("Failed to compile Windows resources: {}", e);
        std::process::exit(1);
    }

    println!("cargo:warning=Windows executable icon and metadata configured successfully!");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // No special build steps needed for non-Windows platforms
}