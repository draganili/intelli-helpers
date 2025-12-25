# 🛡️ IntelliSec Children - Linux Environment Security

<div align="center">
  <img src="../pentestiverse_logo.png" alt="Pentestiverse Logo" width="200" />
  <br />
  <h3>Powered by <a href="https://pentestiverse.com">Pentestiverse</a></h3>
  <p>
    <b>Audit. Visualize. Secure.</b>
    <br />
    A suite of standalone, offline security auditing tools for Linux.
  </p>
</div>

---

## 🚀 Overview

**IntelliSec Children** is a collection of powerful, standalone console tools designed to audit various aspects of your Linux environment. Leveraging the robust capabilities of `osquery`, these tools collect critical system data and generate **beautiful, dark-themed HTML reports** instantly.

### ✨ Key Features

*   **🔒 100% Offline & Private**: No data leaves your machine. All reports are generated locally.
*   **🎨 Beautiful Visualization**: Modern, dark-themed HTML reports with easy-to-read tables.
*   **⚡ Instant Results**: Fast execution and immediate report generation.
*   **🐧 Linux Native**: Optimized for Linux environments (Ubuntu/Debian tested).
*   **📦 Standalone**: Single binary execution (requires `os` executable).

---

## 🛠️ Available Tools

| Tool Name | Executable | Description |
| :--- | :--- | :--- |
| **🔌 USB Devices** | `usb_devices_enum` | Audits connected USB hardware (Vendor, Model, Serial). |
| **👥 Users** | `users_enum` | Enumerates system accounts, UIDs, GIDs, and home directories. |
| **💻 System Info** | `system_information` | Detailed hardware and OS build information (CPU, RAM, Hostname). |
| **📦 NPM Packages** | `npm_packages_enum` | Lists installed Node.js packages, versions, and licenses. |
| **⚙️ Processes** | `processes_enum` | Snapshots all running processes, PIDs, and command arguments. |

---

## 📖 How to Use

Each tool functions identically. Follow these simple steps to generate your security report.

### Prerequisites

1.  **Root Privileges**: `sudo` is required to access low-level system details.
2.  **`os` Executable**: The `os` (osquery) binary must be in the same folder as the tool.

### Step-by-Step Guide

1.  **Download/Compile** the desired tool binary.
2.  Place the tool and the `os` file in the same directory.
3.  Open a terminal and run the tool with `sudo`:

    ```bash
    # Example for USB Devices tool
    sudo ./usb_devices_enum
    ```

4.  **View Report**: The tool will generate an HTML file (e.g., `usb_devices_report.html`) in the current directory and attempt to automatically open it in your default web browser.

---

## 🏗️ Building from Source

If you prefer to build the tools yourself:

```bash
# Clone the repository
git clone https://github.com/draganili/intelli-helpers.git
cd intelli-helpers/linux-env-security

# Build all tools
cargo build --release --bins
```

The binaries will be available in `target/release/`.

---

## 📄 License & Credits

These tools are provided freely by **[Pentestiverse](https://pentestiverse.com)** to help the community secure their infrastructure.

*   **Maintainer**: Pentestiverse
*   **License**: MIT

<div align="center">
  <sub>Made with ❤️ for a safer digital world.</sub>
</div>
