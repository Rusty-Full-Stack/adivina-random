use rand::{thread_rng, Rng};
use read_line::read_line_prompt;

fn main() {
    println!("ADIVINA EL NUMERO!!!");

    // Esta funcion es parte del crate rand
    let mut rng = thread_rng();

    // En Rust, los rangos si toman el primer valor, y excluyen el valor mas grande
    // Para la siguiente linea generaria numeros del 1 al 9.
    let numero: u32 = rng.gen_range(1..10);
    println!("Se ha generado un numero del 1 al 9, adivinalo!");
    loop {
        let user_input = read_line_prompt("Ingresa tu numero:");

        // Utilizamos un contains, porque en el input del usuario, captura el salto de linea \n
        if user_input.contains(&numero.to_string()) {
            break;
        }
        println!("Intentalo de Nuevo!")
    }

    println!("Felicidades, has adivinado!");
}
