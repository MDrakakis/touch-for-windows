# touch-for-windows
Linux touch command for windows implemented in rust. (As learning rust mini project)

![Image of Rust](https://www.rust-lang.org/logos/rust-logo-128x128.png)

## Installation
1. Download Zip or Clone the repository https://github.com/MDrakakis/touch-for-windows.git.
2. Build the project to generete the .exe file and place it on desired location. (Ex. C:/touch).
3. Open the run window and type: rundll32.exe sysdm.cpl,EditEnvironmentVariables (this will open the Enviromental Variables Window or open it manualy).
4. On user variables edit the Path Variable and press Browse.
5. Navigate to the folder where you placed the .exe from step 2 (Ex. C:/touch) press Ok.
6. Restart

## Usage
1. Open terminal and navigate to desired directory.
2. Type
```shell
touch my_new_file.txt
```
