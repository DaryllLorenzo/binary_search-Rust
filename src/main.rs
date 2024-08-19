use std::vec;
fn main() {
    let arr:Vec<i32> = vec![1,2,3,4,6,7,10,12,13,15,16,17];
    let pos_final = arr.get(arr.len() - 1);
    let posicion_inicial: i32 = 0;
    let mut posicion_final : i32 = 0;
    if let Some(pos) = pos_final{
        posicion_final = *pos;
    }   
    let resultado:i32 = binary_search(posicion_inicial, posicion_final, &arr , 13);    
    println!("{}",resultado);
}
fn binary_search(pos_inicial:i32 , pos_final: i32, arr: &Vec<i32>, numero:i32)-> i32{
    // No se encontro el elemento
    if pos_inicial > pos_final{
        return - 1;
    }
    let mitad = (pos_inicial + pos_final) / 2;
    let op = arr.get(mitad as usize);
    if let Some(valor_op) = op{
        // Encontro el elemento 
        if numero == *valor_op {
            return mitad;
        }
        if numero < *valor_op{
            binary_search(pos_inicial, mitad - 1, arr, numero);
        }
        if numero > *valor_op{
            binary_search(mitad + 1,  pos_final, arr, numero);
        }
    }
    return - 1;
}
