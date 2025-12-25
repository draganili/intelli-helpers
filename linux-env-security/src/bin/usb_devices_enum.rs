use linux_env_security::core::IntelliTool;
use anyhow::Result;

fn main() -> Result<()> {
    let tool = IntelliTool::new();
    
    println!("Initializing USB Devices Explanator...");
    tool.init()?;

    println!("Querying osquery for USB devices...");
    let json = tool.ask_os("select * from usb_devices;")?;

    println!("Generating report...");
    let html = tool.json_to_report(
        json, 
        "USB Devices Report", 
        "This report lists all USB devices currently connected to the system, including details like vendor, model, and serial number."
    );

    tool.generate_report(&html, "usb_devices_report.html")?;

    Ok(())
}
