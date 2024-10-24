<p align="center" dir="auto">
    <img style="height:240px;width:240px"  src="https://s2.loli.net/2024/09/08/uDKESYW7ks9eRyf.png" alt="Logo逃走啦~"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">GXCI</h1>

<p align="center">
  <a href="https://crates.io/crates/gxci" target="_blank"><img src="https://img.shields.io/crates/v/gxci"/></a>
  <a href="https://docs.rs/gxci" target="_blank"><img src="https://img.shields.io/docsrs/gxci/0.3.4"/></a>
  <a href="https://github.com/zoneherobrine/gxci" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
</p>

<p align="center">
    Rust-based safe interface development for Daheng Industrial Camera GxIAPI
</p>

<hr />

# New Things in 0.3
1. [x] CHECK:   Check module for COMMON error handling
2. [x] CONFIG:  Full HAL and Raw-binding config module
   - Some FeatureID are missing, so the config module has a few functions are not implemented now.
3. [x] CONTROL: Commonly used part of control module (Based on the Galaxy Viewer's sidebar)
4. [x] (in 0.3.2) gxi_use_stream() allows you to use the custom stream callback function to process the image data. You can see the usage in the hal_use_stream example.
5. [x] (in 0.3.3) re-exported opencv and imageproc.
6. [x] (in 0.3.4) gxi_get_image_as_frame_data(), gxi_get_image_as_raw() and gxi_get_image_as_bytes() provide interfaces to use the image data, and with examples

# Quick Start
1. Ensure you have OpenCV Rust Bindings installed, if not, you can see the [crates page's README](https://crates.io/crates/gxci)
2. Ensure your camera version is supported by the GxIAPI SDK,and ensure you have installed the GxIAPI SDK.

# HAL Part
There five main modules in the HAL: base, device, config, event and network. 

But until 0.3, the event and network module are not implemented.

# RAW Part
The RAW part in GXCI is all-safety, with LazyLock-Arc-Mutex-Option. And you can find the unsafe static mut implementation in the precious lib called [gxi_hako](https://crates.io/crates/gxi_hako), which is a deprecated RAW-only version of GXCI.

# Utils Part
Just the builder pattern and facade pattern, they are friendly to debug.

