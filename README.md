# Survon Module Contribution Guidelines

Survon is an offline, off-grid survival system built on modular principles. To ensure consistency, reliability, and ease of integration, every module must adhere to the following guidelines. This document outlines how modules are categorized, how they must be packaged, and the contribution standards for the community.

---

## 1. Module Categories and Naming Conventions

Modules are organized into distinct categories. Each category has a unique prefix that must be used in the module’s name. This ensures clarity and consistency across the ecosystem.

**Categories:**

- **Communication (com):**  
  Modules for communication systems.  
  *Examples:*  
  - `mod-rust-com--morse-code`
  - `mod-rust-com--irc`

- **Monitoring (mon):**  
  Modules for monitoring systems and sensors.  
  *Examples:*  
  - `mod-rust-mon--temperature-sensor`
  - `mod-rust-mon--camera-feed`

- **Defense (def):**  
  Modules for defensive systems.  
  *Examples:*  
  - `mod-rust-def--manual-turret`
  - `mod-rust-def--motion-turret`

- **Power (pow):**  
  Modules for managing power systems.  
  *Examples:*  
  - `mod-rust-pow--solar-array`
  - `mod-rust-pow--battery-monitor`

- **Weather (wth):**  
  Modules for weather tracking and forecasting.  
  *Examples:*  
  - `mod-rust-wth--storm-alert`
  - `mod-rust-wth--rain-gauge`

- **Library (lib):**  
  Knowledge and computational modules.  
  *Examples:*  
  - `mod-rust-lib--pubmed-library`
  - `mod-rust-lib--math-utils`

- **Agriculture (agr):**  
  Modules for farming and food production.  
  *Examples:*  
  - `mod-rust-agr--soil-monitor`
  - `mod-rust-agr--irrigation-control`

- **Entertainment (ent):**  
  Modules for recreational activities.  
  *Examples:*  
  - `mod-rust-ent--text-adventure`
  - `mod-rust-ent--media-player`

- **Crafting (crf):**  
  Modules for creating or repairing items.  
  *Examples:*  
  - `mod-rust-crf--blueprint-helper`
  - `mod-rust-crf--tool-fabricator`

- **Electronics (ele):**  
  Modules for electronics design and control.  
  *Examples:*  
  - `mod-rust-ele--circuit-simulator`
  - `mod-rust-ele--signal-generator`

- **Sensors (sen):**  
  Modules for sensor integration.  
  *Examples:*  
  - `mod-rust-sen--motion-detector`
  - `mod-rust-sen--proximity-sensor`

- **Traps (trp):**  
  Modules for trapping and detection systems.  
  *Examples:*  
  - `mod-rust-trp--animal-trap`
  - `mod-rust-trp--intruder-alarm`

*Note:* The module name should follow the format `mod-rust-<category_prefix>--<module_name>`, ensuring that each module is clearly identified by its function.

---

## 2. Packaging Requirements

Each module must be delivered as a single ZIP file that follows this exact structure at the root:

```
module-example.zip
├── meta.json
├── mod.rs (or module.rs)
├── [Optional files: README.md, LICENSE, tests/]
```

**File Descriptions:**

- **meta.json:**  
  This file contains metadata that validates the module. It must include, at minimum:
  ```json
  {
    "name": "Module Example",
    "lib_file": "libmodule_example.so",
    "version": "1.0.0"
  }
  ```
- `name`: The unique module name (should match the naming convention above).
- `lib_file`: The filename of the dynamic library produced by compiling the module (for Linux, a `.so` file).
- `version`: The module’s version following semantic versioning.

- **Module Source Code:**  
  The module’s code (typically in a file named `mod.rs` or `module.rs`) must implement the Survon module API.

- **Optional Files:**  
  Any additional documentation (e.g., `README.md`), licensing information (e.g., `LICENSE`), or tests (inside a `tests/` directory).

*Important:* Ensure that **meta.json** and the dynamic library (once compiled) are at the root of the ZIP file—not nested inside an extra top-level folder. On macOS, this means you should select the files inside your module package directory when compressing, rather than the folder itself.

---

## 3. Building the Module

Your module’s Cargo project must be configured to compile as a dynamic library. Ensure your `Cargo.toml` includes:

```toml
[lib]
crate-type = ["cdylib"]
```

For cross-compilation to produce a Linux shared object (`.so` file) on a non-Linux system, add a `.cargo/config.toml` file in your project root with:

```toml
[target.aarch64-unknown-linux-musl]
linker = "aarch64-unknown-linux-musl-gcc"
```

Then build with:

```bash
cargo build --release --target=aarch64-unknown-linux-musl
```

The resulting `.so` file (e.g., `libmodule_example.so`) will be located in `target/aarch64-unknown-linux-musl/release/`.

---

## 4. Packaging Workflow

We provide a Makefile target to automate packaging. In your module project, include a Makefile with a target similar to:

```makefile
.PHONY: package

# Package the module into a ZIP file. Assumes your module package files are in the "package" directory.
package:
	@echo "Packaging module..."
	cd package && zip -r ../module-example.zip . -x "*.DS_Store"
	@echo "Module packaged as module-example.zip"
```

Place your compiled dynamic library, meta.json, and other necessary files directly inside the `package/` directory, then run:

```bash
make package
```

Verify the ZIP structure with:

```bash
unzip -l module-example.zip
```

---

## 5. Testing with Survon

1. **Deploy Your Module:**  
   Place your `module-example.zip` file into the Survon module directory (`/tmp/wasteland`) on the target system. For development using Docker, mount your local module directory (e.g., `./tmp/wasteland`) to `/tmp/wasteland` in the container.

2. **Run Survon:**  
   When the Survon runtime starts, it will scan `/tmp/wasteland`, extract each ZIP file, read the metadata from **meta.json**, and load the module's dynamic library.

3. **Verify:**  
   Check the Survon logs or UI to ensure that your module is detected and loaded correctly.

---

## 6. Community Contribution Guidelines

All modules contributed to the Survon ecosystem must adhere to these standards:

- **Module Naming:** Follow the established category prefixes and naming format.
- **Packaging:** Use the ZIP file structure described above.
- **Versioning:** Follow semantic versioning.
- **Testing:** Ensure that your module passes all required tests before contribution.
- **Documentation:** Include a README.md in your package that describes:
    - The module’s purpose and functionality.
    - Installation instructions.
    - Usage examples.

By following these guidelines, you help ensure that all modules are consistent, reliable, and easy for Survon to integrate and load.

---

## 7. Example Directory Structure

For a module project named `mod-rust-mon--temperature-sensor`, your repository might look like this:

```
mod-rust-mon--temperature-sensor/
├── Cargo.toml
├── src/
│   └── lib.rs         # Implements the Survon module API
├── package/           # Directory for packaging files
│   ├── meta.json      # Metadata file (not manifest.json in this case)
│   ├── libmod_rust_mon--temperature_sensor.so
│   └── README.md      # Module documentation
├── .cargo/
│   └── config.toml    # (If cross-compiling, see instructions above)
└── Makefile           # Contains a target to package the module
```

---

## 8. Final Notes

- **Ensure Consistency:**  
  The filenames in **meta.json** must exactly match the generated dynamic library filenames.
- **Avoid Extra Folders:**  
  When compressing your package directory, make sure the files are at the root of the ZIP file.
- **Testing:**  
  Always test your module package by extracting it locally and verifying that the structure meets the guidelines.

Happy module building and thank you for contributing to the Survon ecosystem!
