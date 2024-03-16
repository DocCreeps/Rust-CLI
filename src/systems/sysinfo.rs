use std::io;
// use std::process::Command;
use whoami;
use sysinfo::{
    Components, Disks, Networks, System,
};


pub fn main(){
    menu();
}

fn all(){
whoami_info();
info_sys();


}

fn whoami_info(){
    println!("Whoami : " );
    println!(
        "Nom réel utilisateur   whoami::realname():    {}",
        whoami::realname(),
    );
    println!(
        "Nom utilisateur        whoami::username():    {}",
        whoami::username(),
    );
    println!(
        "Languages               whoami::lang():       {:?}",
        whoami::lang().collect::<Vec<String>>(),
    );
    println!(
        "Nom ordinateur         whoami::devicename():  {}",
        whoami::devicename(),
    );

    println!(
        "OS Distrib             whoami::distro():      {}",
        whoami::distro(),
    );
    println!(
        "CPU Arch               whoami::arch():        {}",
        whoami::arch(),
    );
}

fn info_sys(){


    println!("Système :");
    println!("Memory :");
    memory();

    println!("Disks :");
    disks();

    println!("CPU :");
    cpu();

}

fn cpu(){
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("CPU:");
    // Get the first CPU from the list of CPUs.
    if let Some(cpu) = sys.cpus().first() {
        println!("CPU Vendor: {}", cpu.vendor_id());
        println!("CPU Brand: {}", cpu.brand());
    }

    sys.cpus().iter().for_each(|cpu| {
        println!("CPU Cores: {}", cpu.name());
        println!("CPU Frequency: {} MHz", cpu.frequency());
        println!("CPU Usage {}% \n", cpu.cpu_usage());
    });
}
fn memory(){
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("Memory");
    println!("total memory: {} GB", sys.total_memory() as f64 / 1_073_741_824.0);
    println!("used memory : {} GB", sys.used_memory() as f64 / 1_073_741_824.0);
    println!("total swap  : {} GB", sys.total_swap() as f64 / 1_073_741_824.0);
    println!("used swap   : {} GB", sys.used_swap() as f64 / 1_073_741_824.0);
}

fn disks(){
    let disks = Disks::new_with_refreshed_list();
    println!("Disks:");
    disks.iter().for_each(|disk| {
        println!("Device Name: {:?}", disk.name());
        println!("Mount Point: {:?}", disk.mount_point());
        println!("File System: {:?}", disk.file_system());
        println!("Total space: {} GB", disk.total_space() as f64 / 1_073_741_824.0);
        println!("Available space : {} GB", disk.available_space() as f64 / 1_073_741_824.0);
    });
}

fn menu(){
        loop {
            println!("Bienvenue dans l'outil info Système");
            println!("Choisissez une option :");
            println!("1. Tous voir");
            println!("2. Identité");
            println!("3. Info système");
            println!("4. Mémoire");
            println!("5. Disque");
            println!("6. CPU");
            println!("0. Retour au menu principal");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");

            let choice: u32 = input.trim().parse().unwrap();

            match choice {

                1 => all(),
                2 => whoami_info(),
                3 => info_sys(),
                4 => memory(),
                5 => disks(),
                6 => cpu(),
                0 => {
                    println!("Au revoir !");
                    break;
                },
                _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
            }
        }
}