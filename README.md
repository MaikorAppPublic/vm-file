# Maikor

>⚠️ Work in progress
>
> Links may be broken, features missing, etc

*Cross platform 16 bit game system*

See more at [maikor.app](https://maikor.app)

### Play

[Android](https://github.com/MaikorAppPublic/android-app)

[iOS](https://github.com/MaikorAppPublic/ios-app)

[Windows, macOS and Linux](https://github.com/MaikorAppPublic/desktop-app)

### Make

[iOS IDE](https://github.com/MaikorAppPublic/ios-app)

[Desktop IDE](https://github.com/MaikorAppPublic/desktop-ide)

[Build tools](https://github.com/MaikorAppPublic/build-tools)

### Project breakdown

#### Major
* [vm-core](https://github.com/MaikorAppPublic/vm-core)
  * Executes Maikor games
* [desktop-app](https://github.com/MaikorAppPublic/desktop-app)
  * Host program for Windows, macOS and Linux
* [android-app](https://github.com/MaikorAppPublic/android-app)
  * Host program for Android
* [ios-app](https://github.com/MaikorAppPublic/ios-app)
  * Host program for iOS
* [desktop-ide](https://github.com/MaikorAppPublic/desktop-ide)
  * IDE program for Windows, macOS and Linux

#### Minor
* [vm-interface](https://github.com/MaikorAppPublic/vm-interface)
  * Acts as hardware simulation layer for the VM
* [vm-interface-android](https://github.com/MaikorAppPublic/vm-interface-android)
  * Android compatible wrapper for `vm-interface`
* [vm-interface-ios](https://github.com/MaikorAppPublic/vm-interface-ios)
  * iOS compatible wrapper for `vm-interface`
* [vm-desktop-simple](https://github.com/MaikorAppPublic/vm-desktop-simple)
  * Simple desktop program for testing Maikor games (it can't save, etc)
* [vm-file](https://github.com/MaikorAppPublic/vm-file)
  * For reading and writing Maikor game files
* [maikor-language](https://github.com/MaikorAppPublic/language)
  * OP names, memory addresses and platform requirements

## vm-file

This library can be used to read and write Maikor game files.

| Struct          | Use        | Min Size | Max Size |
|-----------------|------------|----------|----------|
| GameFileSummary | Summary    | 22B      | 790B     |
| GameFile        | Whole file | 9KB      | 6MB      |

`GameFileSummary` has the game name, version and ID

### Usage

```
GameFileSummary::read(file_path);
//or
GameFile::read(file_path);
```