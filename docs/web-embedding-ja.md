# ブラウザー版の組み込み

[English](web-embedding.md) | **日本語**

ブラウザー版（`www/`）は、クラウドサービスのファイルのプレビューなど、ほかのページの中に置けます。窓の右上に 2 つのボタンを出せば、組み込んだ側のページが「閉じる」と「ダウンロード」を自分で処理できます。**どちらも既定では出ません**（頼まなければ今の playground と同じ見た目です）。

- **閉じる**：いちばん右のボタン。
- **ダウンロード**（アイコン）：その左のボタン。

ボタンを押しても、エディターは自分では閉じたり保存したりしません。組み込んだ側のページに知らせるだけで、何をするか（ダイアログを閉じる、自分の保存先に保存する、など）はページが決めます。

## ボタンを出す

エディターの URL のパラメーター：

| パラメーター | 意味 |
|---|---|
| `view=1` | 読み取り専用モードで起動する（`0`・`false` で編集モード）。メニューバーとツールバーは `view=1` でも既定で表示される。 |
| `menuBar=0` | メニューバーを非表示にする（既定は `menuBar=1` 表示）。 |
| `toolBar=0` | ツールバーを非表示にする（既定は `toolBar=1` 表示）。 |
| `close=1` | 閉じるを出す（`0`・`false` で出さない）。 |
| `download=1` | ダウンロードを出す。 |
| `origin=https://app.example.com` | 親の窓へのメッセージの送り先の origin（iframe のときだけ。下を参照）。 |

JavaScript から渡す場合は、`main.js` より**前**に実行されるスクリプトで：

```html
<script>
  window.ibisOfficeOptions = {
    view: true,
    menuBar: false,
    toolBar: false,
    close: true,
    download: true,
    targetOrigin: 'https://app.example.com', // iframe のときだけ
    eventTarget: document.getElementById('preview'), // 省略可。既定は window
  };
</script>
<script type="module" src="main.js"></script>
```

JavaScript で渡した値は、URL の同じ指定より優先されます。

## ページが受け取るもの

### 同じページに埋め込んだ場合（iframe のときも、これに加えて）

`window`（または `eventTarget`）に `CustomEvent` が届きます。

```js
window.addEventListener('ibisoffice:close', () => { /* プレビューを閉じる */ });
window.addEventListener('ibisoffice:download', (e) => {
  const { name, bytes, blob } = e.detail; // name: ファイル名、bytes: Uint8Array、blob: Blob
});
```

`download` には、**今の内容**（編集を含む）が、そのファイルの形式のまま入っています（ファイル > 保存 で書かれるのと同じバイト列）。

### iframe の場合

親の窓にはメッセージも届きます。

```js
window.addEventListener('message', (e) => {
  if (e.origin !== 'https://ibisoffice.example.com') return; // エディター側の origin
  const m = e.data;
  if (m?.source !== 'ibisoffice') return;
  if (m.type === 'close') { /* … */ }
  if (m.type === 'download') { /* m.name: 文字列、m.bytes: ArrayBuffer */ }
});
```

メッセージは `origin=` / `targetOrigin` で指定した origin に送ります。指定が無ければ `document.referrer`（組み込んだページ）の origin を使います。どちらも分からないときは**何も送りません**（警告をログに出します）。知らないページにファイルを送らないためです。どのページにも届けたいときだけ、`origin=*`（または `targetOrigin: '*'`）を指定してください。

## 補足

- デスクトップ版には影響しません。
- ボタンの文字は、エディターの言語（英語・日本語）に従います。
