# Release Notes

## Changes in Splinter 0.3.16

### Highlights

* The Splinter daemon, `splinterd`, can be configured with multiple read-only
  node registry files (in YAML format). Specify registries with file paths
  (prefixed with `file://`) or HTTP(S) URLs (prefixed with `http://` or
  `https://`).

### Deprecations and Breaking Changes

* The `--transport` option has been removed from the `splinterd` command. The
  `splinterd` `transport` configuration setting was also removed.

* The protocol prefix for TLS transport has been changed to `tcps://`. The old
  prefix, `tls://`, is still supported but is considered deprecated.

* The TLS options and configuration settings for `splinterd` are now prefixed
  with `tls`.

  Changed configuration settings:

    ```
    cert_dir  -> tls_cert_dir
    ca_cert -> tls_ca_file
    client_cert -> tls_client_cert
    client_key -> tls_client_key
    server_cert -> tls_server_cert
    server_key -> tls_server_key
    insecure -> tls_insecure
    ```

  Changed `splinterd` command options:

    ```
    --cert-dir  -> --tls-cert-dir
   --ca-file -> --tls-ca-file
   --client-cert -> --tls-client-cert
   --client-key -> --tls-client-key
    --server-cert -> --tls-server-cert
    --server-key -> --tls-server-key
    --insecure -> --tls-insecure
    ```

* A required `version` field has been added to all config objects. TOML
  configuration files should have a `version = 1` added at the beginning of the
  file.

* The `splinterd` configuration settings `registry_backend` and `registry_file`
  are no longer available. The related `splinterd` command options
  `--registry-backend` and `--registry-file` are also gone. Instead, use the
  `registries` configuration file setting or the `--registry` option with the
  `splinterd` command.

* Nodes may now have multiple network endpoints. The `splinterd` configuration
  setting `network_endpoint`, which was previously a single value, has been
  changed to `network_endpoints` and now takes an array of values. The
  `splinterd` command line option `--network-endpoint` remains the same, but can
  be specified multiple times. This change also affects node registry files,
  circuit proposals, and REST API responses; see the upgrade document below for
  more details.

For upgrade information, see
[Upgrading to Splinter 0.3.16 from Splinter 0.3.15](https://github.com/Cargill/splinter-docs/blob/master/docs/upgrading/splinter-v0.3.16-from-v0.3.15.md).

### libsplinter

* Change the following features from experimental to stable. They can be enabled
  with the “stable” feature flag (instead of the “experimental” feature flag),
  in addition to being enabled individually. These features will be used by the
  default and stable Docker images published at
  [splintercommunity](https://hub.docker.com/u/splintercommunity).

    - `biome-key-management`
    - `biome-credentials`
    - `registry-remote`
    - `rest-api-cors`

* Remove the following features (no longer available as compilation options),
because the functionality is now available by default:

    - `node-registry-unified`

* Update the `Dispatcher` code:

    - Add `From` implementation for `ProtoConversionError` in the
    `dispatch_proto` module.  This will allow for uses of the various proto
    conversion traits to be used with the `?` operator in handlers.
    - Add a function to the `Handler` trait for `match_type`, which more tightly
    couples the message type to the handler.  This change greatly reduces the
    possibility that the handler and the type it is registered to could get out
    of sync.
    - Add a trait for message sending that can be coupled to the Source via
    generics.  A new module, `dispatch_peer`, implements this new trait
    specifically on `PeerId` for `NetworkMessageSender`, which supports the
    existing usages.

* Switch the dispatch `Handler` trait from using generics to using associated
  types.

* Add a new generic parameter on dispatcher-related structs, `Source`, that can
  be either `ConnectionId` or `PeerId`.  The generic type defaults to `PeerId`
  for backwards compatibility.

* Add a step to check if a request has method `OPTIONS` if `CORS` is enabled.

* Allow adding a NetworkMessageSender after a Dispatcher has been created.

* Add an optional whitelist to the RestApi object used to instantiate CORS
  middleware.

* Improve the `InprocTransport` `ConnectionRefused` error when there's no
  `Listener`.

* Improve the error message when a service cannot be initialized.

* Add the method `port_numbers` to `RestApiShutdownHandle`, which returns a list
  of ports that the REST API was bound to when it was created.

* Simplify the connection manager error implementation.

* Improve the node registry:

  - Remove the no-op node registry since it is no longer used.
  - Update node registry errors to match the standard pattern used elsewhere in
    Splinter.
  - Rename `YamlNodeRegistry` to `LocalYamlNodeRegistry` to prevent confusion
    with the `RemoteYamlNodeRegistry`.
  - Add the `RemoteYamlNodeRegistry` for reading node registry files over
    HTTP(S).
  - Improve node registry documentation.
  - Replace the `Node` constructor with the `NodeBuilder`
  - Add a newline to the end of local YAML node registry files.

* Switch two phase coordinator timeout from milliseconds to seconds.

#### Testing

* Re-enable two mesh tests

#### Biome

* Remove the use of concrete stores from the Biome REST API. This makes it
  possible to test `BiomeRestResourceManager`.

* Add a `CredentialsStore`, `RefreshTokenStore`, `UserStore`, and `KeyStore`
  implementation that uses `Arc<Mutex<HashMap>>>` to store objects in memory.

* Change the source field for `<store_error>::StorageError` to
  `Option<Box<dyn Error>>`. This makes it easier to handle cases where the
  underlying error isn't interesting or important to surface, or if the error
  doesn't implement the `Error` trait.

* Fix a bug in the route handler for `GET /biome/key/{public_key}`,
  `PATCH /biome/key/{public_key}` and `DELETE /biome/key/{public_key}`.

* Fix a performance issue when updating a user's password.

* Add initial unit tests for the Biome REST API.


### splinterd

* Update the response for splinterd's `GET /status` endpoint to include the
  node's list of network endpoints.

* Add the `advertised_endpoints` configuration setting and
  `--advertised-endpoints` command option to `splinterd`. This value is used to
  define the node's publicly accessible network endpoints if they differ from
  the node's bound network endpoints. The `advertised_endpoints` setting is
  exposed via `GET /status` REST API endpoint.

* Add the `display_name` configuration setting to `splinterd`. This value is
  used to give the node a human-readable name. The `display_name` setting is
  exposed via splinterd's  `GET /status` REST API endpoint.

* Update the `splinterd` `main` method to use the `Path` struct for building
  file paths.

* Remove the `--transport` option from the `splinterd` command.

* Add a required `version` field to all config objects and check that each
  config object added to the final `ConfigBuilder` object has the correct
  version

* Add support for './' and '../' in file paths for files necessary for
  `splinterd` configuration.

* Remove `registry_backend` and `registry_file` config options

* Add the configuration options `registries`, `registry_auto_refresh_interval`,
  and `registry_forced_refresh_interval`.

* Refactor the `splinterd` local registry location configuration.

### CLI

* Remove the `keygen` feature (no longer available as compilation option); the
  `splinter keygen` subcommand is now available by default.

* Add the long option `--url` to `splinter health status`.

### Documentation

* Add man pages for the following commands: `splinter`, `splinterd`,
  `splinter cert`, `splinter database`, and `splinter keygen`.

  To display a man page with the `man` command, use the dashed form of the name,
  where each space is replaced by a dash. For example, `man splinter-cert`.

### Supplychain

* Fix a non-deterministic failure in a supplychain integration test.

* Update code to support change to multiple endpoints for each node

* Add a database migration for the change from `endpoint` to `endpoints` for the
  `supplychain_member` table.

### Miscellaneous

* Add a justfile to support using `just` for simple cross-repo building,
  linting, and testing.
* Log a commit hash in the `splinter-dev` image.
* Add additional cleanup to the `splinter-dev` image.
* Update the `protoc-rust` dependency to version 2.14.


## Changes in Splinter 0.3.15

### Highlights

* Command name change: Use `splinter circuit propose` to propose a new circuit
  (instead of  `splinter circuit create`).
* The `splinter node alias` subcommands have been removed.
* Default values for the circuit management type and service type are now
  configurable with the environment variables `SPLINTER_CIRCUIT_MANAGEMENT_TYPE`
  and `SPLINTER_CIRCUIT_SERVICE_TYPE`.
* Biome routes for user keys no longer require a user ID.
* The splinterd REST API endpoints for reading and proposing circuits are now
  in the default compilation target. Previously, this functionality required
  the "experimental" feature flag during compilation.
* The "biome" and "postgres" features are now available with the "stable"
  feature flag (instead of "experimental") and will be included in the default
  and stable Docker images published at
  [splintercommunity](https://hub.docker.com/u/splintercommunity).
* There is a new [splinter-ui](https://github.com/Cargill/splinter-ui)
  repository for Canopy and saplings.

### Deprecations and Breaking Changes

* Biome routes for user keys no longer require a user ID.
    - The endpoint `biome/users/{user_id}/keys` is now `biome/keys`
    - The endpoint `biome/user/{user_id}/keys/{public_key}` is now
      `biome/keys/{public_key}`
* The `splinter circuit create` subcommand is now `splinter circuit propose`.
* The `splinter node alias` subcommands have been removed. This functionality
  will be replaced by `splinter circuit template` subcommands in an upcoming
  release.

For upgrade information, see [Upgrading to Splinter 0.3.15 from Splinter 0.3.14](https://github.com/Cargill/splinter-docs/blob/master/docs/upgrading/splinter-v0.3.15-from-v0.3.14.md).

### libsplinter

* Move the "biome" and "postgres" features from experimental to stable. They
  can be enabled with the “stable” feature flag (instead of the “experimental”
  feature flag), in addition to being enabled individually. These features
  will be used by the default and stable Docker images published at
  [splintercommunity](https://hub.docker.com/u/splintercommunity).
* Remove the following features (no longer available as compilation options),
  because the functionality is now available by default:
    - `biome-rest-api`
    - `circuit-read`
    - `proposal-read`
* Remove the following features (no longer available as compilation options):
    - `database` - Redundant; its functionality can be accessed with the
       "postgres" feature,
    - `json-web-tokens` - Deemed unnecessary, because using authorization tokens
       throughout the Biome REST API is always required,
* Clean up and expand the transport module:
    - Remove unused errors, PollError and StatusError, from the transport
      module.
    - Remove transport status enum.
    - Implement `Display` for transport errors.
* Refactor the database module:
    - Refactor the constructor for `ConnectionPool` to `ConnectionPool::new_pg`
      to support multiple backends.
    - Replace `database::DatabaseError` with `database::ConnectionError`
      because the only valid database errors were connection related.

#### Testing

* The REST API tests now shut down after the tests finish.

#### Biome

* Change two Biome routes so that the user ID is not required in the route. The
  user ID is now derived from the provided access token.
    - `biome/users/{USER-ID}/keys` has changed to to `biome/keys`
    - `biome/users/{USER-ID}/keys/{public_key}` has changed to
      `biome/keys/{public_key}`
* Add a "refresh_token"  feature to the list of experimental features. When
  this feature is enabled, `biome/login` now returns a refresh token. (Refresh
  tokens are sent to the `POST /biome/token` endpoint to generate new access
  tokens without having to collect credentials when the short-lived access
  tokens expire.) Also, Biome has a new endpoint, called `/biome/tokens`, that
  validates refresh tokens and returns a new access token to the API consumer.
* Add a `/biome/logout` route.
* Rename and update several Biome items:
    - `SplinterUserStore` is now `DieselUserStore`
    - `SplinterCredentialsStore` is now `DieselCredentialsStore`
    - `SplinterUser` is now `User`.
    - `UserCredentials` is now `Credentials`.
    - `CredentialsStore` trait method `get_usernames` is now `list_usernames`.
    - Remove the generic type definition from the `UserStore` trait and the
      `CredentialsStore` trait.
    - Add the `update_keys_and_password` method to `KeyStore` trait.
* Add the `new_key_pairs` field to the `PUT /biome/users/{user_id}` payload.
     ```
     {
       "username":"test@test1.com",
       "hashed_password":"Admin2193!",
       "new_password":"hello",
       "new_key_pairs":[{
           "display_name":"test",
           "encrypted_private_key":"<encryped_private_key>",
           "public_key":"<private_key>"
       }],
    }
    ```
* `BiomeRestResourceManager` now requires `CredentialsStore` to be created.

#### Protobuf

* Add `FromProto`, `FromNative`, `IntoBytes`, and `FromBytes` to the protobuf
  module.
* Add a `ViaProtocol` generic parameter to the `FromBytes` and `ToBytes` traits.
  This allows for auto-implementations of the types for implementers of
  FromNative and FromProto.
* Add a `splinter::protos::prelude` module.

### CLI

* Change the following features from "experimental" to "stable". They can now
  be enabled with the “stable” feature flag, in addition to being enabled
  directly, and will be used by the default and stable Docker images published
  at [splintercommunity](https://hub.docker.com/u/splintercommunity).
    - `database-migrate-biome`
    - `circuit`
* Remove the following feature (no longer available as compilation options):
    - `node-alias` - Replaced by the `circuit-template` feature
* Remove the `splinter node alias` subcommands `add`, `delete`, `list`, and
  `show`. This functionality will be replaced by the `splinter circuit template`
  subcommands in an upcoming release.
* Update `splinter circuit create` command to display the newly proposed
  circuit after it is submitted.
* Add the `--node-file` option to the `splinter circuit create` command. This
  option loads a list of nodes from a YAML file, either on the local file
  system or from a remote server with HTTP. You can use this option with (or
  instead of) the `--node` option. This option supports the Splinter node file
  types used by the node registry and the `splinter node alias` commands.
* Update the `CreateCircuitMessageBuilder::add_node` method (which is used by
  the `splinter circuit propose` command) to check for duplicate node IDs and
  endpoints. If a duplicate is detected, an error is returned to the user.
* Update the `splinter` CLI's `CreateCircuitMessageBuilder::add_service` method
  (which is used for adding services specified with the `--service` argument),
  to check for duplicate service IDs.
* Remove the `node-alias` feature (the `splinter node alias` subcommands) from
  the splinter CLI. This will be replaced by the `circuit-template` feature
  (`splinter circuit template` subcommands) .
* Simplify proposing a circuit with `splinter circuit propose` by automatically
  setting the circuit metadata.
* Update the `splinter circuit propose` command to check for duplicate service
  arguments and return an error if one is found.
* Remove the `splinter circuit default` subcommands and all associated code.
  Default values for the circuit management type and service type are now
  configurable with the environment variables `SPLINTER_CIRCUIT_MANAGEMENT_TYPE`
  and `SPLINTER_CIRCUIT_SERVICE_TYPE`. This change simplifies circuit creation
  and lets users set default values with environment variables.
* Change the `splinter circuit show` command to require a circuit ID as an
  argument.
* Increase the paging limit for `splinter circuit list` to 1000.

### Canopy

* Remove Canopy from the splinter repository and move it to
  https://github.com/Cargill/splinter-ui

### Documentation

* Add man pages for the `splinter circuit` subcommands and `splinter database migrate`.
  To display a man page with the `man` command, use the dashed form of the name,
  where each space is replaced by a dash.
    - `splinter-circuit`
    - `splinter-circuit-list`
    - `splinter-circuit-proposals`
    - `splinter-circuit-propose`
    - `splinter-circuit-show`
    - `splinter-circuit-template`
    - `splinter-circuit-template-arguments`
    - `splinter-circuit-template-list`
    - `splinter-circuit-template-show`
    - `splinter-circuit-vote`
    - `splinter-database-migrate`

### Miscellaneous

* Update the examples
  [private_counter](https://github.com/Cargill/splinter/tree/master/examples/private_counter)
  and [private_xo](https://github.com/Cargill/splinter/tree/master/examples/private_xo)
  to be compatible with Splinter version 0.3.15.

## Changes in Splinter 0.3.14

### Highlights

* Service ID must now conform to a specific format (a 4-character base-62 string),
  which is enforced by the `SplinterServiceBuilder::build` method when provided
  and randomly generated if one is not.
* Circuit ID must now conform to a specific format (an 11-character string composed
  of two 5-character base-62 strings, joined with a `-`), which is enforced by the
  `CreateCircuitBuilder::build` method when provided and randomly generated if one is not.

### Deprecations

* `splinter::transport::raw` has been deprecated in favor of `splinter::socket::tcp`
* `splinter::transport::tls` has been deprecated in favor of `splinter::socket::tls`

### libsplinter:
* Reorganize the socket-based transports, raw (now renamed tcp) and TLS, under a
  `transport::socket` module.
* Rename `RawTransport` to `TcpTransport` to better reflect the underlying capability.
  Also rename the `socket::raw` module to `socket::tcp`.
* Add a `comments` field to the `Circuit` object to allow for human-readable comments.
* Save a disconnected connection in a `HashMap` to be returned in case the connection
  is removed again at a later time.
* Separate circuit and proposal REST API responses from the internal structs so
  the internal structures can change as needed without impacting the data exposed
  via the REST API.
* Add a `BiHashMap` to `Mesh` for a unique ID to the mesh ID.
* Allow `BiHashMap` look-ups with elements that implement the `Borrow` trait.
* Replace the `RecvError::InternalError` with a `Disconnected` error for accuracy.
* Experimental feature “circuit-template” (new): Implement `CircuitTemplateManager`
  with functionality to list and load available templates.
* For all experimental Biome features:
    - Remove Arc wrappings from Biome’s user store to allow mutable references of
      the user store.
    - Separate Biome models and schemas into their respective modules.
    - Add a `/biome/verify` REST API endpoint to verify a user’s password.
    - Update the Biome Rust API documentation to remove explicit references to
      Rust features.
    - Make the Biome user module private.
    - Rename `biome::datastore` to `biome::migrations` and add module-level
      documentation for biome migrations.
    - Remove all uses of `super` imports in Biome.
* Correct `json-web-tokens` feature guards.
* Experimental feature “proposal-read”:
    - Update the `ProposalStore::proposals` arguments to take a `ProposalFilter`
      enum to allow the proposal store to filter returned proposals.
    - Rename the `/admin/proposals` endpoint's `filter` query parameter to `management_type`.
    - Add `member` filter query parameter to the `GET /admin/proposals` REST API endpoint.

### CLI:
* Experimental feature “circuit”:
    - Update the `splinter circuit create` command to generate a service ID automatically.
    - Add a `comments` argument to the `splinter circuit create` command to optionally
      provide comment for the proposed circuit.
    - Display comments in the output of the `splinter proposals` command.
    - Update the `circuit-read` and `proposal-read` response deserializers to reflect
      the REST API response objects, rather than the related internal structs.
    - Add a `member` argument to the `splinter circuit proposals` command for
      filtering proposals by the given member.
    - Add `SPLINTER_REST_API_URL` environment variable to be used if `-U` or `--url`
      is not specified.
    - Remove the `required` specification from `splinter circuit list` and
      `splinter circuit proposals` arguments that are not actually required.
* Experimental feature “circuit-template” (new): Implement the experimental
  `circuit-template` feature. This feature lets you use a template to create a
   circuit with the new `template` subcommand for `splinter circuit create`.
    - Add the `dry_run` flag to the `circuit create` command, which displays the
      circuit definition without submitting the proposal.
    - Add subcommands behind the `circuit-template` feature to display circuit templates:
      `splinter circuit template list` and `splinter circuit template show`.

### packaging:
* Publish the Splinter package to crates.io when the repository is tagged.
* Add pandoc to the `splinter-dev` image.
* Update the version of `splinter-dev` to v2 and update Dockerfiles to use `splinter-dev:v2`.
* Experimental feature "circuit-template" (new): Include a `scabbard` template with
  the Splinter CLI and include the `supplychain` template with the supplychain daemon.

## Changes in Splinter 0.3.13

### Highlights

* Breaking change: Socket-based transports (TCP and TLS) now require a version
  handshake when connecting.
* New experimental feature `circuit-template`: Circuit template library to
  simplify circuit creation.

### Deprecations and Breaking Changes

For upgrade information, see [Upgrading to Splinter 0.3.13 from
Splinter 0.3.12](https://github.com/Cargill/splinter-docs/blob/master/docs/upgrading/splinter-v0.3.13-from-v0.3.12.md).

* Socket-based transports, such as TCP and TLS, require a version handshake when
  connecting. This handshake allows the pair to negotiate the header version for
  messages sent over the connection. The V1 header currently consists of a
  version, a payload length, and a header checksum.
* `TlsConnection::new` is deprecated. A `TlsConnection` should only be created
  via `TlsTransport`.

### libsplinter

* `RestApi::run` now returns a bind error if it fails to bind to an address.
  `RestApiServerError` has an additional variant, `BindError`.  Matching on
  specific errors will require a new statement for this variant, unless a
  catch-all statement is used.
* Experimental feature "circuit-template" (new): The circuit::template module
  provides library code for building tools to create circuits using templates.
* All experimental Biome features:
    - New OpenAPI documentation for REST API routes.
    - Improved Rust documentation; see
      [docs.rs/splinter](https://docs.rs/splinter/).
    - Restructured data stores to follow new model that enables future
      database selections.
* Experimental feature "biome-key-management": Added response bodies to the
  key management REST API routes.
* Experimental feature "biome-user":
  - Added response bodies to the user REST API routes.
  - Added `/biome/users/{user_id}/keys/{public_key}` endpoint, with both GET and
    DELETE handlers to fetch a specific user by public key and delete a user,
    respectively.
* Experimental feature “json-web-tokens” (new): Moved the sessions and secrets
  modules from behind the biome experimental feature to the REST API module,
  behind the “json-web-tokens” experimental feature
* Experimental feature "connection-manager":
    - Remove `CmResponse` from the Connector API in order to provide a more
      idiomatic Rust API.
    - Automatic reconnection for broken connections.

### CLI

* Experimental feature "circuit": The `splinter circuit create` subcommand now
  displays the circuit ID for a new circuit.

## Changes in Splinter 0.3.12

### Highlights:
* REST APIs and clients are now versioned with a ProtocolVersion.
* OPTIONS/CORS is now supported by the splinterd REST API
* [CanopyJs](https://github.com/Cargill/splinter-canopyjs) and [SaplingJS](https://github.com/Cargill/splinter-saplingjs) have been moved to their own repos.
* The scabbard CLI now provides experimental list and show commands for uploaded
  smart contracts.
* The scabbard REST API now has an experimental get state route
* Numerous bug fixes and documentation improvements.

### Deprecations and Breaking Changes:
  For information on upgrading from 0.3.11 to 0.3.12 see [Upgrading](https://github.com/Cargill/splinter-docs/blob/master/docs/upgrading/splinter-v0.3.12-from-v0.3.11.md) documentation
* Added “admin” prefix to circuits, key registry and node registry REST API
  routes.
* Rewrote the circuit create command so it does not accept a circuit definition
  in a YAML file, but rather it creates the circuit definition based on CLI
  arguments.
* Removed the deprecated generate-cert feature. Now you must use the `splinter
  cert generate` command to create development certificates and keys for using
  the TLS transport. See the how-to for [Generating Insecure Certificates for
  Development](https://github.com/Cargill/splinter-docs/blob/master/docs/howto/generating_insecure_certificates_for_development.md) for more information.
* Changed `scabbard upload` command to `scabbard contract upload` and updated
  .scar file to be loaded by name and version from a specified `--path`
  argument.
* Updated the version of Sawtooth Sabre used by scabbard to v0.5.

### libsplinter:
* Add “admin” prefix to circuits route, key registry and node registry routes.
* Add optional protocol headers to REST APIs. The protocol guard looks for an
  optional header, SplinterProtocolVersion. If this is provided, it will return
  a BadRequest if the version provided is out of range.
* Add protocol guards to the Scabbard REST API endpoints
* Add protocol guards to the Admin service REST API endpoints
* Add protocol guards to the splinterd REST API endpoints
* Add protocol version guards to biome endpoints
* Add openapi documentation for the Biome routes.
* Wait full time in scabbard batch status check.
* Add a CircuitStore trait. This trait will be the external interface for access
  circuit information. This trait was also implemented for SplinterState
  directly.
* Update the circuit routes to use CircuitStore instead of an
  Arc<RwLock<SplinterState>> directly.
* Include node_id in circuit vote error message when the node is not allowed to
  vote for a node on the proposal.
* Validate the requestor's public key length when acting on a circuit proposal
  or vote in the AdminService.
* Reorganize the admin REST API and move circuit REST API into the
  admin::rest_api module.
* Improve shutdown of event reactor by removing the delay on shutdown caused by
  the event::Reactor using the combination of a running flag and reacting to
  that flag based on the, as well as signalling and joining shutdown in the
  same function (delaying other shutdown activities).
* Remove unwrap in libsplinter storage module. This error was propagated to the
  CLI and made debugging difficult.
* Add better documentation updates for circuit-read. Includes fixing the openapi
  documentation for the circuit routes, as well as adding rust doc comments to
  the route implementations.
* Add ServiceArgValidator and implement the trait for Scabbard. The admin
  services will now validate that a circuit proposal has valid service
  arguments, using the new, experimental, trait ServiceArgValidator.
* Add the CircuitFilter, for use as strongly-typed filter parameters to the
  circuits function on CircuitStore.
* Update the sawtooth, sawtooth-sdk, and transact dependencies to the latest
  versions.
* Remove sawtooth-sdk dependency from the scabbard client; it is no longer
  needed with updates to transact.
* Update the scabbard client's ServiceId struct to allow creating directly from
  circuit and service IDs, and to check that circuit and service ID are
  non-empty when parsing from a string.

### splinterd:
* Add a ClapPartialConfigBuilder object, used to construct a PartialConfig
  object from Clap argument values, available behind the `config-command-line`
  feature flag.
* Add a DefaultPartialConfigBuilder object, used to construct a PartialConfig
  object from default values, available behind the `config-default` feature
  flag.
* Add an EnvPartialConfigBuilder object, used to construct a PartialConfig
  object from configuration values defined as environment variables, available
  behind the `config-env-var` feature flag.
* Separate the TomlConfig object from the PartialConfigBuilder implementation as
  this object is now used to define the valid format for a configuration toml
  file.
* Renamed the previous PartialConfigBuilder implementation for the TomlConfig to
  TomlPartialConfigBuilder.
* Add the ConfigBuilder object, which takes in PartialConfigs and then
  constructs a Config object from the values set in the PartialConfig objects.
* Add the Config object; it is used to hold configuration variables compiled
  from several PartialConfig builder objects as well as the source of each
  respective value.
* Simplify logging for the Config object to display the raw configuration
  variables defined in the Config object.
* Added more robust logging for file operations, logging the fully qualified
  path of a file anytime a file is used.
* Remove the deprecated generate-cert feature. Now you must use the
  `splinter cert generate` command to create development certificates and keys
  for using the TLS transport.
* Clean up the main function in splinterd, specifically in how the Config
  objects are used.
* Handle OPTIONS and CORS Requests in splinterd. Respond to an HTTP OPTIONS
  request by returning all of the allowed methods. Also add support for
  handling CORS requests, guarded by a rust feature "rest-api-cors".  This
  checks the preflight conditions of the request, and fails the request if the
  preflight conditions are not met.


### splinter CLI:
* Update the output of the `circuit` and `proposal` subcommands to default to a
  human readable format.
* Add experimental `node alias` subcommand to save node information locally.
  This information can be used to simplify creating circuits.
* Add experimental `circuit default` commands to save local defaults for
  service-types and management type. This information can be used to simplify
  creating circuits.
* Rename keygen `--admin` to `--system` to better reflect the functionality of
  the flag, which is to generate keys for use by a splinter node.
* Send the ADMIN_PROTOCOL_VERSION with the SplinterProtocolVersion header when
  making admin client requests.
* Set the log level for low level crates to Warn to reduce the noise when using
  the cli with -vv
* Rewrite the circuit create command so it does not accept a circuit definition
  in a YAML file, but rather it creates the circuit definition based on CLI
  arguments.
* Add improved help text for `splinter cert generate`.

## scabbard CLI
* Add `GET /state` and `GET /state/{address}` endpoints for scabbard, behind the
  experimental `scabbard-get-state` feature.
* Add contract list/show subcommands to scabbard CLI. `scabbard list` will list
  the name, versions and owners of deployed contracts. `scabbard show` will
  print out the name version, inputs, outputs and who created the contract.
* Send the SCABBARD_PROTOCOL_VERSION with the SplinterProtocolVersion header
  when making scabbard client requests.
* Fix "execute" feature name in cfg's.
* Eliminate recursion in scabbard client's wait. Update the scabbard client's
  `wait_for_batches` function to use a loop rather than recursion to avoid stack
  overflows.
* Use new transaction/batch building pattern enabled by sabre/sawtooth/transact
  to simplify submitting transactions.
* Load .scar files using transact's new .scar file loading functionality; this
  changes the `contract upload` command to take a path and a contract
  name/version as arguments rather than a file path for the .scar file.

### health service:
* Add rest-api feature to Cargo toml. This fixes a bug where it could not build
  if built outside of the workspace.

### Supplychain:
* Fix games disappearing on refresh. Adds a check to ensure that games are not
  refreshed if selectedSupplychain state is empty.
* Remove the vuex-module-decorators dependency, which was causing issues with
  debugging and provides little benefit.
* Implement a new component, Loading, which renders a spinner and a message
  supplied by a prop. This standardizes the approach to loading indicators
  throughout supplychain.
* Update the vuex page loading store to store a message.
* Trigger a loading indicator when pages are being lazy loaded or data has to be
  fetched before the page can fully load
* Update supplychain daemon to actix 2.0.
* Remove unnecessary Pike namespace permissions from setting up the XO contract
  in the supplychain daemon.
* Use new transaction/batch building pattern enabled by sabre/sawtooth/transact
  to simplify submitting transactions to scabbard.
* Update the Sabre version for transactions submitted by the supplychain web app to
  match the Sabre version used by scabbard (v0.5).

### Supplychain cli:
* Change supplychain cli version to match the rest of the repo

### Packaging:
* Add curl to the scabbard-cli docker image to enable fetching remote .scar
  files.
* Add scabbard to splinter-dev dockerfile

## Changes in Splinter 0.3.11

### Highlights:
* Splinter supports dynamic multi-party circuit creation in scenarios where the
peers are not connected when the circuit proposal is submitted.
* A new scabbard CLI provides experimental subcommands to submit batches of
transactions to a scabbard service.
* New experimental endpoints have been added to get state from a scabbard
service.
* Information on how to run the Supplychain demo using Kubernetes is available in
[docker/kubernetes/README.md](https://github.com/Cargill/splinter/blob/master/docker/kubernetes/README.md).


### libsplinter:
- Enable more than two-party circuit connection. The admin service now waits
for all nodes in the circuit proposal to be peered before handling a consensus
proposal.
- Add Biome (user management) routes for fetching, listing and deleting users
and updating credentials.
- Establish connection to peers when handling votes. If the connection between
nodes is dropped after a proposal is submitted, a node will try to re-establish
the connection before submitting the vote.
- Add support to configure the timeout value for two-phase commit.
- Set the default timeout for admin and scabbard services to 30 seconds.
- Add experimental endpoints (behind the "scabbard-get-state" feature) for
fetching and listing entries in scabbard state.

### splinterd:
- Replace the current Config object with a PartialConfig and a
PartialConfigBuilder. This is the first part of significantly refactoring how
the Splinter daemon is configured.
- Fix the panic caused by unwrapping a bad protobuf message

### Supplychain:
- Add example files and instructions (in docker/kubernetes) on how to run the
Supplychain demo using Kubernetes.

### scabbard CLI:
- Add the scabbard CLI with experimental subcommands that closely resemble
those of the sabre CLI in Sawtooth Sabre.

### splinter CLI:
- Add a keygen subcommand to generate a user's public/private key pair. This
subcommand uses the "keygen" experimental feature.

### Packaging:
- Update the libsplinter Cargo.toml file to include only experimental and
stable in the list of features in package.metadata.docs.rs.
- Add the Supplychain CLI to the splinter-dev docker image.
- Publish Docker images built with experimental features during nightly cron
builds.

## Changes in Splinter 0.3.10

### libsplinter:
- Fix the wait for batch status in the ScabbardClient by adding the base URL if
  one is not provided.
- Add comments to ScabbardClient tests.

### splinterd:
- Add the CLI flag --enable-biome to the splinterd CLI.  The addition of this
  flag relaxes the requirement that the database-url option must be set.
  Currently, the database-url option is only required for the Biome subsystem.

### Supplychain:
- Add migrations for the Supplychain database to support upgrading an existing
  supplychain daemon.

### Supplychain CLI:
- Add supplychain CLI, which will initially be used to migrate the supplychain
  database.

## Changes in Splinter 0.3.9

### Highlights:
* A Splinter crate is now available at https://crates.io/crates/splinter.
* The "splinter" CLI has new experimental subcommands:
  - splinter circuit create (create a circuit)
  - splinter circuit vote (vote on a circuit)
  - splinter circuit list (display a list of circuits)
  - splinter circuit proposals (display a list of circuit proposals)
  - splinter circuit show (display circuit and proposal details)

### Deprecations and Breaking Changes:
* The new node registry format is enforced as of release 0.3.8. Each entry in
  the node registry must specify a node ID, a display name, and an endpoint.

### libsplinter:
* Add experimental list and fetch routes for circuit proposals to the admin
  service REST API.
* Add key management routes to the Biome REST API, including POST, GET and
  PATCH  to /biome/users/{user_id}/keys.
* Fix Scabbard initial events by rejecting an empty last_seen_event query
  parameter and properly handling empty transaction receipt stores.
* Add a newline to key files generated by the splinter CLI.
* Start initial implementation of ScabbardClient, which provides a convenient
  way to submit batches to a scabbard instance.

### splinterd
* Add experimental Biome users API routes to splinterd.
* Add experimental list and fetch routes for circuit proposals to the admin
  service REST API.
* Log a debug message instead of a warning when the splinterd config file is
  not found.
* Remove panics that can be caused by a user from Splinter daemon startup.
* Fix typos and standardize capitalization in splinterd help and error messages.

### splinter CLI
* Implement experimental "circuit create" subcommand.
* Implement experimental "circuit vote" subcommand.
* Add experimental "circuit list" and "circuit show" subcommands.
* Add "splinter circuit proposals" subcommand to list proposals.
* Add support for showing proposal details to "circuit list" subcommand.
* Change verbose and quiet flags to be global.

### Supplychain
* Add a docker-compose file that uses published images. This compose file can be
  used along with the repository in situations where building from scratch is
  not feasible.
* Update the Supplychain README with CARGO_ARGS instructions for running with
  experimental features.

### Packaging
* Update splinterd packaging for the current node registry format.
* Log current git HEAD commit hash during docker image builds.
* Add a description to the Cargo.toml files.

## Changes in Splinter 0.3.8

### Highlights:

* A new "experimental" feature set.  Features marked as experimental are
  available for use, but are subject to change.
* The “splinter-cli” command has been renamed to “splinter”.

### Deprecations and Breaking Changes:

* The Splinter CLI name has changed from “splinter-cli” to “splinter”.
  “splinter-cli” remains as an alias for “splinter”, but should be considered
  deprecated as of this release.
* The node registry trait was extensively updated to support iteration,
  implementation-agnostic filtering, and simplified modifications.

### Libsplinter:

* The protobuf files are now under the libsplinter subdirectory.  This enables
  publishing the "splinter" library crate.
* NodeRegistry::list_nodes now returns an iterator.
* NodeRegistry::update_node and add_node have been merged to
  NodeRegistry::insert.
* The REST API implementations for circuits, node registry, and key registry
  have moved from splinterd to libsplinter.
* A new "rest-api" stable feature includes the library functions mentioned
  above.
* New Biome features for user-related functionality such as credentials and key
  management. (Biome is the libsplinter module that supports user management.)
    - "biome"
    - "biome-credentials"
    - "biome-key-management"
    - "biome-notifications"
* A new "experimental" feature set that includes all experimental features.

## Changes in Splinter 0.3.7

### Highlights:
* The admin service and the scabbard service can now send catch-up events to
  bring new subscribers up to date

### Deprecations and Breaking Changes:
* The splinterd --generate-certs flag, which was deprecated in 0.3.6, is still
  available by default. In 0.3.8, the flag will not be available by default.
  Instead, you must use the Rust compile-time feature “generate-certs” to
  explicitly enable the deprecated --generate-certs flag. For more information,
  see the 0.3.6 release notes.
* In the next release, the splinter CLI name will change from “splinter-cli” to
  “splinter”. “splinter-cli” will exist as an alias for “splinter”, but should
  be considered deprecated as of release 0.3.8.

### libsplinter:
* Refactor the admin service and scabbard service to separate the REST API code
* Change admin service events to include a timestamp of when the event occurred
* Update the admin service to send all historical events that have occurred
  since a given timestamp when an app auth handler subscribes
* Update the scabbard event format to correlate directly with a transaction
  receipt
* Remove EventHistory from the REST API because it is no longer used
* Remove EventDealer from the REST API and replace it with the EventSender
* Update the event sender to send catch-up events as an asynchronous stream
* Add state-delta catch-up to the scabbard service, sending all events that
  occurred since a given event ID when a subscription request is received
* Update Network to properly clean up connections on disconnection

### splinterd:
* Fix the splinterd --heartbeat argument to properly accept a value

### Supplychain example:
* Update the supplychaind app auth handler to track the timestamp of the last-seen
  admin event
* Add the timestamp for the last-seen admin event to the app auth handler’s
  subscription request for getting any catch-up events
* Update the supplychain state-delta subscriber to track the ID of the last-seen
  scabbard event
* Add the ID of the last-seen scabbard event when subscribing to scabbard on
  restart, which lets the supplychain daemon receive catch-up events

### Private XO example:
* Replace the transact git repo dependency with a crates.io dependency

## Changes in Splinter 0.3.6

### Highlights:
* Peers can now successfully reconnect after restarting
* Faster build times:
  * Added a Dockerfile for splinter-dev docker image
  * Based the installed images on the splinter-dev image
  * Updated the compose files to build the installed docker images
  * Added parallelization to Travis CI builds
* Initial database structure for Biome, the libsplinter module that supports
  user management
* New Supplychain Technical Walkthrough document that explains the Splinter
  functionality that powers the Supplychain application; see
  examples/supplychain/README.md for a link to the PDF

### Deprecated Features:
* The --generate-certs flag for splinterd is now deprecated. Instead, please
  generate development certificates and keys using the new command "splinter-cli
  cert generate". This command will generate the certificates and keys in
  /etc/splinter/certs/ (by default) or in the specified directory.
  Note:  --generate-certs is still available by default in 0.3.6. It will be
  turned off in 0.3.7, but will still be available with a Rust compile-time
  feature flag. If using generated certificates, run splinterd with the
  --insecure flag.

### libsplinter:
* Improve logging:
  * Log when a peer is removed
  * Log an event Reactor background thread error on startup
  * Log REST API background thread startup errors immediately, rather than on
    shutdown
  * Log a WebSocket shutdown
  * Log a peer connection initiation
  * Log the configuration used to start splinterd
  * Add timestamp and thread name to log messages
* Return an error when a peer is disconnected
* Allow consensus threads to log error and exit, rather than panic
* Enforce that member, endpoint, and service IDs are unique to a circuit
* Update the example TOML configuration file
* Verify a CircuitManagementPayload message's payload field, header field, and
  payload signature
* Update example circuit files to use correct enum types
* Fix a typo in DurabilityType enum
* Stop the admin service once a shutdown signal is received
* Fix a locking bug that prevented admin service from properly shutting down
* Stop running services upon admin service shutdown
* Include the service definition in service shutdown error
* Update format lint for Rust 1.39
* Add a Splinter PostgreSQL database to be used by Splinter modules
* Decouple EventDealer and EventHistory to allow the storage of events to be
  managed separately from event history
* Change the log levels of received messages and pings/pongs
* Update EventDealers to return error from EventDealer.add_sender method and
  handle errors from EventDealer.dispatch method
* Store AuthServiceEvents in a Mailbox, replacing LocalEventHistory
* Start reorganizing the admin service module
* Store pending changes as transaction receipts in scabbard
* Add a "state_" prefix to variables that refer to the scabbard LMDB backend
  database, which helps distinguish this database from other databases that
  scabbard may maintain
* Run tests behind the "experimental" feature
* Move the zmq-transport feature, which loads the ZMQ dependency, to experimental
* Rename the node registry method create_node add_node, which more accurately
  reflects its functionality
* Update the struct used to build REST API resources to represent multiple
  method and handler pairs for a given resource
* Fix the node registry implementation’s file editing to completely overwrite
  the YAML node registry file rather than append changes
* Add a disconnect listener to Network; this listener is used to close the
  connection when a peer is disconnected from the network
* Register the AuthorizationManager to listen for peer disconnections to clean
  up old state about the disconnected peer

### splinterd:
* Add endpoints for local registry, including:
  * POST /nodes
  * DELETE /nodes/{identity}
  * PATCH /nodes/{identity}
* Move the node registry implementation from splinterd to libsplinter
* Update the struct used to build REST API resources to represent multiple
  method and handler pairs for a given resource
* Run tests behind the experimental feature
* Add /circuits route, available with circuit-read experimental feature,
* Update splinterd to look for certificates and keys in /etc/splinter/certs (by
  default) or the location specified by "--cert-dir" or the environment variable
  SPLINTER_CERT_DIR
* Deprecate the generate-cert flag (will be removed in a future release) now
  that "splinter-cli cert generate" is available

### splinter-cli:
* Add subcommand "cert generate" to generate certificates and keys that can be
  used to run splinterd for development.

### Canopy:
* Add CSS styles for responsive side navigation bar
* Add default color styles to be used in design app
* Add default typography styles and initial typography documentation
* Add CSS class defaults and themes for navigation
* Add structure and initial introduction page for the documentation app
* Add configuration to build theme CSS bundles
* Add the initial structure for a sapling example (an application to extend
  Canopy)
* Implement register and initialize functions for saplings in CanopyJS
* Add lint and unit tests to Travis CI
* Refactor CanopyJS to improve clarity and extensibility
* Implement CanopyJS user

### Supplychain example:
* Add a generic-themed Supplychain app to installed docker-compose file
* Add functions to check for active supplychains and resubscribe on startup
* Add volumes for /var/lib/splinter to the docker-compose file
* Add timestamp and thread name to log messages
* Remove the hardcoded protocol for octet-stream submission; instead, use a
  relative URL handled by the proxy
* Attempt to reconnect WebSocket clients if a "close" message is received
* Time out WebSocket client connections and attempt to reconnect
* Convert signature hex string to bytes for signing payloads
* Base the test docker image on the splinter-dev docker image
* Fix a bug with cell selection

### Packaging:
* Remove known errors during a .deb package install

## Changes in Splinter 0.3.5

### Highlights:
* Add network-level heartbeats to improve peer connectivity
* Update Supplychain UI to use the WebSocket Secure protocol (wss) when the
  application protocol is HTTPS
* Improve libsplinter tests
* Add code of conduct to README
* Add the command-line option --common-name to splinterd

### Canopy:
* Add initial directory structure for the Canopy project, a web application
  that hosts pluggable applications and tools built on Splinter

### Supplychain example:
* Remove unnecessary logo files
* Update UI to use wss when the application protocol is HTTPS. This fixes an
  issue where the application could not communicate via WebSockets if the
  application was communicating over HTTPS
* Check for batch status after batch is submitted, then wait for batch to be
  committed or invalidated in supplychaind
* Remove member node’s metadata from supplychain propose request payload
* Fetch member node information from splinterd when supplychaind receives a
  supplychain propose request

### libsplinter:
* Add dockerfile for libsplinter crate generation
* Document the limitations for two-phase commit
* Add network-level heartbeats. The network now creates a thread that will send
  a one-way heartbeat to each connected peer every 30 seconds by default.
* Rename libsplinter crate to splinter
* Store the current state root hash for scabbard's shared transaction state in
  order to support restarts
* Simplify where services can be connected. This ensures that a service is
  connected to the first allowed node and that allowed nodes can only have one
  service.
* Remove peers when a node is disconnected


### libsplinter Testing:
* Update key_not_registered test to use a valid circuit
* Rename error_msg to msg in AdminDirectMessage tests
* Correctly set message type to CircuitMessageType::CIRCUIT_DIRECT_MESSAGE in
  AdminDirectMessage tests
* Fix typos in doc comments

## Changes in Splinter 0.3.4

### Highlights
* Implement a batch status endpoint for scabbard
* Set up the Cypress integration test framework for the Supplychain UI

### Supplychain example
* Copy Splinter .proto files into installed client builds
* Redirect the user to a “Not Found” page if the page does not exist
* Set up integration tests using Cypress
* Add XO smart contract to installed supplychaind builds

### libsplinter
* Reduce latency of events by replacing run_interval in EventDealerWebSocket
  with streams

### scabbard
* Change the scabbard database name to be the sha256 hash of
  service_id::circuit_id to ensure that it will be a valid file name
* Add signature and structure verification to the scabbard service when it
  receives batches submitted via the REST API
* Add /batch_statuses endpoint to scabbard and update /batches endpoint to
  return a /batch_statuses link for the submitted batch IDs

### splinterd
* Add config builder with toml loading (experimental feature)

### Packaging
* Add Dockerfile to package supplychain UI
* Update packaging for supplychaind and splinterd so modified systemd files are
  not overwritten
* Modify supplychaind and splinterd postinst scripts to add data directories
* Add plumbing to properly version deb packages

## Changes in Splinter 0.3.3

### Highlights
* Add functionality to create and play XO games

### libsplinter
* Add EventHistory trait to EventDealer to allow for new event subscribers to catch
  up on previous events. This trait describes how events are stored.
* Add LocalEventHistory, a basic implementation of EventHistory that stores events
  locally in a queue.
* Add MessageWrapper to be consumed by EventDealerWebsockets, to allow for
  shutdown messages to be sent by the EventDealer
* Enforce that a Splinter service may only be added to Splinter state if the
  connecting node is in its list of allowed nodes
* Add Context object for WebSocket callbacks to assist in restarting WebSocket
  connections
* Add specified supported service types to the service orchestrator to determine
  which service types are locally supported versus externally supported
* Only allow initialization of the orchestrator’s supported service
* On restart, reuse the services of circuits which are stored locally
* Add circuit ID when creating a service factory, in case it is needed by the
  service
* Replace UUID with service_id::circuit_id, which is guaranteed to be unique on
  a Splinter node, to name the Scabbard database
* Fix clippy error in events reactor
* Fix tests to match updated cargo args format
* Change certain circuit fields from strings to enums
* Remove Splinter client from CLI to decrease build time

### Supplychain Example
* Add ability in the UI to fetch and list XO games
* Correct arguments used to fetch the members of an existing supplychain, allowing
  the members to be included in the /supplychains endpoint response
* Add GET /keys/{public_key} endpoint to supplychaind, to fetch key information
  associated with a public key
* Add UI functionality to create a new XO game:
* Add ability to calculate addresses
* Add methods to build and sign XO transactions and batches
* Add methods to submit XO transactions and batches
* Add form for user to create new game
* Add new game notification to UI and supplychaind
* Add player information displayed for a game in UI
* Implement XO game board in UI
* Implement XO take functionality and state styling in UI
* Add component to show game information in the Supplychain details page in the UI
* Use md5 hash of game name when creating a game, rather than URL-encoded name
  that handles special characters
* Add player information when updating an XO game from exported data (from state
  delta export)
* Add auto-generated protos for the UI
* Remove the explicit caching in the Supplychain Detail view in the UI, because Vue
  does this automatically
* Make various UI styling fixes
* Remove unused imports to avoid cargo compilation warnings

## Changes in Splinter 0.3.2

### Highlights
* Completed the code to propose, accept, and create a supplychain in the Supplychain
  example application

### libsplinter
* Persist AdminService state that includes the pending circuits
* Replace the WebSocketClient with a new events module, which improves
  multi-threaded capabilities of the clients (libsplinter::events; requires the
  use of "events" feature flag)
* Improve log messages by logging the length of the bytes instead of the bytes
  themselves
* Fix issue with sending and receiving large messages (greater than 64k)
* Fix issues with threads exiting without reporting the error
* Removed inaccurate warn log message that said signature verification was not
  turned off

### splinterd
* Add Key Registry REST API resources
* Increase message queue sizes for the admin service's ServiceProcessor.

### splinter-cli
* Remove outdated CLI commands

### Supplychain Example
* Add XoStateDeltaProcessor to Supplychain application authorization handler
* Add route to supplychain REST API to submit batches to scabbard service
* Set six-second timeout for toast notifications in the UI
* Add notification in the UI for newly active supplychains
* Enhance invitation UI and add tabs for viewing sent, received, or all
  invitations
* Fix bug that caused read notifications to not appear as read in the UI
* Fix bug where the Supplychain WebSocket was sending notifications to the UI
  every 3 seconds instead of when a new notification was added

## Changes in Splinter 0.3.1

### Highlights

* Completion of circuit proposal validation, voting, and dynamic circuit creation
* Addition of key generation and management, as well as role-based permissions
* Continued progress towards proposing, accepting, and creating a supplychain in the
  Supplychain example application

### libsplinter

* Add AdminService, with support for:
  * Accepting and verifying votes on circuit proposals
  * Committing approved circuit proposals to SplinterState
* Add notification to be sent to application authorization handlers when a
  circuit is ready
* Update scabbard to properly set up Sabre state by adding admin keys
* Add support for exposing service endpoints using the orchestrator and service
  factories
* Add WebSocketClient for consuming Splinter service events
* Add KeyRegistry trait for managing key information with a StorageKeyRegistry
  implementation, backed by the storage module
* Add KeyPermissionsManager trait for accessing simple, role-based permissions
  using public keys and an insecure AllowAllKeyPermissionManager implementation
* Add SHA512 hash implementation of signing traits, for test cases
* Add Sawtooth-compatible signing trait implementations behind the
  "sawtooth-signing-compat" feature flag.

### splinterd

* Add package metadata and license field to Cargo.toml file
* Add example configuration files, systemd files, and postinst script to Debian
  package
* Reorder internal service startup to ensure that the admin service and
  orchestrator can appropriately connect and start up
* Use SawtoothSecp256k1SignatureVerifier for admin service

### splinter-cli

* Add "splinter-cli admin keygen" command to generate secp256k1 public/private
  key pairs
* Add "splinter-cli admin keyregistry" command to generate a key registry and
  key pairs based on a YAML specification

### Private XO and Private Counter Examples
* Add license field to all Cargo.toml files
* Rename private-xo package to private-xo-service-<version>.deb
* Rename private-counter packages to private-counter-cli-<version>.deb and
  private-counter-service-<version>.deb

### Supplychain Example
* Add package metadata and license field to supplychaind Cargo.toml file
* Add example configs, systemd files, and postinst script to supplychaind Debian
  package; rename package to supplychain-<version>.deb
* Implement notification retrieval using WebSocket subscription and
  notifications endpoints
* Show pending and accepted supplychains in the Supplychain UI
* Add full support for signing CircuitManagementPayloads with the user's
  private key and submitting it to splinterd
* Update supplychaind to specify itself as the scabbard admin and submit the XO
  smart contract when the circuit is ready
* Make various UI enhancements

## Changes in Splinter 0.3.0

### Highlights

* Completion of the two-phase-commit consensus algorithm with deterministic
  coordination
* Continued progress towards dynamically generating circuits, including
  dynamic peering and circuit proposal validation
* Continued progress on the Supplychain example, including UI updates and
  automatic reconnection

### libsplinter

* Add a service orchestration implementation
* Add Scabbard service factory 
* Implement a deterministic two-phase-commit coordinator
* Reorder the commit/reject process for the two-phase-commit coordinator. The
  coordinator now tells proposal manager to commit/reject before broadcasting
  the corresponding message to other verifiers.
* Refactor two-phase-commit complete_coordination. Move the process of 
  finishing the coordination of a proposal in two-phase commit to a single
  function to reduce duplication.
* Implement a two-phase-commit timeout for consensus proposals
* Update the two-phase-commit algorithm to ignore duplicate proposals
* Allow dynamic verifiers for a single instance of two-phase-commit consensus
* Add an Authorization Inquisitor trait for inspecting peer authorization state
* Add the ability to queue messages from unauthorized peers and unpeered nodes
  to the admin service
* Fix an issue that caused the admin service to deadlock when handling proposals
* Add Event Dealers for services to construct websocket endpoints
* Add a subscribe endpoint to Scabbard
* Validate circuit proposals against existing Splinter state
* Update create-circuit notification messages to include durability field

### splinterd

* Log only warning-level messages from Tokio and Hyper
* Improve Splinter component build times
* Add a NoOp registry to handle when a node registry backend is not specified

### Private XO and Private Counter Examples

* Use service IDs as peer node IDs, in order to make them compatible with
  two-phase consensus

### Supplychain Example

* Add server-side WebSocket notifications to the UI 
* Add borders to the Acme UI
* Improve error handling and add reconnects to the Application Authorization
  Handler
* Add a circuit ID and hash to GET /proposals endpoint
* Standardize buttons and forms in the UI
* Improve error formatting in the UI by adding toasts and progress bar spinners
* Change the Supplychain REST API to retrieve node data automatically on startup
* Split the circuit_proposals table into supplychain and supplychain_proposals tables
* Use the [Material elevation strategy](https://material.io/design/color/dark-theme.html)
  for coloring the UI
* Decrease the font size
* Change the UI to redirect users who are not logged in to login page
* Add a dashboard view
* Add an invitation cards view
* Add a button for creating a new supplychain to the UI

## Changes in Splinter 0.2.0

### libsplinter

* Add new consensus API (libsplinter::consensus)
* Add new consensus implementation for N-party, two-phase commit
  (libsplinter::consensus::two_phase)
* Add new service SDK with in-process service implementations
  (libsplinter::service)
* Add initial implementation for Scabbard, a Splinter service for running Sabre
  transactions with two-phase commit between services
(libsplinter::service::scabbard)
* Add REST API SDK (consider this experimental, as the backing implementation
  may change)
* Add new node registry REST API endpoint for providing information about all
  possible nodes in the network, with initial YAML-file backed implementation.
* Add new signing API for verifying and signing messages, with optional
  Ursa-backed implementation (libsplinter::signing, requires the use of
"ursa-compat" feature flag)
* Add MultiTransport for managing multiple transport types and selecting
  connections based on a URI (libsplinter::transport::multi)
* Add ZMQ transport implementation (libsplinter::transport::zmq, requires the
  use of the "zmq-transport" feature flag)
* Add peer authorization callbacks, in order to notify other system entities
  that a peer is fully ready to receive messages


### splinterd

* Add REST API instance to provide node registry API endpoints
* Add CLI parameter --bind for the REST API port
* Add CLI parameters for configuring node registryy; the default registry type
  is "FILE"

### Supplychain Example

* Add supplychain example infrastructure, such as the supplychaind binary, docker
  images, and compose files
* Add Login and Register UI
* Add New Supplychain UI
* Add UI themes for both parties in demo
* Initialize Supplychain database
* Add circuit proposals table
* Initialize Supplychain REST API
* Implement Supplychain REST API authentication routes
* Implement Supplychain REST API create supplychain endpoint
* Implement Supplychain REST API proposals route
* Implement /nodes endpoint in supplychaind
