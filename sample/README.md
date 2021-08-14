### 設定方法
main.rs以外でコーディングし、実行する場合はCargo.tomlで以下のように引数設定を追加する必要がある。<br>
Cargo.toml
```
[[bin]]
name = "sample"
path = "src/sample.rs"
```
### 実行方法
```
cargo run --bin sample
```
### 実行結果
### ・_hello_rust1
<img width="298" alt="hello_rust1" src="https://user-images.githubusercontent.com/76214131/129443722-42063819-ec78-41d6-8081-eb0c8ce76b75.png">

### ・_hello_rust2
<img width="299" alt="hello_rust2" src="https://user-images.githubusercontent.com/76214131/129443721-8c54a934-3a59-49c0-b0b9-96f21d50f1ae.png">

### ・_image_load
<img width="295" alt="image_load" src="https://user-images.githubusercontent.com/76214131/129443723-eb82bf5e-36a0-4978-8f6b-df470e6563eb.png">
