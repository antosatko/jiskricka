import os
import shutil

BUILD_DIR = "build"
BINARY_NAME = "jiskricka.exe"


# build cargo project
os.system("cargo build --release")

# create build directory
if not os.path.exists(BUILD_DIR):
    os.makedirs(BUILD_DIR)

# copy the binary to the build directory
shutil.copy("target/release/{}".format(BINARY_NAME), BUILD_DIR)

# copy sfml dlls to the build directory
for file in os.listdir("SFML-2.6.1/bin"):
    shutil.copy("SFML-2.6.1/bin/{}".format(file), BUILD_DIR)


