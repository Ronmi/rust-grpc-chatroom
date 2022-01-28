一步一步用 rust + grpc (tonic) + postgresql + quasar 作一個簡單的聊天室

# 概要

這是一個簡易的教學兼練習專案，在本專案中將會練習

- 基本 rust 應用
- 使用 tracing 記 log
- 使用 clap (cli argument parser for rust)
- 使用 tonic (grpc for rust)
- 使用 sqlx (db driver for rust)
- grpc unary 與 streaming
- 使用 [cross](https://github.com/cross-rs/cross) 在 linux host 上 cross compile
  - Windows: 只能使用 gnu c lib，不支援 msvc (有工具可以使用 wine 輔助編譯到 msvc target，但不太穩定)
  - RaspberryPi: Raspbian (32bit) 或 [vanilla Debian](https://wiki.debian.org/RaspberryPi) (64bit)
- 基本的 vue3 composition api 及相關 plugin 的使用
- 使用 quasar 快速開發尚稱美觀的前端
- 使用 grpc-web 串接前後端

### 預備知識

- 基本 rust 型別 (primitive types)
- 基本 struct / enum / tuple 的語法
- 如何定義一個基本的 trait
- 如何實作 (impl) 基本的 trait
- 新手等級的 ownership 知識 
  * 能 *大致* 判斷變數生命週期
  * *大致* 知道什麼情況下變數 ownership 會轉移
- 如何引用 crate 及透過 feature 調整 crate 的功能

### 閱讀方式

直接從頭到尾查看每一個 commit 改動的檔案和 commit log。

# 預先準備的工具

一般工具

- 適當的編輯器或 IDE
- 由於本專案是以 linux 為開發環境，如果你只能在 windows 下開發的話必須
  * 能夠理解如何在 windows 下正確執行 commit log 裡提到的 linux 指令
  * **或是** 使用 WSL / VM 一類的替代方案

後端部份

- [安裝](https://rustup.rs) rust toolchain
- 為了方便，也會使用 `cargo-add` 和 `cargo-feature` 兩個輔助程式
  * `cargo install cargo-add`
  * `cargo install cargo-feature`
- docker
  * cross 會使用到
  * 學習 sqlx 的過程中需要一個開發用的 postgresql server，也會用 docker 來跑

前端部份

- 使用套件 (apt/winget/brew/...) 或 [nvm](https://github.com/nvm-sh/nvm) 準備 node 環境
- 安裝 yarn (習慣使用 npm 的話請自行把 yarn 指令代換成等效的 npm)
- 安裝 [quasar cli](https://quasar.dev/quasar-cli/installation)

# License

WTFPL

