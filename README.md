# not_saizeriya

存在しないサイゼリヤのメニューを考えてくれる


# Usage

```
NAME:
    not_saizeriya

COMMAND:
    /help           ヘルプの表示

    /generating     無いメニューを生成
    /lots [arg]     1000円ガチャ
        ├ true      お酒ありで出力
        └ false     お酒なしで出力

    /info [subcmd]  いろんな情報を提供
        ├ version   Botのバージョン,アップデート情報
        └ pacman    pacman -Syu
```

# Install

- dotenv.sampleを参考に.envにTOKENを入れる

- ビルド

# Build

```
cargo build --release
```

# Other

- menu.jsonの中身は自由に追加しても動く.

- menu.json, menu_words.json, help.txt, update.txtをtarget/releaseフォルダに入れる.

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