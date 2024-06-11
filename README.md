# Descrição

**Disciplina**: Unidade Curricular de Análise de Algoritmos e Estruturas de Dados.

**Professor**: Professor Mestre Alexandre de Oliveira (Montanha).

## Enunciado

**Data de Entrega**: 06/06/2024

**Exercício de Otimização de Planejamento e Controle de Produção**

Neste exercício, vocês irão desenvolver um sistema de planejamento e controle de produção. O objetivo é otimizar a produção de uma fábrica que produz diversos produtos, cada um composto por diferentes subconjuntos de matérias-primas. Alguns produtos podem compartilhar as mesmas matérias-primas, enquanto outros não.

**Entradas**:
- Lista de produtos a serem produzidos.
- Lista de matérias-primas necessárias para cada produto, incluindo as quantidades necessárias.
- Data de entrega para cada pedido de produto.

**Saídas**:
- Lista de pedidos de compra de matérias-primas, incluindo as datas de pedido, as quantidades totais e os pedidos mínimos.

**Premissas**:
1. As matérias-primas devem ficar o menor tempo possível em estoque, ou seja, devem ser compradas o mais próximo do uso.
2. Comprar lotes de matérias-primas que evitem ao máximo o desperdício, ou seja, que fiquem o mais próximo da quantidade a ser utilizada na produção.

**Restrições**:
1. A capacidade de produção é limitada.
2. As matérias-primas podem não estar sempre disponíveis para compra.
3. O tempo necessário para a entrega das matérias-primas após a compra pode variar.
4. Existe um limite para a quantidade de matérias-primas que podem ser armazenadas de uma vez.
5. A qualidade das matérias-primas pode variar.
6. O custo das matérias-primas pode variar.
7. A quantidade de mão de obra disponível pode ser limitada.
8. As práticas de produção devem estar em conformidade com as regulamentações ambientais.


## Entrega

O desafio é desenvolver um algoritmo, em Rust, que leve em consideração todas essas entradas, saídas, premissas e restrições para otimizar o processo de produção.

Utilize o projeto base disponível no repositório do GitHub: <https://github.com/alexmontanha/otimizando_pcp>.

Considere os arquivos `json` disponíveis no repositório como exemplos de entrada.

O módulo `pcp` deve conter a implementação do algoritmo de otimização. Lá você encontra a implementação de carga das matérias-primas, faça a implementação da carga dos arquivos de produto e pedidos.

O código deve ser entregue via GitHub, em um repositório privado, com acesso concedido ao professor Montanha (alexmontanha).

Além disso, deve ser entregue um arquivo README.md, contendo as seguintes informações:

1)	Apresentação do Problema
Apresentar de forma sintética, a motivação do trabalho, qual é o problema abordado e o objetivo da pesquisa;

2)	Complexidade do Problema
Utilizando a análise de complexidade de problemas, classificar o problema quanto à sua complexidade de resolução, se P. NP ou NP-Completo, justificando sua resposta, com base na teoria e exemplos;

3)	Algoritmos conhecidos
Pesquisar os algoritmos conhecidos que resolvam o problema em questão, tentando, ao máximo, colocar características de cada um;

4)	Algoritmo escolhido e implementação
Apontar o algoritmo escolhido, indicando a fonte de onde tirou o mesmo, implementar e versionar no GitHub;

5)	Complexidade do algoritmo escolhido
Utilizando o método de contagem simples, classificar com a classificação Big-O, o algoritmo, destacando o trecho de maior complexidade;

6)	Paradigma e Estratégia do algoritmo escolhido
Evidenciar, analisar e apresentar quais estratégias o algoritmo escolhido utiliza;

7)	Comparação com, pelo menos mais um algoritmo
Selecionar outro algoritmo, que não o escolhido e comparar em complexidade e paradigma;

8)	Linguagem
Explique como o Rust foi utilizado no desenvolvimento do algoritmo;

 
Apresentação
1)	Capa
2)	Resumo
3)	O Problema
4)	O Algoritmo
5)	Apresentar as complexidades Big-O com exemplos
6)	Apresentar a escolha do algoritmo
7)	Apresentar o código
8)	Lições aprendidas na UC

#Trabalho-Prof.Montanha
