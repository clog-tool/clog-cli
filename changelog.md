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
