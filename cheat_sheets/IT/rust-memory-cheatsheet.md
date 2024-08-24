# **Rust CheatSheet - Memory

##### **Table of Contents**
###### [§ Stack](#-Stack-1)
-  [Definizione](#Definizione)
- [Visualizzazione Dati nello Stack](#Visualizzazione-Dati-nello-Stack)
###### [§ Heap](#-Heap-1)
- [Definizione](#Definizione)
- [Visualizzazione Dati nello Heap](#Visualizzazione-Dati-nello-Heap)
- [Uso nel Codice](#Uso-nel-Codice)
	
___
## **§ Stack**
	
**Definizione**: Allocazione usata per tipi di dati dove sappiamo con certezza la grandezza necessaria.
**Tags**: #Stack #Memory #Arrays 
	
### Definizione
	
**Disposizione**: 
- La memoria è allocata in sequenza "appoggiando" i dati uno "dietro" l'altro.
**Gestione I/O**: 
- LIFO (Last In, First Out)
**Puntatore**: 
- Hanno un operatività di gestione veloce utilizzando un *offset* sul puntatore
- Gli stack sequenzializzano i dati prendendo come rifermento l' inizio delle sequenze
**Capacita**: 
- Lo spazio è limitato esempio: *arrays*.
	
### Visualizzazione Dati nello Stack
	
Data List:
- DataX = 3bit
- DataY = 4bit
- DataZ = 2bit
- DataU = 3bit
	
0 ->B = **Indirizzi**
	
| 0     | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     | 9     | A     | B     |
| ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- |
| DataX | DataX | DataX | DataY | DataY | DataY | DataY | DataZ | DataZ | DataU | DataU | DataU |

Data Addresses:
- DataX = 0
- DataY = 3
- DataZ = 7
- DataU = 9
	
	
## **§ Heap**
	
**Definizione**: Allocazione usata per tipi di dati dove ***non*** sappiamo con certezza la grandezza necessaria.
**Tags**: #Heap #Memory #usize #Hashmaps #Vectors 
	
### Definizione
	
**Disposizione**: 
- La memoria è allocata con un algoritmo che la posiziona in maniera dinamica sfruttando le proprietà di una porzione di memoria *stack*.
**Puntatore**: 
- Hanno un operatività di gestione lenta utilizzando un *offset* sul puntatore della parte *stack* per recuperare gli indirizzi allocati dinamicamente in memoria.
- I dati per i puntatori sono *usize*
**Capacita**: 
- Il metodo di allocazione è usato da: *HashMaps* e *Vectors* **.
- La memoria è direttamente gestita da RAM e Disks
	
### Visualizzazione Dati nello Heap
	
Data List:
- DataX = 3bit = Address: Stack 0
- DataY = 4bit
- DataZ = 3bit
- DataU = 1bit
	
Address List:
- DataX = Stack 0
- DataY = Stack 1
- DataZ = Stack 2
- DataU = Stack 3
	
| 0      | 1      | 2      | 3      | 4   | 5   | 6   | 7   | 8   | 9   | A   |
| ------ | ------ | ------ | ------ | --- | --- | --- | --- | --- | --- | --- |
| 00 + 6 | 10 + 0 | 20 + A | 40 + 7 |     |     |     |     |     |     |     |
	
##### Allocazione Dinamica Heap
Si alloca come negli scacchi e si prosegue in sequenza.
	
|     |   0   |   1   |   2   |   3   |  4  |  5  |   6   |   7   |   8   |  9  |   A   |
| :-: | :---: | :---: | :---: | :---: | :-: | :-: | :---: | :---: | :---: | :-: | :---: |
| 00  |       |       |       |       |     |     | DataX | DataX | DataX |     |       |
| 10  | DataY | DataY | DataY | DataY |     |     |       |       |       |     |       |
| 20  |       |       |       |       |     |     |       |       |       |     | DataZ |
| 30  | DataZ | DataZ |       |       |     |     |       |       |       |     |       |
| 40  |       |       |       |       |     |     |       | DataU |       |     |       |
	
##### Gestione I/O 
Se si vuole aggiungere un dato la dove non c'è più posto come **3 bit** per **DataX** il set di dati verrà spostato la dove esiste posto modificando poi il puntatore nella memoria *stack*
	
### Uso nel Codice
	
- **Sintassi**: `Box<Type>` 
- **Esempio**:
	
```Rust
struct Person {
	id: u8,
}

fn main() {
	let data = Person { id: 09 }; // Crea su stack
	let data_allocated_to_heap: Box<Person> = Box::new(data); // Muove su heap
	let data_allocated_to_stack = *data_heap; // Spostare di nuovo su stack
}
```
	
- **Soluzione a**:
	
```sh
Error type cannot have an unboxed trait object

whatever
  ^^^doesnt have a size known at compile-time
```
	
---
###### Suggested Follow-up
[Rust CheatSheet - ](./.md)
	  
---
  
**Author**: Kenneth Boldrini