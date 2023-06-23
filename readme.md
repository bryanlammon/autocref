# AutoCref <!-- omit in toc -->

**AutoCref's functionality has been integrated into [Supra](https://github.com/bryanlammon/supra). So it's no longer necessary or functioning. Use Supra instead.**

AutoCref is a [Supra](https://github.com/bryanlammon/supra) and [Pandoc](https://pandoc.org) post-processor that turns footnote cross-references in a Word document into automatically updated fields.

- [About](#about)
- [Usage](#usage)
  - [1. Unzip the .docx File](#1-unzip-the-docx-file)
  - [2. Run AutoCref Inside the Archive](#2-run-autocref-inside-the-archive)
  - [3. Zip the Files Into a .docx File](#3-zip-the-files-into-a-docx-file)
  - [Makefile](#makefile)
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

# After Pandoc creates a .docx file, AutoCref will change the numbers 1 and 2 into automatically updating numbers.
```

AutoCref recognizes two kinds of cross-references: (1) those to a single number, and (2) those in a range (*e.g.*, "notes 10–12").
AutoCref expects that ranges of notes will be separated by an en-dash.
But it will also recognize a hyphen.
There should be no spaces before or after the en-dash (or hyphen).
Sets of cross-references (*e.g.*, "notes 10 & 12") are not supported at this time.

## Usage

Using AutoCref involves three steps.
First you must unzip the `.docx` file.
Then you run AutoCref.
Then you re-zip the `.docx` file.

### 1. Unzip the .docx File

First, the `.docx` file must be unzipped into its own directory.

```zsh
unzip example.docx -d example-contents
```

### 2. Run AutoCref Inside the Archive

Move to the directory into which you unzipped the `.docx` file and run AutoCref.

```zsh
cd example-contents
autocref
```

By default, AutoCref will alter the `document.xml` and `footnotes.xml` in the `./word/` subdirectory.
If for some reason you want to set different files, you can set the `document.xml` and `footnotes.xml` files by providing filenames after `autocref`.
The `document.xml` filename must come first.
The `footnotes.xml` filename must come second.

### 3. Zip the Files Into a .docx File

Then re-zip the files in that directory into a .docx file.

```zsh
zip -r example.docx *
```

`zip` should be run inside the directory into which you unzipped the `.docx` contents.
After re-zipping, move the new `.docx` file to wherever you want to keep it.
You can then delete the directory containing the `.docx` contents.

### Makefile

Probably the easiest way to use AutoCref is adding it to the Makefile used for Supra and Pandoc.
The following lines can come after those for Pandoc:

```Makefile
unzip example.docx -d autocref-temp; \
cd autocref-temp; \
autocref; \
zip -r example.docx *; \
mv example.docx ../; \
cd ..; \
rm -r autocref-temp
```

## Changelog

* 0.1.0: Initial release
