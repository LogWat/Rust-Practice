# Chapter 3

**型** 
- `i : int`
- `u : unsigned`
- `f : float`
<br>
(`i`, `u`は8~128までサイズ指定可能. `f`は32 or 64のみ)<br>
- その他に環境に依存したサイズの`isize`と`usize`が存在する
- 文字列型は`str`のみ (標準ライブラリで定義された`String`が使える)
- `str`は変更不可だが`String`は可能 (相互変換が可能)
<br><br>
**頻出する標準ライブラリの型**<br>
`Option`, `Result`, `Vec`, `Box`
- `Option`型は戻り値がないかもしれないとき
- `Result`型は失敗するかもしれないとき
- `?`演算子 : `Ok()`なら値展開, `Err()`ならそれを`return`
---




