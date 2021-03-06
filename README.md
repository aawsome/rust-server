# rust-server
A REST server built in rust for use with restic

Works pretty similar to [rest-server](https://github.com/restic/rest-server). Most features are already implemented.

## Dependencies
Is built using [tide](https://github.com/http-rs/tide), [tide-rustls](https://github.com/http-rs/tide-rustls) and
[tide-http-auth](https://github.com/chrisdickinson/tide-http-auth).

tide-http-auth is embedded as 0.3.0 does not support tide 0.16.0, see [issue](https://github.com/chrisdickinson/tide-http-auth/issues/6) 

## Missing features / TODOs:

- Tests
- CI pipeline
- An option `max-size`
- support for prometheus
- Storage part: Error handling etc


## Additional feature:
Allows to give ACLs im TOML format, use option `--acl`

Example TOML file:

    # default sets ACL for the repo without explicit path (and for the repo under path "default", if exists)
    [default]
    alex = "Read"
    admin = "Modify"
    
    [alex]
    alex = "Modify"
    bob = "Append"
