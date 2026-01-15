# dfraw_json_parser

[![supports df 50.xx](https://img.shields.io/badge/Supports%20Dwarf%20Fortress-0.50.xx-%235E3E0D?style=plastic)](https://bay12games.com/dwarves/)
[![Clippy Check](https://github.com/nwesterhausen/dfraw_json_parser/actions/workflows/clippy.yml/badge.svg)](https://github.com/nwesterhausen/dfraw_json_parser/actions/workflows/clippy.yml)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/nwesterhausen/dfraw_json_parser/badge)](https://securityscorecards.dev/viewer/?uri=github.com/nwesterhausen/dfraw_json_parser)

A solution to present information about what raws are in your save game, in a searchable format.

I made this because I was playing with [Splint's Vanilla Expanded Mod](http://www.bay12forums.com/smf/index.php?topic=177593.0)
and [Primal](http://www.bay12forums.com/smf/index.php?topic=172869.15) and at the prepare carefully
screen I had no way to figure out what some of the animals were.

Yes there is the [raw explorer](http://www.bay12forums.com/smf/index.php?topic=103360) program but I found
that difficult to search with and the information was displayed in basically the same format as the raw file
itself, so it was hard to read.

## Current Functionality

Creates JSON from Dwarf Fortress raw files. This JSON can be used with the small web client to search
through, be plopped into Algolia and search or, you could simply CTRL+F or grep for what you are looking
for in the file itself. I find the JSON easier to read than the RAW txt files, and it currently doesn't
include a lot of items that were not important to me when looking up creatures. I was most concerned with
the description of the animal, if they laid eggs, if they were milkable, and how big they were.

- Parses raw files for creatures
- Parses raw files for plants

## Documentation

- **[API Documentation](https://docs.rs/dfraw_parser)** - Generated documentation for all public APIs
- **[Documentation Improvements](DOCUMENTATION_IMPROVEMENTS.md)** - Detailed analysis of documentation status and improvement suggestions
- **[Quick Reference](DOCUMENTATION_QUICK_REFERENCE.md)** - Quick reference for documenting code

## Contributing

Contributions are welcome! Please see:

- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Guidelines for contributing, including documentation standards
- **[DOCUMENTATION_SUMMARY.md](DOCUMENTATION_SUMMARY.md)** - Overview of documentation resources and current status

To check for missing documentation:
```bash
cargo clippy --package dfraw_parser -- -D missing_docs
```
