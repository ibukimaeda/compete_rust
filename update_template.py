from tomlkit.toml_file import TOMLFile

with open("src/template.rs", "r") as src_file:
    src_content = src_file.read()

for dir in ["contest", "virtual_contest"]:
    toml = TOMLFile(f"./{dir}/compete.toml")
    toml_data = toml.read()
    template = toml_data.get("template")
    template["src"] = f"""{src_content}\n"""

    toml.write(toml_data)

