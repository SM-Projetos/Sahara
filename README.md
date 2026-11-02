# Sahara

Gerenciador de arquivos para Linux, desenvolvido em Rust e Slint.

## Estado atual

A aplicação possui uma janela inicial e navegação básica por diretórios: lista pastas e arquivos, permite abrir subdiretórios e retornar ao diretório anterior. Operações como copiar, mover, renomear e excluir ainda não estão disponíveis.

## Requisitos

- Linux.
- Rust e Cargo (toolchain compatível com Slint 1.18).
- Dependências de desenvolvimento do backend gráfico utilizado pelo Slint.
- Consulte os pré-requisitos oficiais do Slint para o ambiente X11/Wayland utilizado.

## Executar

```bash
cargo run
```

## Testes

```bash
cargo test
```

## Estrutura

- `src/main.rs`: inicialização da janela e integração da navegação.
- `src/filesystem.rs`: leitura e ordenação de entradas de diretório.
- `build.rs`: compilação dos arquivos Slint durante o build.
- `ui/app-window.slint`: janela principal e apresentação da listagem.

## Direção do projeto

O tema escuro é o padrão inicial. O tema claro será considerado em uma etapa posterior.
