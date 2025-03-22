# Configuration File

The configuration file for Sherlock is located at `~/.config/sherlock/config.toml`, unless specified otherwise. This file allows you to customize various parameters to tailor Sherlock to your needs. Below, we will explore the available options and their purposes.

---

## Default App Section `[default_apps]`

| **Keyword**       | **Default**          | **Explanation**                                                                                                                  |
|-------------------|----------------------|-------------------------------------------------------------------------------------------------------------------------------|
| `terminal`        | Automatically detected | May be required if the `TERMINAL` environment variable is not set. Specify the executable name of your terminal (e.g., `"gnome-terminal"`, `"konsole"`). |

---
## Debug Section `[debug]`

| **Keyword**           | **Default** | **Explanation**                                                                 |
|-----------------------|-------------|---------------------------------------------------------------------------------|
| `try_suppress_errors` | `false`     | If set to `true`, errors and warnings will not be displayed when starting the app. |
| `try_suppress_warnings` | `false`   | If set to `true`, only errors will trigger the error screen at startup, while warnings will be ignored. |

---

## Appearance Section `[appearance]`

| **Keyword**     | **Default** | **Explanation**                                                                                                                 |
|-----------------|-------------|-------------------------------------------------------------------------------------------------------------------------------|
| `width`    | `900`        | Sets the width of the main window.|
| `height`    | `593`        | Sets the height of the main window. | 
| `gsk_renderer`  | `"cairo"`   | Specifies the renderer used to display the Sherlock application. During testing, `cairo` showed the fastest startup times. You can also use `ngl`, `gl`, or `vulkan` based on your system's performance. |
| `recolor_icons` | `false`     | Appends the `-symbolic` postfix to all icons, allowing them to be colorized. Note: not all icons have a symbolic version.       |
| `icon_paths`    | `[]`        | Defines custom paths for the application to search for icons. This is useful for adding custom icons for commands or aliases through the `sherlockalias` file. |
| `icon_size`    | `22`        | Sets the default icon size for the icons in each tile. |

---

