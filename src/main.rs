// impelmentamos librerias
use std::collections::HashMap;
use std::io::{self};

#[derive(Debug)]
enum Taskstatus{
    Pending,
    Complete,
}

#[derive(Debug)]
struct Task {
    description:String,
    status:Taskstatus,
}

impl Task {
    fn new(description:&str)-> Task{
        Task{
            description:description.to_string(),
            status:Taskstatus::Pending,
        }
    }
}

fn main() {
    let mut tasks: HashMap<usize, Task> = HashMap::new();
    let mut id_counter:usize =1;

    loop {
        println!("Gestor de Tareas");
        println!("1. Añadir tarea");
        println!("2. Listar tareas");
        println!("3. Marcar como terminada");
        println!("4. salir");

        let mut choice:String = String::new();
        io::stdin().read_line(&mut choice).expect("Error al leer la entrada");
        let choice:&str = choice.trim();
        
        match choice {
            "1"=>{
                println!("Introduce la descripcion de la tarea");
                let mut descripcion:String = String::new();
                io::stdin().read_line(&mut descripcion).expect("Error al leer la entrada"); // input from cmd
                let task:Task = Task::new(&descripcion.trim());

                // id_counter +=1;
                while tasks.contains_key(&id_counter) {
                    id_counter +=1;
                }
                tasks.insert(id_counter, task);
                println!("Tarea añadida con ID: {}", id_counter);

            },
            "2"=>{
                for(_id,task) in &tasks{
                    let status = match task.status {
                        Taskstatus::Pending=>"Pendiente",
                        Taskstatus::Complete=> "Completada",
                    };
                    println!("ID:{}, Descripcion: {}, Estado:{}",id_counter, task.description,status);
                }
            },
            "3"=>{
                println!("Introduce el ID de la tarea a marcar como completada");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Error al leer la entrada");

                let id:usize= match id.trim().parse() {
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("ID invalido");
                        continue
                    },
                };
                if let Some(task) = tasks.get_mut(&id){
                    task.status = Taskstatus::Complete;
                    println!("Tarea con ID {} marcada como completada",id);
                }else{
                    println!("No se encontro la tarea con ID {}",id);
                }
            },
            "4"=>{
                println!("Saliendo...");
                break;
            },
            _=>println!("Opcion no valida, favor intentar nuevamente"),
        }

    }
}