# not_saizeriya

存在しないサイゼリヤのメニューを考えてくれる

# 使い方

- dotenv.sampleを参考に.envにTOKENを入れる

- あとは自鯖にBotを導入するだけ

## コマンド

Discordのテキストチャンネルで`/generating`というスラッシュコマンドを打つ

# ビルド

```
cargo build --release
```

# その他

- menu.jsonの中身は自由に追加しても動くと思う. ありそうでなさそうなメニューを追加してみてくれ

- menu.jsonをtarget/releaseフォルダに入れないとパニック起こすみたいです
# コントリビュート

## Issues

- [ ] メニューの追加

- [ ] おまけ機能の実装

    - [ ] 1000円ガチャ

    - [ ] 店舗検索

- [x] Warningの掃除

- [x] モジュール分割
# License

MIT License