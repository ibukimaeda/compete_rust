from tomlkit.toml_file import TOMLFile
import subprocess


toml = TOMLFile("./Cargo.toml")
toml_data = toml.read()
members = toml_data["workspace"]["members"]

exclude_member = ["mysnippet"]
for member in members:
    if member in exclude_member:
        continue

    subprocess.run(["cargo", "member", "rm", member])
