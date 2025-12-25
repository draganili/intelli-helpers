use linux_env_security::core::IntelliTool;
use anyhow::Result;

fn main() -> Result<()> {
    let tool = IntelliTool::new();
    
    println!("Initializing System Information Tool...");
    tool.init()?;

    println!("Querying osquery for system information...");
    let json = tool.ask_os("select * from system_info;")?;

    println!("Generating report...");
    let html = tool.json_to_report(
        json, 
        "System Information Report", 
        "This report provides details about the system hardware and OS, including hostname, CPU, memory, and OS version."
    );

    tool.generate_report(&html, "system_information_report.html")?;

    Ok(())
}
