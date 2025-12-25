use linux_env_security::core::IntelliTool;
use anyhow::Result;

fn main() -> Result<()> {
    let tool = IntelliTool::new();
    
    println!("Initializing Processes Enumerator...");
    tool.init()?;

    println!("Querying osquery for system processes...");
    let json = tool.ask_os("select * from processes;")?;

    println!("Generating report...");
    let html = tool.json_to_report(
        json, 
        "System Processes Report", 
        "This report lists all running processes on the system, including PID, name, path, and command line arguments."
    );

    tool.generate_report(&html, "processes_report.html")?;

    Ok(())
}
