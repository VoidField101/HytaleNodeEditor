# Open Hytale NodeEditor [WIP]

> **Work in Progress & Not Working** 
> 
> This Project is in very early stages of development. The project itself **DOES NOT WORK** at the moment and is missing key components.
> 
> Please **DO NOT USE** it or expect it to work for anything yet


This is a open source reimplementation of the [Hytale](https://hytale.com/) NodeEditor based on egui.


This reimplementation aims to be compatible with Linux (and hopefully also MacOS) as the original one is only woring on Windows and also doesn't work with Wine/Proton. 

It also should be as compatible as possible with the original Editor both in terms of user experience and file formats.

## How to run


### 1. Optain the NodeEditor Workspace files
Before you start you need the original NodeEditor because it contains the workspace files which tell the editor which modes and nodes exist.

You find this **on Windows** ``%APPDATA%\Hytale\install\release\package\game\latest\Client\NodeEditor`` on a Windows machine.

If you want to use Bottles **on Linux** do the following:
 1. Create a new Bottle
 2. Select ``Gaming`` and set the runner to ``ge-proton``
 3. Go to ``Dependencies`` and install ``webview2``
 4. [Download](https://hytale.com/download) Hytale for Windows
 5. Run the installer using ``Launch Executable``
 6. When the launcher has started install the launcher
 7. Close the game and go to ``Browse C:/ drive``
 8. Go to ``users/steamuser/AppData/Roaming/Hytale/install/release/package/game/latest/Client/NodeEditor`` (This is virtually the same path as mentioned for Windows above)

 **Note**: Sometimes instead of "steamuser" your local username will be used. You may have to check other directories inside users instead (it sould never be in "Public").


 Whatever method you've used to optain the NodeEditor you need to copy ``Workspaces`` into the directory of this project (Same location as the README) and rename it to ``hytale_workspaces``

### 2. Compile & Run

 To compile and run the project you need to have [Rust](https://rust-lang.org/) installed and run the following command:
 ```
 cargo run --release
 ```

 or to just compile it
 ```
 cargo build --release
 ```
 in which case you will find the executable in ``target/release/HyNodeEditor``

 **Note:** The editor will look for ``hytale_workspaces`` in the current working directory per default!