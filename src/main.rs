// impelmentamos librerias
use std::collections::HashMap;
use std::fs::File;
use std::io::{self,BufRead,Write};

#[derive(Debug)]
enum Taskstatus{
    Pending,
    Complete,
}

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
    let mut id_counter:usize =0;

    loop {
        println!("Gestor de Tareas");
        println!("1. Añadir tarea");
        println!("2. Listar tareas");
        println!("3. Marcar como terminada");
        println!("4. salir");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error al leer la entrada");
        let choice = choice.trim();
        
        match choice {
            "1"=>{
                println!("Introduce la descripcion de la tarea");
                let mut descripcion = String::new();
                io::stdin().read_line(&mut descripcion).expect("Error al leer la entrada");
                let task:Task = Task::new(&descripcion.trim());
                tasks.insert(id_counter,task);
                id_counter +=1;
            },
            "2"=>{
                for(id,task) in &tasks{
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