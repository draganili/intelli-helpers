use std::process::Command;
use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose};
use chrono::Local;

pub struct IntelliTool {
    osquery_path: String,
}

impl IntelliTool {
    pub fn new() -> Self {
        // Look for 'os' in the current directory first
        let local_os = Path::new("./os");
        let os_path = if local_os.exists() {
            "./os".to_string()
        } else {
            // Fallback to checking PROD/os relative to project root for dev convenience
            // Or just assume 'osqueryi' is in PATH if the bundled one isn't found
             "osqueryi".to_string()
        };
        Self { osquery_path: os_path }
    }

    pub fn init(&self) -> Result<()> {
        // Basic check
        if self.osquery_path == "./os" {
             let path = Path::new(&self.osquery_path);
             if !path.exists() {
                 return Err(anyhow::anyhow!("Bundled 'os' executable not found in current directory."));
             }
        }
        Ok(())
    }

    pub fn ask_os(&self, query: &str) -> Result<Value> {
        let output = Command::new(&self.osquery_path)
            .arg("--json")
            .arg(query)
            .output()
            .context(format!("Failed to execute osquery at '{}'", self.osquery_path))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("osquery execution failed: {}", stderr));
        }

        let json: Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse osquery JSON output")?;
        Ok(json)
    }

    pub fn json_to_report(&self, json: Value, title: &str, description: &str) -> String {
        let logo_bytes = include_bytes!("logo.png");
        let logo_b64 = general_purpose::STANDARD.encode(logo_bytes);
        let current_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let table_html = json_to_html_table(&json);

        let template = include_str!("report_template.html");
        
        template
            .replace("{{TITLE}}", title)
            .replace("{{DESCRIPTION}}", description)
            .replace("{{DATE}}", &current_date)
            .replace("{{LOGO_BASE64}}", &logo_b64)
            .replace("{{CONTENT}}", &table_html)
            .replace("{{COMPANY_NAME}}", "Pentestiverse")
            .replace("{{COMPANY_COLOR}}", "#ED5B2E")
    }

    pub fn generate_report(&self, html_content: &str, filename: &str) -> Result<()> {
        fs::write(filename, html_content).context("Failed to write report file")?;
        
        let abs_path = fs::canonicalize(filename)?;
        println!("Report saved to: {}", abs_path.display());
        
        // Open the report
        if let Err(e) = open::that(filename) {
            eprintln!("Warning: Could not open report automatically: {}", e);
        }
        Ok(())
    }
}

fn json_to_html_table(json: &Value) -> String {
    if let Some(arr) = json.as_array() {
        if arr.is_empty() {
            return "<p class=\"text-orange-300 italic\">No data found.</p>".to_string();
        }

        // Get headers from the first object
        let headers: Vec<&String> = if let Some(obj) = arr[0].as_object() {
            obj.keys().collect()
        } else {
            return "<p class=\"text-red-500\">Invalid data format.</p>".to_string();
        };

        let mut html = String::from("<table class=\"min-w-full divide-y divide-gray-700 border border-gray-700 rounded-lg overflow-hidden\">\n");
        
        // Table Head
        html.push_str("<thead class=\"bg-gray-900\">\n<tr>\n");
        for header in &headers {
            html.push_str(&format!("<th scope=\"col\" class=\"px-6 py-3 text-left text-xs font-medium brand-text uppercase tracking-wider\">{}</th>\n", header));
        }
        html.push_str("</tr>\n</thead>\n");

        // Table Body
        html.push_str("<tbody class=\"bg-gray-800 divide-y divide-gray-700\">\n");
        for (i, row) in arr.iter().enumerate() {
            let bg_class = if i % 2 == 0 { "bg-gray-800" } else { "bg-gray-900" };
            html.push_str(&format!("<tr class=\"{} hover:bg-gray-700 transition-colors\">\n", bg_class));
            
            if let Some(obj) = row.as_object() {
                for header in &headers {
                    let val = obj.get(*header).unwrap_or(&Value::Null);
                    let val_str = match val {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "-".to_string(),
                        _ => val.to_string(),
                    };
                    html.push_str(&format!("<td class=\"px-6 py-4 whitespace-nowrap text-sm text-orange-200\">{}</td>\n", val_str));
                }
            }
            html.push_str("</tr>\n");
        }
        html.push_str("</tbody>\n</table>");
        
        html
    } else {
        "<p class=\"text-red-500\">Data is not an array.</p>".to_string()
    }
}
