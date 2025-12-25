use linux_env_security::core::IntelliTool;
use anyhow::Result;

fn main() -> Result<()> {
    let tool = IntelliTool::new();
    
    println!("Initializing Users Enumerator...");
    tool.init()?;

    println!("Querying osquery for system users...");
    let json = tool.ask_os("select * from users;")?;

    println!("Generating report...");
    let html = tool.json_to_report(
        json, 
        "System Users Report", 
        "This report lists all user accounts on the system, including UID, GID, shell, and home directory information."
    );

    tool.generate_report(&html, "users_report.html")?;

    Ok(())
}
