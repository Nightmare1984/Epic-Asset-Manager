#!/bin/bash

# Set the Unreal Engine installation directory
UE_INSTALL_DIR=/path/to/unreal/engine/installation

# Set the plugin directory
PLUGIN_DIR=/path/to/plugin/directory

# Set the plugin name
PLUGIN_NAME=MyPlugin

# Set the build configuration (e.g. Debug, Development, Shipping)
BUILD_CONFIG=Development

# Set the architecture (e.g. x64, ARM64)
ARCH=x64

# Compile the plugin
$UE_INSTALL_DIR/Engine/Build/BatchFiles/Linux/Build.sh \
  -project=$PLUGIN_DIR/$PLUGIN_NAME.uproject \
  -plugin=$PLUGIN_DIR/$PLUGIN_NAME/Binaries/Linux/$PLUGIN_NAME.so \
  -build=$BUILD_CONFIG \
  -arch=$ARCH \
  -pak