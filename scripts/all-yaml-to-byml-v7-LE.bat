:: Converts every file matching *.yaml to byml (v7 Little Endian)
:: 
:: Note: byml-to-yaml.exe must be present in the same folder in order for this to work

for /r %%i in (*.yaml) do byml-to-yaml.exe to-byml "%%~ni.yaml" -o "%%~ni.byml" -v "7" -e "little"
