# optruna

optrunaは、Optunaによるハイパーパラメータ最適化を支援するために開発されたRustライブラリです。

## 特徴

- `param!`マクロを使用してハイパーパラメータを定義します。各パラメータは名前、型、初期値、最小値、最大値、ステップ幅を持ちます。
- ビルド時に環境変数を読み込むため定数として宣言できます。
- フィーチャーフラグを使用して動作を変更します。`optimize`フラグがtrueの場合、ビルド時に環境変数から値を読み取り、それを定数として宣言します。`optimize`フラグがfalseの場合、デフォルト値がそのまま使用されます。
- `py`フラグが有効な場合、ビルド時に`param!`マクロ内のハイパーパラメータを最適化するPythonコードを生成します。

## 使用方法
`Cargo.toml`に以下を追加します:
```toml
[dependencies]
optruna = { git = "https://github.com/qropa/optruna.git" }

[features]
optruna = [ "optruna/optimize" ]
py = [ "optruna/py", "optruna/optimize" ]
```
crates.ioにはいずれ公開すると思います。

プロジェクトのルートに`build.rs`を作成し、以下を記述します:
```rust:build.rs
fn main() {
    println!("cargo:rerun-if-changed={}", "./non_existent_file.rs")
}
```

ハイパーパラメータの定義は以下のように行います：
```
param! {
    variable_name: type = default_value, (min_value, max_value, step_size),
}
```
例えば、以下のように記述します：
```rust
param! {
    start_temp: f64 = 20000.0, (10.0, 100000.0, 1.0),
    end_temp: f64 = 30.0, (0.001, 100.0, 0.001),
}
```
`param!`マクロは１回のみ使用してください。

optunaでの最適化の前に、フィーチャーフラグ`py`を立ててビルドしてください。以下のような`optruna.py`が作成されます。
```python

import optuna
import os
import subprocess
        
# name, type, default, (min, max, step)
params = [
['start_temp', 'f64', 20000.0, (10.0, 100000.0, 1.0)],['end_temp', 'f64', 30.0, (0.001, 1000, 0.001)],
]

def set_params_and_build(trial: optuna.trial.Trial):
    for [name, type, default, (min, max, step)] in params:
        if type in ['u8', 'u16', 'u32', 'u64', 'usize', 'i8', 'i16', 'i32', 'i64', 'isize']:
            value = trial.suggest_int(name, min, max, step=step)
        elif type in ['f32', 'f64']:
            value = trial.suggest_float(name, min, max, step=step)
        os.environ[name] = str(value)
        
    subprocess.run(["cargo", "build", "--release", "--features", "optruna"], stderr=subprocess.DEVNULL)
    
def set_inital_trial_params(study: optuna.study.Study):
    study.enqueue_trial({param[0]: param[2] for param in params})
```
`set_params_and_build`関数はsuggestされたハイパーパラメータを環境変数に設定し、再ビルドします。trialの最初に使用してください。
`set_inital_trial_params`関数はデフォルト値を一番最初のtrialに使用するよう設定します。`optuna.create_study`後に使用してください。

## Todo
- クレートを使用できない環境で利用可能な形式にするコマンドの作成
- `A+B+C=100`のような割合の最適化への対応

## ライセンス
MITライセンスの下で公開されています。詳細はLICENSEファイルをご覧ください。
