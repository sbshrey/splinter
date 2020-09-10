% SCABBARD-SP(1) Cargill, Incorporated | Splinter Commands
<!--
  Copyright 2018-2020 Cargill Incorporated
  Licensed under Creative Commons Attribution 4.0 International License
  https://creativecommons.org/licenses/by/4.0/
-->

NAME
====

**scabbard-sp** — Provides management of Sabre smart permissions.

SYNOPSIS
========

**scabbard ns** \[**FLAGS**\] \[**SUBCOMMAND**\]

DESCRIPTION
===========
This command provides management functionality for the Sabre smart permissions
of a scabbard service.

FLAGS
=====
`-h`, `--help`
: Prints help information.

`-v`
: Increases verbosity. Specify multiple times for more output.

SUBCOMMANDS
===========
`create`
: Creates a smart permission in a scabbard service's state.

`delete`
: Deletes a smart permission from a scabbard service's state.

`update`
: Updates the an existing smart permission in a scabbard service's state.

SEE ALSO
========
| `scabbard-ns-create(1)`
| `scabbard-ns-delete(1)`
| `scabbard-ns-update(1)`
|
| Splinter documentation: https://www.splinter.dev/docs/
