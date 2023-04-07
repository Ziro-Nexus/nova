# Nova interprete 🚀 #

![Nova logo](nova_logo.jpeg)
<br>
<br>
### Nova es un lenguaje de programación interpretado escrito en Rust, con tipado dinamico y enfocado al scripting.

<br>

## Módulos 📦 ##

El intérprete Nova se divide en varios módulos, cada uno de los cuales maneja una parte específica del proceso de interpretación.

- ast: contiene el generador del Árbol Sintáctico Abstracto (AST) del lenguaje Nova.
- ast_macros: contiene las macros utilizadas para convertir el AST en código de Rust.
- var_table: contiene la tabla de variables del intérprete.
- nova_interpreter: contiene el motor de Nova que se encarga de interpretar el código.


<br>


## El AST 🌳 ##

El AST es una representación estructurada del código fuente. El módulo ast contiene el generador de AST del lenguaje Nova. Esto se hace mediante la creación de estructuras de datos que representan las diferentes construcciones sintácticas del lenguaje.

- En ast_macros se implementan las macros necesarias para convertir estas estructuras de datos en código de Rust que pueda ser interpretado por la máquina.


<br>


## La Tabla de Variables 🗃️ ##

- La tabla de variables es una estructura de datos que almacena todas las variables definidas en el programa. El módulo var_table contiene la implementación de esta tabla.


<br>


## El Motor de Nova 🚀 ##

El motor de Nova es el módulo nova_interpreter. Se encarga de coordinar todo el proceso de interpretación del código fuente.

- La función nova_engine::nova_engine::grammar_parser es responsable de leer el archivo de origen línea por línea y generar el AST correspondiente.
- La función nova_engine::nova_engine::resolver se encarga de resolver todas las variables y logica de aplicación
- La función nova_engine::nova_engine::_get_tree devuelve el AST generado.


<br>


## Ejemplos funcionales 💪 ##

### Suma de variables ###

```csharp
nya a = 20;
nya b = 20;

nya res = (var::a + var::b);

stdout write res;
```
```bash
40
```

### Soporte de interpolación de datos en strings ###

```csharp
nya a = 20;
nya b = 20;

nya res = (var::a + var::b);
nya res = ("Resultado: " + "var::res");

stdout write res;
```
```bash
"Resultado: 40"
```

### Soporte para expresiones booleanas y comparaciones numericas ###

```csharp
nya expr1 = (true == !false);
nya expr2 = (false == true);

nya value1 = 9;
nya value2 = 3;

nya expr3 = (var::value1 > var::value2);

nya res = (
    "(true == !false) is " + "var::expr1" + "\n" +
    "(false == true) is " + "var::expr2" + "\n" +
    "(var::value1 > var::value2) is " + "var::expr3" + "\n"
);

stdout write res;

```
```bash
"(true == !false) is true
(false == true) is false
(9 > 3) is true"
```

### Soporte para variable shadowing ###

```csharp
nya num = 20;
nya num = (20 + var::num);
nya num = (20 + var::num);
nya num = (var::num * 2);

stdout write num;
```
```bash
120
```