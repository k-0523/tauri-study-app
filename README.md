## 概要
- 勉強記録のようなもの
- Markdown形式で入力できる
- 右側と左側で表示するものを変える
  - 右側
    - 今日記録するエリア
  - 左側
    - 5日前に記録したもの

### なぜ5日前？
**脳科学的に5日前のものを復習しながら勉強するのが効果的っぽいため**

![](https://user-images.githubusercontent.com/50039010/226815292-cf15662b-d1c0-4642-a742-d3c97f15e3ea.png)

↓Markdownプレビューモード
![](https://user-images.githubusercontent.com/50039010/226815465-737dfe9d-f8f0-4fd1-bfa7-aed268479465.png)

# 自分用メモ
## 環境構築
### tauri install
```cargo install tauri-cli```

### yarn install
```npm install -g yarn```

### sqlx install
```cargo install --version=<version> sqlx-cli```

### create db
```sqlx db create --database-url "sqlite:<name>.db"```

### add migration file
```sqlx migrate add <name>```

### run migration
```sqlx migrate run --database-url "sqlite:<name>.db"```

## 実行方法
```yarn tauri dev```
