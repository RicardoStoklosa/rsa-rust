Como rodar:
para versao otimizada
```
$ cargo build --release
$ ./target/release/rsa

```
ou apenas

```
$ cargo run
```

commandos:
 ```
 Commands:
        genkey
                Generate the public and private key
        encrypt [file path] [key exp] [key n]
                Encrypt a file
        decrypt [file path] [key exp] [key n]
                Decrypt a file
        help
```