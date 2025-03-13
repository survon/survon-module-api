# Survon Module Packaging Guidelines

This document specifies the packaging requirements for modules to be loaded by the Survon runtime. All modules must adhere to these guidelines so that they can be detected, extracted, and loaded automatically.

## 1. Module Package Structure

Each module must be delivered as a ZIP file with the following structure at the **root** of the archive (i.e., no extra top-level folder):

```
module-example.zip
├── manifest.json
└── libmodule_example.so
```

- **manifest.json:**  
  A JSON file providing the module's metadata.
- **libmodule_example.so:**  
  The compiled dynamic library for your module.

> **Important:**  
> When extracted, the ZIP file must show `manifest.json` and your dynamic library directly in the extraction directory.

## 2. manifest.json Requirements

Your `manifest.json` file must be a valid JSON file and include at least the following keys:

- `name`: The unique name (and namespace) of the module (string).
- `lib_file`: The filename of the dynamic library (string).

**Example:**

```json
{
  "name": "Module Example",
  "lib_file": "libmodule_example.so"
}
```

Ensure the value for `lib_file` exactly matches the filename of your dynamic library.

## 3. Building Your Module as a Dynamic Library

To compile your module as a dynamic library that works with Survon, your Cargo project must be configured appropriately. In your module’s `Cargo.toml`, include the following configuration:

```toml
[lib]
crate-type = ["cdylib"]

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-musl-gcc"
```

This setting tells Cargo to compile your project as a dynamic library (a shared object) suitable for linking with other programs. Then, simply run:

```bash
cargo build --release
```

The resulting dynamic library will be located in the `target/release` directory (e.g., as `libmodule_example.so` on Linux).

## 4. Packaging Your Module

1. **Prepare Your Package Directory:**  
   Create a folder (e.g., `module_package/`) in your project root and place the following files at its root:
    - Your compiled dynamic library (e.g., `libmodule_example.so`)
    - `manifest.json`

2. **Create the ZIP Archive:**  
   To package the module so that its contents are at the root of the archive, navigate into the package directory and run:

   ```bash
   cd module_package
   zip -r ../module-example.zip . -x "*.DS_Store"
   cd ..
   ```

   This will create `module-example.zip` in your project root, with the correct structure.

3. **Verify the ZIP Structure:**  
   Check the contents with:

   ```bash
   unzip -l module-example.zip
   ```

   Ensure that `manifest.json` and your `.so` file appear at the top level, without an extra directory.

## 5. Testing Your Module with Survon

1. **Place the ZIP File:**  
   Copy your module ZIP file (`module-example.zip`) into the module directory used by Survon (typically `/tmp/wasteland`).

2. **Run Survon:**  
   Start the Survon runtime. It will scan `/tmp/wasteland`, extract the ZIP file, read the manifest, and load your module.

3. **Verify the Log Output:**  
   Confirm that the runtime logs indicate your module was successfully detected and loaded.

## 6. Example Workflow Summary

1. **Develop Your Module:**
    - Implement your module using the Survon module API.
    - Ensure your `Cargo.toml` includes:
      ```toml
      [lib]
      crate-type = ["cdylib"]
      ```
    - Run `cargo build --release` to compile your dynamic library.

2. **Prepare Your Package Directory:**  
   Place `manifest.json` and your dynamic library (e.g., `libmodule_example.so`) in a folder (e.g., `module_package/`).

3. **Package the Module:**
   ```bash
   cd module_package
   zip -r ../module-example.zip . -x "*.DS_Store"
   cd ..
   ```

4. **Test the ZIP:**
   ```bash
   unzip -l module-example.zip
   ```

5. **Deploy with Survon:**  
   Place `module-example.zip` in `/tmp/wasteland` on your Survon runtime and start the system.

By following these guidelines, you ensure that your module will be compatible with the Survon runtime and that the open source community has a clear standard to follow. Happy module building!
