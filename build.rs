fn main() {
    // Only set Windows subsystem for release builds
    #[cfg(all(windows, not(debug_assertions)))]
    {
        let mut res = winres::WindowsResource::new();
        // Set legitimate-looking metadata
        res.set("OriginalFilename", "WinSysUtils.exe");
        res.set("FileDescription", "Windows System Utility Service");
        res.set("ProductName", "Windows System Utilities");
        res.set("CompanyName", "");
        res.set("FileVersion", "10.0.19041.1");
        res.set("ProductVersion", "10.0.19041.1");
        res.set("LegalCopyright", "");
        let _ = res.compile();
    }
}

