fn main() {
    // Only set Windows subsystem for release builds
    #[cfg(all(windows, not(debug_assertions)))]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico"); // Optional: add an icon
        // This makes it a GUI app with no console window
        res.set("OriginalFilename", "WinSysUtils.exe");
        res.set("FileDescription", "Windows System Utility");
        res.set("ProductName", "Windows System Utility");
        res.set("CompanyName", "Microsoft Corporation");
        let _ = res.compile();
    }
}

