use linux_env_security::core::IntelliTool;
use anyhow::Result;

fn main() -> Result<()> {
    let tool = IntelliTool::new();
    
    println!("Initializing NPM Packages Enumerator...");
    tool.init()?;

    println!("Querying osquery for NPM packages...");
    let json = tool.ask_os("select * from npm_packages;")?;

    println!("Generating report...");
    let html = tool.json_to_report(
        json, 
        "NPM Packages Report", 
        "This report lists installed NPM packages found on the system, including version, description, and license information."
    );

    tool.generate_report(&html, "npm_packages_report.html")?;

    Ok(())
}
