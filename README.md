# grocery_list_generator

An application to generate 3 formatted grocery lists from a single input file. The three are:
1. The master list for Matt.
1. Half of the items for Max.
1. The other half of the items for Miles.

## Introduction

This is a custom application which performs a very specific task, for a very picky customer set - my sons Max and Miles. We go grocery shopping and we split up the list between the to of them, and they read their list and collect their goods. The master list (mine) serves as a check on this.

The output from this application's companion application, [emeals_getter](https://github.com/mattcaron/emeals_getter) can be used as input for this.

## Usage

1. Make a list of groceries, one entry per line.
1. Run `grocery_list_generator <listname>`.
1. The `.tex` will be created with the same stem as `<listname>` above (so, `groceries.txt` becomes `groceries.tex`).
1. Generate the PDF from that `.tex` file. (See [my LaTeX scripts](https://github.com/mattcaron/latex_scripts) for help with this.)

## TODO

- Add test code.
- There is much copypasta tech debt that needs refactoring.
- Add a license file.