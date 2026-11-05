# Changelog

## Não lançado

- Corrigido o dimensionamento horizontal dos layouts internos da listagem para alinhar os nomes à esquerda e os tipos à direita.

- Reformulada a interface do Sahara com paleta azul-marinho e ciano, tipografia Inter incorporada, navegação lateral, barra de endereço e listagem em superfícies arredondadas.
- Adicionados ícones SVG próprios para a identidade e navegação do gerenciador de arquivos.
- Mantida a integração existente com o sistema de arquivos e os callbacks de navegação.
- Corrigida a definição do tamanho inicial da janela Slint com preferred-width e preferred-height, mantendo os limites mínimos de redimensionamento.
- Corrigida a sintaxe das bordas na interface Slint, substituindo a abreviação incompatível pelas propriedades border-width e border-color.
- Tornados funcionais os atalhos laterais para a pasta pessoal, diretórios XDG do usuário e raiz do sistema de arquivos.
- Adicionada a leitura dos caminhos personalizados em user-dirs.dirs, com respeito a XDG_CONFIG_HOME e fallback para os diretórios padrão.
- Adicionados testes para resolução dos caminhos XDG, expansão de $HOME e atalhos desconhecidos.
- Implementada a listagem de arquivos e diretórios do local inicial do usuário.
- Adicionada navegação para subdiretórios e retorno ao diretório anterior.
- Adicionado tratamento de erros de leitura e testes unitários do módulo de sistema de arquivos.
- Removidas do README referências a projetos externos.
- Criada a estrutura inicial em Rust e Slint para Linux.
- Adicionada a primeira composição da janela principal com navegação lateral, barra superior, área central e barra de status.
- Definidos os tokens iniciais do tema escuro.
