# Dotenv Aggregator
Dotenv Aggregator는 dotenv를 새로운 template으로 update를 보다 쉽게 하기위해 만들었습니다.

Dotenv Aggregator is migration tool for dotenv. You can migrate already existed .env file with new .env.template!

## Installation
### Required
 - rust
 <br>

``` bash
cargo install dotenv-aggregator --version 0.0.1-alpha
```

## Usage

``` bash
dotenv-aggregator <target> <from>
```



## Documentation

Target File에 From File을 뒤집어 씌웁니다.

예를 들어, 다음과 같이 A.env B.env가 있다고 했을 때,
```
# A.env
A=1
B=2
```

```
# B.env
A=2
C=3
```

이 때, `dotenv-aggregator A.env B.env` 를 입력하면 output은 다음과 같은 형태로 출력됩니다.
(현재는 실험 단계이며, output은 `test`라는 이름으로만 나오는 부분 참고 부탁드립니다.)
```
# A.env
A=2
B=2
```

## License

MIT License

