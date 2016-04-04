<a name="0.9.1"></a>
## 0.9.1 (2015-10-25)


<a name="0.8.2"></a>
### 0.8.2 (2015-07-02)


#### Bug Fixes

* **CLI**  fixes a bug when passing config at command line ([bee2505c](https://github.com/thoughtram/clog/commit/bee2505c20469b48916432c913e15397ef4bb62e))



<a name="0.8.1"></a>
## 0.8.1 (2015-07-02)


#### Bug Fixes

* **Cargo.toml**  patch up version number ([017066fa](https://github.com/thoughtram/clog/commit/017066fa5fd63d33b885cacfc6500b67edb68d2d))



<a name="0.8.0"></a>
## 0.8.0 (2015-07-02)


#### Features

*   adds automatic changelog writing for lib ([a790b703](https://github.com/thoughtram/clog/commit/a790b7034119f49ddff3464b2a3fb81ac20c6744))
*   improves non-cli lib use and configuration ([c0e969c3](https://github.com/thoughtram/clog/commit/c0e969c335bebbe1aa79269c5e05680b09f77fcf))
*   split bin from lib ([218f1d04](https://github.com/thoughtram/clog/commit/218f1d047309a66c3cd132e762df3b2b9f22a5f7))

#### Bug Fixes

*   fixes bug when using current working dir and default config ([f90f0f0e](https://github.com/thoughtram/clog/commit/f90f0f0ebf567f59469f64186440231cf84c130e))
*   fixes bug with non-cli configuration ([731c71d3](https://github.com/thoughtram/clog/commit/731c71d39aab95cc8667e8a42218f821425c3d88))
* **Rust Nightly**  fixes to compile on nightly again ([d2f9afe4](https://github.com/thoughtram/clog/commit/d2f9afe41734ee3a62e645fc3510c1d9c4a5e72b))



<a name="0.7.0"></a>
## 0.7.0 (2015-05-29)


#### Features

* **repoflavor**  allows specifying the flavor of link to generate ([b3dd5762](https://github.com/thoughtram/clog/commit/b3dd5762544f05b7ed7da5dc67d9d17ba80332ff))
* **sections**  allows users to use empty components in commit subjects ([71b32ee6](https://github.com/thoughtram/clog/commit/71b32ee6776d9f05771ee884b12c25d98a7eb59f), closes [#2](https://github.com/thoughtram/clog/issues/2))
* **version headers**  distinguishes between minor and patch version headers ([c5c02764](https://github.com/thoughtram/clog/commit/c5c02764bc715dbf6cc758f7b628b29036ad8e80), closes [#5](https://github.com/thoughtram/clog/issues/5))



<a name="v0.6.0"></a>
## v0.6.0 (2015-05-05)


#### Bug Fixes

* **--from**  fixes a bug where --from is ignored if from-latest-tag is true .clog.toml ([8e195799](https://github.com/thoughtram/clog/commit/8e1957995788e241626cd620f1117b75d8bff3ce))

#### Features

* **changelog.md**  allows specifying custom file for changelog ([7fa505aa](https://github.com/thoughtram/clog/commit/7fa505aa918832fb2301570c365201cb93ea84ac), closes [#34](https://github.com/thoughtram/clog/issues/34))



<a name="v0.5.0"></a>
## v0.5.0 (2015-05-04)


#### Bug Fixes

* **autoversion**  correctly follow previous version's use of 'v', i.e. v1.2.3 vs 1.2.3 ([c6950fe5](https://github.com/thoughtram/clog/commit/c6950fe5baee959ee18ea2a07e3b5a8cbe5e3716))

#### Features

* **CustomSections**  allows addition of custom sections and aliases ([0fb0d5b7](https://github.com/thoughtram/clog/commit/0fb0d5b7e5189ce60b1effdb04ee7ac50b987ef4), closes [#31](https://github.com/thoughtram/clog/issues/31))



<a name="v0.4.0"></a>
## 0.4.0 (2015-04-26)


#### Features

* **aliases**  implement aliases for commit types ([44f7d493](https://github.com/thoughtram/clog/commit/44f7d49389cfae40ad09033c6deaf54852d75d70), closes [#3](https://github.com/thoughtram/clog/issues/3))
* **clog**  auto increment version with --major, --minor, or --patch ([329e119a](https://github.com/thoughtram/clog/commit/329e119a0326e54cdf4d669b58f835ebb111d47f), closes [#19](https://github.com/thoughtram/clog/issues/19))
* **build**  add travis-ci support ([671171bb](https://github.com/thoughtram/clog/commit/671171bbda6d3647e0118695b5282b3ed27270ee))
* **config**  support .clog.toml configuration file ([bb3072b8](https://github.com/thoughtram/clog/commit/bb3072b80416fb3c874845360e9d46704cd53c79))



<a name="0.3.2"></a>
## 0.3.2 (2015-04-08)


#### Bug Fixes

* **main.rs**  changed the help message of the 'to' in order to reflect default value ([048d6418](https://github.com/thoughtram/clog/commit/048d6418b655153facc9dcbbf93b1ada9d0f3b42))



<a name="0.3.1"></a>
## 0.3.1 (2015-04-01)


#### Bug Fixes

* **main**
  *  create changelog.md if it doesn't exist ([2f081dd5](https://github.com/thoughtram/clog/commit/2f081dd51f3205d96d0dae1d4818944c8e930318))
  *  make --subtitle optional ([e0c31534](https://github.com/thoughtram/clog/commit/e0c31534000cef4b8e64e382ba725ebd0dbfe7b3))
  *  make --repository optional ([df6cd68e](https://github.com/thoughtram/clog/commit/df6cd68ef3635d57f5cc08d7f57c12d3a3bf3e38))
  *  make --from optional ([86dd25d4](https://github.com/thoughtram/clog/commit/86dd25d477c27b1f2bd6889368f4a28c66edb6b0))
  *  make --to param optional ([bcb7d425](https://github.com/thoughtram/clog/commit/bcb7d425b4b4524bf548d3a3332dcd53beef0ecf))
* **README.md**  update try instructions ([7a90b31f](https://github.com/thoughtram/clog/commit/7a90b31fb5d4ba667d6dcc7c433ed31b1427b716))



## 0.3.0 (2015-03-31)


#### Bug Fixes


* **docopts.rs**  updated Cargo.lock to use the latest docopt.rs ([315ad76d](https://github.com/thoughtram/clog/commit/315ad76d238858a7bcae305dc627eb20b9b2c3c0))
* *****  upgrade to latest Rust ([d230dd8d](https://github.com/thoughtram/clog/commit/d230dd8d323cc0edaebaa55e6a4b0e6a93e527ef))
* **git**  get_mut_ref() was deprecated, use as_mut().unwrap() ([f073d69a](https://github.com/thoughtram/clog/commit/f073d69a0bc6c3c87fee4375dfc49211fdab6b44))
* **section_builder**  find_or_insert was deprecated, only some verbose workarounds available yet ([88ccacd5](https://github.com/thoughtram/clog/commit/88ccacd5bd559e8af996f3e67a5d58fe31b3f87c))
* **cargo**  track working master branch ([9496bc8b](https://github.com/thoughtram/clog/commit/9496bc8b7752d248c1781fbcbff0b969a10defe0))

#### Features


* **main**
  *  implement fallback for --setversion ([d276786a](https://github.com/thoughtram/clog/commit/d276786a383813337a82b0a1f5e72333443517ab))
  *  include links to closed issues ([602fb29e](https://github.com/thoughtram/clog/commit/602fb29e90aa2c87b14c395b11b3bbbf7ca0a69b))
* **log_writer**  include anchor in header ([01645092](https://github.com/thoughtram/clog/commit/01645092893fcfb10d22c76624ce8ca493bf282d))



## 0.2.0 (2014-09-23)


#### Bug Fixes


* **cargo**  temporally switch to docopt fork ([6eb6128d](https://github.com/thoughtram/clog/commit/6eb6128d3d8a0c894c23a0e6c1fe6f2baa1d6464))
* **main**  don't fail if changelog.md does not exist ([47e9250e](https://github.com/thoughtram/clog/commit/47e9250ec15dd5a7e81804b05c2ae50b79bc9ce8))

#### Features


* **readme**  cover new commands ([c7a1f1c7](https://github.com/thoughtram/clog/commit/c7a1f1c7e71d49bc5b1e43848a82a9697aeacd8f))
* **main**
  *  always prepend rather than append ([31c4d465](https://github.com/thoughtram/clog/commit/31c4d465285c4baa2a9f86fa66da5944ebbff49a))
  *  print out notification ([81389b98](https://github.com/thoughtram/clog/commit/81389b980702684275789a7afd23425eeac92ba7))
  *  always append to file ([b880ba1c](https://github.com/thoughtram/clog/commit/b880ba1c9d93aaa8f08a1ee7b3b88aaa819be133))
  *  implement --from-latest-tag flag ([dfd420fc](https://github.com/thoughtram/clog/commit/dfd420fcee1695d2498ca2f1cc02d55c8e9503e9))
  *  implement short param -r for repository ([192ae014](https://github.com/thoughtram/clog/commit/192ae0144eafe9c06e138c7609fd95c7d0521cd4))
  *  give better help ([ca0236b6](https://github.com/thoughtram/clog/commit/ca0236b6243994f3c2d1c8eb2ff6a7e9696bb63c))



## 0.1.0 crazy-dog (2014-09-14)


#### Bug Fixes


* **log_writer**  write fallback links ([e7ea409e](https://github.com/thoughtram/clog/commit/e7ea409e0daca6fc6e95a6c965876813e93ce685))

#### Features


* **main**
  *  write proper header ([c667e1e8](https://github.com/thoughtram/clog/commit/c667e1e889d7c875a322e6431637b4679c48874e))
  *  parse --repository option from CLI ([12a85460](https://github.com/thoughtram/clog/commit/12a85460a3149a9dea6510e9ee9bb648960be217))
  *  add basic functionality ([05199ce1](https://github.com/thoughtram/clog/commit/05199ce128315f03204a3fc4722440a753bfdccc))
* **Readme**  describe basic usage ([e85854f3](https://github.com/thoughtram/clog/commit/e85854f3840e8b77b0a385200bb17ea0ea6b75ab))
