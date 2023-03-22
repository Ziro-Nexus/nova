## Zirox Translator 🤖 ##

Zirox Translator es un intérprete basado en Rust para el lenguaje de programación Zirox. El intérprete utiliza las librerías syn y quote para generar un árbol de sintaxis abstracta (AST) para el código Zirox de entrada.

## Características 🚀 ##
La versión actual de Zirox Translator puede:

- Analizar declaraciones de variables, como local num = 20.
- Generar un AST utilizando AllocatorGrammar.
- Usar la macro ast_tree! para generar un AST para una sola línea de código.
- Usar la macro print_tree! para generar un AST para varias líneas de código y mostrarlas en la consola.

## Uso 📝 ##
Para utilizar Zirox Translator, se puede ejecutar el binario zirox_translator y proporcionarle código Zirox para interpretar. Por ejemplo:

`$ echo "local num = 20" | zirox_translator`


Esto generará un AST para el código de entrada y lo mostrará en la consola.

## Trabajo futuro 🔮 ##
Las futuras versiones cercanas de Zirox Translator planean implementar características adicionales, incluyendo:

- Gramática de bucle.
- Gramática condicional.
- Gramática de función.
- Crear tabla de simbolos para las variables
- Crear analizador sintatico para encontrar posibles errores de sintaxys
- Interpretar el AST
