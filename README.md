# Sahara

Gerenciador de arquivos para Linux, desenvolvido em Rust e Slint.

## Interface

A interface utiliza uma identidade visual escura em azul-marinho, com superfícies em gradiente, detalhes em ciano, bordas discretas e cantos arredondados. A tipografia Inter é incorporada ao aplicativo para manter a apresentação consistente entre sistemas. Os arquivos da fonte e sua licença SIL Open Font License 1.1 estão em assets/fonts/.

A janela é redimensionável e contém uma navegação lateral, barra de endereço, área de listagem e barra de status. O tema escuro é o padrão.

## Estado atual

A aplicação lista arquivos e diretórios, permite abrir subdiretórios, retornar ao diretório anterior e acessar os locais XDG do usuário e a raiz do sistema de arquivos. Os diretórios XDG são lidos de user-dirs.dirs, respeitando XDG_CONFIG_HOME; quando uma configuração não está disponível, são usados os nomes padrão. Locais ausentes ou sem permissão de leitura apresentam uma mensagem de erro.

Abertura de arquivos, copiar, mover, renomear e excluir ainda não estão disponíveis.

## Requisitos

- Linux.
- Rust e Cargo (toolchain compatível com Slint 1.18).
- Dependências de desenvolvimento do backend gráfico utilizado pelo Slint.
- Consulte os pré-requisitos oficiais do Slint para o ambiente X11/Wayland utilizado.

## Executar

    cargo run

## Testes

    cargo test

## Estrutura

- src/main.rs: inicialização da janela e integração da navegação.
- src/filesystem.rs: leitura, ordenação e resolução dos diretórios do usuário.
- build.rs: compilação dos arquivos Slint durante o build.
- ui/app-window.slint: janela principal e componentes da interface.
- assets/icons/: ícones SVG da aplicação.
- assets/fonts/: variantes da fonte Inter e sua licença.
