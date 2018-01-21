
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
