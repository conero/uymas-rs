## hero

> 2023年9月13日 星期三









支持工具

- [x] config 配置文件管理
- [x] log 日志支持





**条件编译**

- log    启动日志
- dev   系统开发模式

```shell
# 启用日志
cargo build -p hero --features log

# 指定多个
cargo build -p hero --features log --features dev
cargo build -p hero --features 'dev,log'
```

