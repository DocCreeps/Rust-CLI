use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::io;
use rodio::{Decoder, OutputStream, Sink};

pub fn main() {
    println!("Entrez la durée du timer (format : Min:s) :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");

    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        println!("Format d'entrée invalide. Veuillez entrer la durée sous la forme minutes:secondes.");
        return;
    }

    let minutes: u32 = parts[0].trim().parse().expect("Entrée invalide");
    let seconds: u32 = parts[1].trim().parse().expect("Entrée invalide");

    let duration = Duration::from_secs((minutes as u64) * 60 + seconds as u64);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(duration);
        tx.send(()).unwrap();
    });

    println!("Le timer a démarré.");

    let mut time_left = duration;
    while time_left > Duration::from_secs(0) {
        let mins = time_left.as_secs() / 60;
        let secs = time_left.as_secs() % 60;
        println!("Temps restant : {:02}:{:02}", mins, secs);
        thread::sleep(Duration::from_secs(1));
        time_left = time_left.checked_sub(Duration::from_secs(1)).unwrap_or(Duration::from_secs(0));
    }

    rx.recv().unwrap();
    println!("Le temps est écoulé !");

    // Jouer un son à la fin du timer
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = std::fs::File::open("alarm.mp3").unwrap();
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
