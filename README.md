# RoutedHub 主页
## 简介
本项目是RoutedHub的主页前端项目，使用[Yew](https://yew.rs/)框架编写，CSS采用[Tailwind CSS](https://tailwindcss.com/)实现。

## 配置环境

### Rust

首先需要配置Rust环境，目前使用的是2021版。推荐使用rustup安装

### Tailwind Cli

```shell
$ npm i -g tailwindcss
```

或

```shell
$ yarn global add tailwindcss
```

检查是否安装成功：

```shell
$ tailwindcss --help
```


### Bonnie

[Bonnie](https://github.com/arctic-hen7/bonnie)类似于Make，可以自动化编排编译过程。

安装：
    
```shell   
$ cargo install bonnie
```

## 编译

```shell
$ bonnie build frontend
```

调试运行
    
```shell
$ bonnie run frontend
```

编译Tailwind
        
```shell
$ bonnie setup
```


