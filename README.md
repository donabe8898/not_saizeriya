# not_saizeriya

存在しないサイゼリヤのメニューを考えてくれる


# Usage

```
NAME:
    not_saizeriya

COMMAND:
    /generating     存在しないかもしれないメニューを出力
    /lots           1000円ガチャ
```

# Install

- dotenv.sampleを参考に.envにTOKENを入れる

- ビルド

# Build

```
cargo build --release
```

# Other

- menu.jsonの中身は自由に追加しても動くと思う. ありそうでなさそうなメニューを追加してみてくれ

- menu.jsonをtarget/releaseフォルダに入れないとパニック起こすみたいです

## Issues

- [ ] メニューの追加

- [ ] おまけ機能の実装

    - [x] 1000円ガチャ

    - [ ] 店舗検索

- [x] Warningの掃除

- [x] モジュール分割

# Author

donabe8898

# License

MIT License