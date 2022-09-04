# AutoCref <!-- omit in toc -->

AutoCref is a [Supra](https://github.com/bryanlammon/supra) and [Pandoc](https://pandoc.org) post-processor that turns footnote cross-references in a Word document into automatically updated fields.

- [About](#about)
- [Usage](#usage)
- [Changelog](#changelog)

## About

AutoCref is a [Supra](https://github.com/bryanlammon/supra) and [Pandoc](https://pandoc.org) post-processor that turns footnote cross-references in a Word document into automatically updated fields.

`.docx` files are a zip file of mostly `.xml` files.
AutoCref adds markup to two of those files—`document.xml` and `footnotes.xml`—which turns the Supra-produced footnote cross-references into automatically updated fields.

The program works with cross-references that consist of an intalicized "*supra*" or "*infra*" immediately followed by a space and the word "note" or "notes".
The cross-references produced by Supra should work fine.
Manual cross-references (such as those referring the reader to other notes where something is discussed) are less reliable.
If there is anything besides a space between the italicized "*supra*" or "*infra*" and the word "note" or "notes", AutoCref will not change the number to a field.

So, for example:

```Markup
# A document with the following Supra markup...
Some text.^[*See* [@jones2001] at 100.]
Some more text.^[*See* *supra* notes [?id1]–[?id2].]

# ... will render as the following plain text:
Some text.^[*See* Jones, *supra* note 1, at 100.]
Some more text.^[*See* *supra* notes 1–2.]

# After Pandoc creates a .docx file, AutoCref will change the numbers 1 and 2
# into automatically updating numbers.
```

AutoCref recognizes two kinds of cross-references: (1) those to a single number, and (2) those in a range (*e.g.*, "notes 10–12").
AutoCref expects that ranges of notes will be separated by an en-dash.
But it will also recognize a hyphen.
There should be no spaces before or after the en-dash (or hyphen).
Sets of cross-references (*e.g.*, "notes 10 & 12") are not supported at this time.

## Usage

AutoCref requires an input `.docx` file.
You can follow that with an optional output `.docx` file.
If no output is provided, AutoCref will overwrite the input.

```zsh
# Overwriting the input
autocref input.docx

# Setting an output
autocref input.docx output.docx
```

Probably the easiest way to use AutoCref is adding it to the Makefile used for Supra and Pandoc.

## Changelog

* 0.1.0: Initial release
* 0.2.0: Runs on `.docx` files directly
