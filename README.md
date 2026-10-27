# elfinstaller

A simple terminal application to install and manage ELF packages.

## Usage

Install an ELF package:
```sh
elfinstaller --install elfName --rename newName
```

Uninstall an ELF package:
```sh
elfinstaller --uninstall newName
```

Install an ELF package with default name:
```sh
elfinstaller --install elfName
```

Check version:
```sh
elfinstaller --version
```

Show help:
```sh
elfinstaller --help
```

## Features
- Install ELF packages from a predefined repository.
- Rename installed packages.
- Uninstall packages.
- Simple CLI interface.

## Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/andy64lol/elfinstaller.git
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Install the `elfinstaller` binary to `/usr/local/bin` (requires `sudo`):
   ```sh
   sudo cp target/release/elfinstaller /usr/local/bin/elfinstaller
   ```

4. Ensure the binary is executable (requires `sudo`):
   ```sh
   sudo chmod +x /usr/local/bin/elfinstaller
   ```

### Arch Linux package

On Arch Linux, install `base-devel` and `rust`, then build a pacman package from
the current checkout:

```sh
make arch-package
sudo pacman -U dist/elfinstaller-*.pkg.tar.zst
```

The package installs `elfinstaller` to `/usr/bin`. Remove it with
`sudo pacman -R elfinstaller`.

### Notes
- **`sudo` is required** for installing files to `/usr/local/bin`.
- Ensure you have the necessary permissions to modify system directories.

### Installing ELF Packages

After installing `elfinstaller`, you can install ELF packages directly into `/usr/local/bin`:

```sh
elfinstaller --install myelf --rename mycustomname
```

**Note:** `sudo` is required to install ELF packages to `/usr/local/bin`.

This will create an executable file at `/usr/local/bin/mycustomname_v0.1.0`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
