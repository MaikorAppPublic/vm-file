# Maikor

>⚠️ Work in progress
>
> Links may be broken, features missing, etc

*Cross platform 16 bit game system*

See more at [maikor.app](https://maikor.app) and the [project homepage](https://github.com/MaikorAppPublic)

### Play

[Android](https://github.com/MaikorAppPublic/android-app)

[iOS](https://github.com/MaikorAppPublic/ios-app)

[Windows, macOS and Linux](https://github.com/MaikorAppPublic/desktop-app)

### Make

[iOS IDE](https://github.com/MaikorAppPublic/ios-app)

[Desktop IDE](https://github.com/MaikorAppPublic/desktop-ide)

[Build tools](https://github.com/MaikorAppPublic/build-tools)

[REPL](https://play.vm.maikor.app)
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