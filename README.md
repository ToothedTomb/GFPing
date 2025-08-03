# GFPing
Ping like program but with random emojis on every ping. This program is used to test your internet and to see if you can access the network. So its a tool that will test if you are connect to the internet or not. 

# Screenshots of the program:
Running with internet:

![Screenshot From 2025-02-18 14-25-16](https://github.com/user-attachments/assets/db7c38c1-c6e8-416e-9c9b-00202d669bc7)

Running without internet:

![Screenshot From 2025-02-18 13-28-53](https://github.com/user-attachments/assets/28815d5c-71aa-4c95-9f6f-fc1b2878fe87)

# Useage:
gfping <hostname>

Example: gfping www.google.com

# Compile:
cargo build --release

# Make into a Debian package:
Install cargo-deb to begin with -  cargo install cargo-deb
Now go to the project folder and run - run cargo-deb
Now you have a Debian package. 
# How to install:
Go to /target/release and then use the install.sh script there or you can download the Debian package (only for Arm64 at the moment). 
# Credits:
Joanthan Steadman. :)
