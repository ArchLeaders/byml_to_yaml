# Byml to Yaml (RS)

Simple BYML to YAML converter written in rust using the [roead](https://github.com/NiceneNerd/roead) library by [NiceneNerd](https://github.com/NiceneNerd) with BYML v7 support.

## Scripts (Windows Only)

In the [scripts](/scripts) folder there are two batch files, one to convert every byml file to yaml in a folder, and one to convert every yaml file to byml.

**Note:** YAML files will be converted to **v7 little endian** byml files. To change this, edit the `yaml-yo-byml` batch file and update the `-v` and `-e` flags at the end.
