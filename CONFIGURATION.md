# Configuration Documentation

## Introduction

SCU is a fetch utility designed to display system information. It supports two display modes: "table" (default) and "neomimic".

## Configuration Directory

By default, SCU creates a configuration directory at `$HOME/.config/omnid/scu`. This directory stores configuration files in `.json` format. You can create multiple configuration files and select which one to use by passing the `--config` argument when running SCU. The argument can be either:

- The name of the configuration file (e.g., `custom` or `custom.json`), if it’s located in the default directory.
- The absolute path to the `.json` file (e.g., `/path/to/custom.json`).

## Configuration File Structure

A SCU configuration file is a JSON object with three main sections:

- **`global`**: Defines boolean flags that control SCU’s behavior, mirroring launch arguments.
- **`table`**: Configures the "table" display mode, organizing system information into categories.
- **`neomimic`**: Configures the "neomimic" display mode, inspired by utilities like neofetch or fastfetch.

Here’s an default structure:

```json
{
  "global": {
    "raw_models": false,
    "simplify": false,
    "no_colors": false,
    "no_logo": false,
    "multicpu": false,
    "neomimic": false,
    "enable_versions": true
  },
  "table": {
    "categories": {
      "System": ["hostname", "username", "os", "device", "init", "terminal", "shell", "uptime"],
      "Processor": ["cpu"],
      "Memory": ["memory"],
      "Graphics": ["gpu", "display_server", "de", "wm", "brightness"],
      "Packages": ["packages"],
      "Disks": ["disks"],
      "Batteries": ["battery"]
    }
  },
  "neomimic": {
    "logo": "default",
    "modules": ["header", "separator", "os", "device", "kernel", "uptime", "packages", "shell", "de", "wm", "terminal", "cpu", "gpu", "memory"]
  }
}
```

## Global Section

The `"global"` section contains boolean flags (values must be `true` or `false`) that adjust SCU’s behavior. These flags correspond to optional launch arguments:

- **`raw_models`**: If `true`, shows unprocessed model names for hardware.
- **`simplify`**: If `true`, simplifies the output format.
- **`no_colors`**: If `true`, disables colored output.
- **`no_logo`**: If `true`, hides the logo.
- **`multicpu`**: If `true`, enables correct CPU information fetching on multi-CPU systems.
- **`neomimic`**: If `true`, activates the "neomimic" display mode by default.
- **`enable_versions`**: If `true`, includes version details for system components.

Example:

```json
"global": {
  "raw_models": false,
  "simplify": false,
  "no_colors": false,
  "no_logo": false,
  "multicpu": false,
  "neomimic": false,
  "enable_versions": true
}
```

## Table Section

The `"table"` section configures the table display mode. It contains a `"categories"` object where:

- **Keys** are category titles (e.g., `"System"`, `"Processor"`).
- **Values** are arrays of module names, each representing a piece of system information to display under that category.

In table mode, SCU organizes and presents information in a tabular layout based on these categories.

Example:

```json
"table": {
  "categories": {
    "System": ["hostname", "username", "os", "device", "init", "terminal", "shell", "uptime"],
    "Processor": ["cpu"],
    "Memory": ["memory"],
    "Graphics": ["gpu", "display_server", "de", "wm", "brightness"],
    "Packages": ["packages"],
    "Disks": ["disks"],
    "Batteries": ["battery"]
  }
}
```

## Neomimic Section

The `"neomimic"` section configures the neomimic display mode, which mimics utilities like neofetch or fastfetch by showing system info alongside a logo. It has two keys:

- **`logo`**:
  - `"default"`: Displays the default logo (a penguin named Tux, official mascot of the Linux kernel).
  - A full file path (e.g., `"/path/to/custom_logo.txt"`): SCU reads and displays a custom logo from this file.
- **`modules`**: An array of module names specifying which system information to show. Unlike the "table" mode, modules here aren’t grouped into categories.

Example:

```json
"neomimic": {
  "logo": "default",
  "modules": ["header", "separator", "os", "device", "kernel", "uptime", "packages", "shell", "de", "wm", "terminal", "cpu", "gpu", "memory"]
}
```

## Available Modules

SCU supports the following modules, which can be used in both "table" and "neomimic" modes. Each module fetches and displays specific system information:

- **`arch`**: System architecture.
- **`battery`**: Battery status and details.
- **`brightness`**: Screen brightness level.
- **`cpu`**: CPU information.
- **`de`**: Desktop environment.
- **`device`**: Device' name.
- **`disks`**: Disk usage and info.
- **`display_server`**: Display server (e.g., X11, Wayland).
- **`gpu`**: GPU information.
- **`header`**: A header in format "{username}@{hostname}".
- **`hostname`**: System hostname.
- **`init`**: Current init system.
- **`kernel`**: Kernel name and version.
- **`locale`**: Current system locale.
- **`memory`**: Information about installed RAM and SWAP space.
- **`os`**: OS name.
- **`packages`**: List of package managers and number of installed packages.
- **`rootfs`**: Filesystem used for root directory.
- **`separator`**: A separator line in the output.
- **`shell`**: Name and version of currently used shell.
- **`terminal`**: Name of currently used terminal emulator.
- **`uptime`**: System uptime.
- **`username`**: Current user.
- **`wm`**: Window manager.

## Example Configuration

Here’s the config example, fully configured:

```json
{
  "global": {
    "raw_models": false,
    "simplify": false,
    "no_colors": false,
    "no_logo": false,
    "multicpu": false,
    "neomimic": false,
    "enable_versions": true
  },
  "table": {
    "categories": {
      "System": ["hostname", "username", "os", "device", "init", "terminal", "shell", "uptime"],
      "Processor": ["cpu"],
      "Memory": ["memory"],
      "Graphics": ["gpu", "display_server", "de", "wm", "brightness"],
      "Packages": ["packages"],
      "Disks": ["disks"],
      "Batteries": ["battery"]
    }
  },
  "neomimic": {
    "logo": "default",
    "modules": ["header", "separator", "os", "device", "kernel", "uptime", "packages", "shell", "de", "wm", "terminal", "cpu", "gpu", "memory"]
  }
}
```

This configuration:
- Sets `"enable_versions": true` in "global" to show version info.
- Organizes "table" mode into categories like "System" and "Processor".
- Uses the default logo and a custom module list in "neomimic" mode.

## Using Custom Configurations

To run SCU with a specific configuration, use the `--config` parameter followed by:

- The configuration file name (e.g., `custom`), if it’s in `$HOME/.config/omnid/scu`.
- The absolute path to the file (e.g., `/path/to/custom.json`).

Examples:

```bash
scu --config custom
```

```bash
scu --config /path/to/custom.json
```
