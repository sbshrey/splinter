% SCABBARD-SP-CREATE(1) Cargill, Incorporated | Splinter Commands
<!--
  Copyright 2018-2020 Cargill Incorporated
  Licensed under Creative Commons Attribution 4.0 International License
  https://creativecommons.org/licenses/by/4.0/
-->

NAME
====

**scabbard-sp-create** — Creates a Sabre smart permission.

SYNOPSIS
========

**scabbard sp create** \[**FLAGS**\] \[**OPTIONS**\] ORG_ID NAME

DESCRIPTION
===========
This command allows users to create a new Sabre smart permission in state for
the targeted scabbard service. Smart permissions are business logic implemented
in a programming language, stored in state, and executed within a smart
contract. The purpose of smart permissions is to enable complex permissions for
transaction execution.

FLAGS
=====
`-h`, `--help`
: Prints help information.

`-v`
: Increases verbosity. Specify multiple times for more output.

OPTIONS
=======
`-f`, `--filename` PATH
: Specifies the path to a file containing the smart permission to upload to the
  scabbard service. This option is required.

`-k`, `--key` FILE
: Indicates the key file to use for signing scabbard transactions. The `FILE`
  can be a relative or absolute file path, or it can be the name of a .priv file
  in the `$HOME/.splinter/keys` directory. The target file must contain a valid
  secp256k1 private key. This option is required.

`--service-id` ID
: Specifies the fully-qualified service ID of the targeted scabbard service,
  using the format `CIRCUIT_ID::SERVICE_ID`. This option is required.

`-U`, `--url` URL
: Specifies the URL for the `splinterd` REST API that is running the targeted
  scabbard service. (default `http://localhost:8080`) This option is required.

`--wait` SECONDS
: If provided, waits the given number of seconds for the batch to commit.
  Displays an error message if the batch does not commit in time.

ARGUMENTS
=========
`ORG_ID`
: Provides the ID of the organization this smart permission applies to.

`NAME`
: Provides a name for the new smart permission.

EXAMPLES
========
The following command creates a new `admin` smart permission for the `acme`
organization ID in a scabbard service on circuit `01234-ABCDE` with service ID
`abcd`, running on the node with the REST API endpoint `http://localhost:8088`.
The smart permission is loaded from the file at `~/smart-perm-acme-admin`, and
the transaction will be signed with the key located in the file `~/user.priv`.

```
$ scabbard sp create \
  --url http://localhost:8088 \
  --service-id 01234-ABCDE::abcd \
  --key ~/user.priv \
  --filename ~/smart-perm-acme-admin \
  acme \
  admin
```

The next command creates a new `data_entry` smart permission for the `bubba`
organization ID in the same scabbard service, using the smart permission file
`~/smart-perm-bubba-data_entry`. This time, the command specifies a key in the
`$HOME/.splinter/keys` directory by name and waits up to 10 seconds for the
smart permission creation batch to commit.

```
$ scabbard sp create \
  --url http://localhost:8088 \
  --service-id 01234-ABCDE::abcd \
  --key user \
  --wait 10 \
  --filename ~/smart-perm-bubba-data_entry \
  bubba \
  data_entry
```

SEE ALSO
========
| `scabbard-sp-delete(1)`
| `scabbard-sp-update(1)`
|
| Splinter documentation: https://www.splinter.dev/docs/
