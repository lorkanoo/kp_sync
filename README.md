# KP Sync
Kill proof synchronizer for Nexus.

## Features
- Automatically refresh kp.me when exiting raid / strike map.
- Reattempt refresh on schedule if refresh failed due to KP refresh rate limit.
- Reattempt refresh on Guild Wars 2 start if game was closed before scheduled refresh succeeded.
- Define custom map ids that will trigger refresh (in config file).

## Installation
1. Install the [Nexus](https://github.com/RaidcoreGG/Nexus) addon manager ([website](https://raidcore.gg/Nexus)).
2. Download [`kp_sync.dll`](../../releases/latest) and place it in your `addons` folder (e.g. `C:\Program Files\Guild Wars 2\addons`).
3. Open the Nexus addon window, click on the refresh button if necessary and load KP Sync.
4. Enter your Killproof ID or account name in addon settings.

## Known issues
- Addon might show success message when non-existing kp id is provided.

## Roadmap
- Automatically get account name from arcdps event if not configured and arcdps is installed.