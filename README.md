# Game
André Tessmer, João Ramos, Vinicius Rossoni

# Triangulo
Estrutura do Triangulo, lógica de rotação, translação...
![triangulo](prints/triangle.png)

![triangulo](prints/triangle-rotate.png)


# Main
Estruturação e lógica de funcionamento do jogo

![main](prints/main-vars.png)
- `triangle` segura a instancia do nosso triangulo
- `spawner` segura a instancia do nosso spawner, responsavel por spawnar ou não novos inimigos
- `bullets` segura os tiros que estão/vão ser desenhados na tela
- `enemies` segura os inimigos que estão/vão ser desenhados na tela.

depois disso, iniciamos o looping principal
![main](prints/main-loop.png)


![main](prints/main-enemy-collision.png)
![main](prints/main-bullet-collision.png)


dentro do looping principal.

<!-- 

## Enemy damage
![main](prints/enemy-takedamage.png)

## Triangle struct
![triangle struct](prints/triangle.png)
v1, v2 e v3 - representam os vértices do triangulo

## Triangle rotate
![triangle rotate](prints/triangle-rotate.png)
recebe quantos graus deve rotacionar, utilizando multiplicação de matriz em relação ao centro do triangulo.

## Triangle translate
![triangle rotate](prints/triangle-translate.png)
dependendo da direção da movimentação, alteramos x ou y de todos os vértices de acordo. -->

