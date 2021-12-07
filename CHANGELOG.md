
0.10.1 / 2021-11-22
===================

  * Merge pull request #16 from @Norio4 / add-perform_at
    * Add public function perform_at() for Client
  * Cargo lint fixes
  * README: fix last version

0.10.0 / 2021-10-27
===================

  * Merge pull request #15 from @Norio4 / add_perform_in
    * Add public function perform_in() for Client
  * Update badge and others minor updates
  * Merge pull request #14 from spk/dependabot/add-v2-config-file
    * Upgrade to GitHub-native Dependabot

0.9.1 / 2021-04-05
==================

  * Update r2d2_redis requirement from 0.13 to 0.14
  * Add github actions
  * Cargo fmt
  * Update rand to 0.8
  * Cargo fmt

0.9.0 / 2021-01-06
==================

  * Merge pull request #12 from @liaden / patch-1
    * Make ClientError public
  * Cargo.toml: Rust 2018
  * Cargo.toml: Makefile => Justfile
  * Switch Makefile to Justfile
  * Update README [ci skip]
  * Implement std::error::Error for ClientError
  * Remove deprecated Error::description and Error::cause
  * Update r2d2_redis to 0.13
  * Rust edition 2018 fix
  * Fix bench for simple push
  * Use of deprecated item 'try': use the '?' operator instead

0.8.6 / 2019-10-21
==================

  * Merge pull request #5 from @jkcclemens / update
    * chore: update r2d2_redis

0.8.5 / 2019-09-07
==================

  * Update r2d2_redis and use exported r2d2 from it
  * Use exported redis from r2d2_redis
  * Update rand to 0.7

0.8.4 / 2019-06-19
==================

  * README,LICENSE: bump year and fix travis badge
  * travis: remove preview component
  * Merge pull request #4 from @jkcclemens / master
    * chore: update redis and r2d2_redis

0.8.3 / 2018-12-01
==================

  * Update rand to 0.6
  * Remove clippy from Cargo.toml
  * README remove experimental status badge

0.8.2 / 2018-08-18
==================

  * Merge pull request #3 from @jkcclemens
    * chore: update redis and r2d2_redis

0.8.1 / 2018-08-17
==================

  * Fix fmt
  * Fix rust fmt ci check
  * Update rand to 0.5
  * Clippy allow failures on travis
  * clippy fix redundant field names in struct initialization
  * Add Dependency status badge
  * Update rand to 0.4
  * Update year [ci skip]

0.8.0 / 2018-01-21
==================

  * Fix fmt
  * Update r2d2 and r2d2_redis
  * Fix clippy warnings and rust fmt
  * README: use master as ci badge
  * Fix clippy warnings
  * Add code formatting check

0.7.0 / 2017-06-12
==================

  * Always use last clippy version
  * Update serde to 1.0
  * Add the `html_root_url` attribute to the crate
  * Less strict deps
  * Add REDIS_URL_ENV const
  * Add clippy check

0.6.0 / 2017-02-08
==================

  * Update redis and r2d2_redis

0.5.0 / 2017-02-01
==================

  * Update to serde 0.9

0.4.0 / 2017-01-02
==================

  * Merge pull request #1 from @ttdonovan feature/push-bulk
    * pub fn push_bulk Client

0.3.0 / 2016-09-10
==================

  * Use opaque struct for ClientError
  * Better error handling for create_redis_pool fn

0.2.2 / 2016-08-20
==================

  * Fix error format display on Client

0.2.1 / 2016-08-20
==================

  * Update redis,r2d2 and better error handling

0.2.0 / 2016-08-20
==================

  * Replace rustc-serialize by serde
  * Use std::time instead of time crate

0.1.3 / 2016-06-05
==================

  * Expose RedisPooledConnection

0.1.2 / 2016-05-30
==================

  * Add basic Makefile and benches
  * Use multi pipe and move get connection to new

0.1.1 / 2016-05-22
==================

  * pub use ClientOpts

0.1.0 / 2016-05-22
==================

  * Initial release
