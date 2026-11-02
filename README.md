# Sahara

Gerenciador de arquivos para Linux, desenvolvido em Rust e Slint.

## Estado atual

O repositório contém a estrutura inicial da aplicação e a primeira composição visual da janela principal. Esta versão é um protótipo de interface: ainda não navega pelo sistema de arquivos nem executa operações sobre arquivos.

## Requisitos

- Linux.
- Rust e Cargo (toolchain compatível com Slint 1.18).
- Dependências de desenvolvimento do backend gráfico utilizado pelo Slint.
- Consulte os pré-requisitos oficiais do Slint para o ambiente X11/Wayland utilizado.

## Executar

```bash
cargo run
```

## Estrutura

- `src/main.rs`: ponto de entrada e inicialização da janela.
- `build.rs`: compilação dos arquivos Slint durante o build.
- `ui/app-window.slint`: janela principal, layout inicial e tokens do tema escuro.

## Direção do projeto

A interface segue a identidade visual do Gamefic: superfícies escuras, roxo como cor de destaque e tipografia compacta. O tema escuro é o padrão inicial; o tema claro será considerado em uma etapa posterior.

A lógica de domínio e o acesso ao sistema de arquivos serão implementados em módulos Rust separados da interface.
