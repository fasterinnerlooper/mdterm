// Build script for chafa-sys
// Handles linking to libchafa

fn main() {
    // Try to find libchafa using pkg-config with explicit path
    let package_name = "libchafa";
    
    // First try with explicit PKG_CONFIG_PATH
    let pkg_config_paths = vec![
        std::env::var("PKG_CONFIG_PATH").unwrap_or_default(),
        "/usr/lib/x86_64-linux-gnu/pkgconfig".to_string(),
        "/usr/lib/pkgconfig".to_string(),
        "/usr/share/pkgconfig".to_string(),
    ];
    
    // Try to find chafa.pc
    for path in &pkg_config_paths {
        if path.is_empty() {
            continue;
        }
        let pc_path = std::path::Path::new(path).join("chafa.pc");
        if pc_path.exists() {
            println!("cargo:warning=Found chafa.pc at {:?}", pc_path);
            break;
        }
    }
    
    // Now try pkg-config
    let result = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe(package_name);
    
    match result {
        Ok(lib) => {
            // Found via pkg-config - link to the library
            println!("cargo:warning=Found libchafa via pkg-config");
            
            for path in lib.include_paths {
                println!("cargo:include={}", path.display());
            }
            
            // Link to the library
            println!("cargo:rustc-link-lib=chafa");
            
            // Add library search path
            for path in lib.link_paths {
                println!("cargo:rustc-link-search=native={}", path.display());
            }
        }
        Err(_) => {
            // Fall back to searching common library paths
            println!("cargo:warning=Could not find libchafa via pkg-config");
            
            // Try common library paths
            let common_paths = vec![
                "/lib/x86_64-linux-gnu",
                "/usr/lib/x86_64-linux-gnu",
                "/usr/lib",
                "/lib",
            ];
            
            let mut found = false;
            for lib_path in &common_paths {
                let so_path = std::path::Path::new(lib_path).join("libchafa.so");
                if so_path.exists() || std::path::Path::new(&format!("{}/libchafa.so.0", lib_path)).exists() {
                    println!("cargo:warning=Found libchafa.so in {}", lib_path);
                    println!("cargo:rustc-link-search=native={}", lib_path);
                    println!("cargo:rustc-link-lib=chafa");
                    found = true;
                    break;
                }
            }
            
            if !found {
                println!("cargo:warning=libchafa will be loaded dynamically at runtime.");
                println!("cargo:warning=Please install libchafa if you encounter runtime errors:");
                println!("cargo:warning=  Windows (MSYS2): pacman -S mingw-w64-x86_64-chafa");
                println!("cargo:warning=  Linux (Ubuntu): apt install libchafa-dev");
                println!("cargo:warning=  macOS: brew install chafa");
            }
        }
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}
