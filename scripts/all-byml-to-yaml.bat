:: Converts every file ending with *.byml to yaml in the current folder
::
:: Note: byml-to-yaml.exe must be present in the folder in order for this to work

for /r %%i in (*.byml) do byml-to-yaml.exe to-yaml "%%~ni.byml" -o "%%~ni.yaml"
