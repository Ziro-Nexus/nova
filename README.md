## Lenguaje Nova 🚀 ##

Nova es un lenguaje de programación diseñado para ser simple y fácil de aprender, pero lo suficientemente poderoso como para crear aplicaciones complejas. Aquí se explica cómo funciona el código del intérprete Nova.


## Módulos 📦 ##

El intérprete Nova se divide en varios módulos, cada uno de los cuales maneja una parte específica del proceso de interpretación.

- ast: contiene el generador del Árbol Sintáctico Abstracto (AST) del lenguaje Nova.
- ast_macros: contiene las macros utilizadas para convertir el AST en código de Rust.
- var_table: contiene la tabla de variables del intérprete.
- nova_interpreter: contiene el motor de Nova que se encarga de interpretar el código.

## El AST 🌳 ##

El AST es una representación estructurada del código fuente. El módulo ast contiene el generador de AST del lenguaje Nova. Esto se hace mediante la creación de estructuras de datos que representan las diferentes construcciones sintácticas del lenguaje.

- En ast_macros se implementan las macros necesarias para convertir estas estructuras de datos en código de Rust que pueda ser interpretado por la máquina.
La Tabla de Variables 🗃️

- La tabla de variables es una estructura de datos que almacena todas las variables definidas en el programa. El módulo var_table contiene la implementación de esta tabla.

## El Motor de Nova 🚀 ##

El motor de Nova es el módulo nova_interpreter. Se encarga de coordinar todo el proceso de interpretación del código fuente.

- La función nova_engine::nova_engine::grammar_parser es responsable de leer el archivo de origen línea por línea y generar el AST correspondiente.
- La función nova_engine::nova_engine::resolver se encarga de resolver todas las variables y logica de aplicación
- La función nova_engine::nova_engine::_get_tree devuelve el AST generado.
