// Control de flujo

// Decidir si ejecutar o no algún código dependiendo de si una condición
// es verdadera o repretir un código mientras una condición es veradera
// son bloques de construcción básicos en la mayoría de los lenguajes
// de programación.

// Las construccioines que permiten controlar el flujop de ejecución del
// código Rust son las expresiones if y los bucles.

fn main() {

    let numero = 42;
    // let numero = 444;

    if numero < 100 {
        println!("Se ha cumplido la condición");
    } else {
        println!("No se ha cumplido la condición");
    }

    let es_cierto = true;

    if es_cierto {
        println!("¡Sí, es cierto!");
    } else {
        println!("No es cierto...");
    }

    let numero_numero = 42;

    if numero_numero < 50 {
        println!("El número es menor que 50");
    } else if numero_numero  < 100 {
        println!("Está comprendido entre 50 y 100");
    } else {
        println!("Es igual o mayor que 100");
    }

    let condicion = true;

    let numero3 = if condicion { 5 } else { 13 };

    println!("El valor de numero es: {}", numero3);


    // Bucles

    // Suele ser habitual querer ejecutar un bloque de código más de una
    // vez. Para esta tarea, Rust proporciona diferentes bucles.

    // Rust tiene tres tipos de bucles: loop, while, y for.

    // loop

    // loop sirve para definir bucles infinitos que se acabarán mediante el
    // uso de una sentencia break.

    // Con loop, es posible devolver un resultado al resto de código.
    // Para hacer esto, se agrega el valor que desea que se devuelva después de
    // la expresión break que usa para detener el bucle.

    let mut contador = 0;

    let resultado_loop = loop {
        println!("Hola amigos");
        println!("{}", contador);
        contador += 1;

        if contador == 10 {
            break contador * contador;
        }
    };

    println!("El valor de resultado_loop es: {}", resultado_loop);


    // while

    // while sirve para definir bucles que se ejecutarán mientras una
    // condición es verdadera.

    let mut contador_nuevo = 10;

    while contador_nuevo != 0 {
        println!("Contador nuevo: {}", contador_nuevo);
        contador_nuevo -= 1;
    }

    let matriz = [10,20,30,40,50,60,70,80,90];
    let mut indice = 0;

    while indice < 9 {
        println!("Valor del array en la posición {}: {}", indice+1, matriz[indice]);
        indice += 1;
    }


    // for

    // for in se utiliza para iterar a través de un Iterator.

    let matriz_nueva = [10,20,30,40,50,60,70,80,90];

    for numero_num in matriz_nueva.iter() {

        println!("Valor del elemento de la matriz es: {}", numero_num);

    }

    // Una de las formas más sencillas de crear un iterador es utilizar la
    // notación de rango a..b. Esto produce valores de a (inclusive) a b
    // (exclusivo) en pasos de uno.

    // Alternativamente, a..=b puede usarse para un rango que sea inclusivo
    // en ambos extremos.

    for numero_en_el_buclo_con_puntos in 1..10 {

        println!("Valor del elemento 1..10 es: {}", numero_en_el_buclo_con_puntos);
                            // recurda que 10 está EXCLUIDO
                            // se puede no excluir por usar 1..=10
    }

    for numero in 1..=10 {

        println!("Valor del elemento 1..=10 es: {}", numero);

    }

    //

    for numero in (1..10).rev() {
        println!("De forma inversa: (1..10).rev() es: {}", numero);
    }

    for numero in (2..200).rev() {
        println!("Y al final... {}", numero);
    }


}





