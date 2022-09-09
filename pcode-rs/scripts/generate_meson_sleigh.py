"""Script to generate the "AUTOMATICALLY GENERATED" section of meson.build.

To prevent issues, that part of the build file is not automatically inserted, just
output by this script to stdout. The output should be manually inserted into the
build file.
"""

from json import dumps
from pathlib import Path
from re import search, sub

SOURCE_ROOT = Path(__file__).parents[1]
GHIDRA_PROCESSORS_DIR = SOURCE_ROOT / "ghidra" / "Ghidra" / "Processors"

DISABLED_PROCESSORS = ("V850",)


def main() -> None:
    """Iterate through each processor and assemble the list of non-slaspec sources,
    then assemble the list of slaspec targets and output both to stdout.
    """

    sleigh_processor_sources = {}
    sleigh_processor_specs = {}

    for processor_dir, language_dir in map(
        lambda d: (d, d / "data" / "languages"), GHIDRA_PROCESSORS_DIR.iterdir()
    ):
        if not language_dir.is_dir() or processor_dir.name in DISABLED_PROCESSORS:
            continue

        sleigh_processor_sources[processor_dir.name] = list(
            set(
                map(
                    lambda x: x.name,
                    filter(
                        lambda x: x.suffix not in (".sla", ".slaspec") and x.is_file(),
                        language_dir.iterdir(),
                    ),
                )
            )
        )

        sleigh_processor_specs[processor_dir.name] = {}

        for spec_file in language_dir.glob("*.slaspec"):
            sleigh_processor_specs[processor_dir.name][
                spec_file.stem + ".sla"
            ] = spec_file.name

    str_specs = "sleigh_processor_specs = " + dumps(
        sleigh_processor_specs, indent=4
    ).replace('"', "'")

    str_sources = "sleigh_processor_sources = " + dumps(
        sleigh_processor_sources, indent=4
    ).replace('"', "'")

    fixup_str_specs = []
    cur_proc = ""
    for line in str_specs.splitlines():
        mtch = search(r"^\s*'([^']+)': '([^']+)',?$", line)

        if mtch:
            fixup_str_specs.append(
                sub(
                    r"^\s*('[^']+'):\s*('[^']+'),?$",
                    lambda m: f"        {m.group(1)}: [{m.group(2)}] + sleigh_processor_sources.get('{cur_proc}'),",
                    line,
                )
            )
            continue

        mtch = search(r"^\s*'([^']+)': \{", line)
        if mtch:
            cur_proc = mtch.group(1)

        fixup_str_specs.append(line)

    print(str_sources)
    print("\n")
    print("\n".join(fixup_str_specs))


if __name__ == "__main__":
    main()
