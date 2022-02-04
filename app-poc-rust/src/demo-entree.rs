// #![allow(warnings, unused)] 

use std::io;
use std::thread;
use std::sync::mpsc::channel; 
use std::process::{Command, Stdio};
use std::io::Write;
use std::io::BufReader; 
use std::io::BufRead; 

#[derive(Debug)]
enum Canal {
    // Pret,
    Stop,
    // Erreur, 
    // Initialisation,
    // Reinitilisation, 
    Message(String) 
}

fn main() {

    let (emeteur_parent, recepteur_enfant) = channel::<Canal>(); 
    // let (emeteur_enfant, recepteur_parent) = channel::<Canal>(); 

    let fil_parent_reception = thread::spawn( move || {
        let entree_generale = io::stdin(); 
        let mut buffer = String::new();
        loop { 
            match entree_generale.read_line( &mut buffer ) { 
                Ok( 0 ) => break, 
                Ok( _ ) => { 
                    // ici désérialiser le message reçu 
                    emeteur_parent.send( 
                        Canal::Message( buffer.to_string() )
                    ).unwrap(); 
                } 
                Err( erreur ) => panic!("[RUST] fil_parent_reception en erreur : {:?}", erreur) 
            } 
            buffer.clear(); 
        } 
        println!("[RUST] plus rien à consommer depuis le parent (1)");
        emeteur_parent.send( 
            Canal::Stop 
        ).unwrap(); 
    } ); 

    let mut child = Command::new("python3")
        .arg( "./support.py" ) 
        .stdin(Stdio::piped()) 
        .stdout(Stdio::piped()) 
        .stderr(Stdio::piped()) 
        .spawn()
        .unwrap();
    let mut enfant_stdin = child.stdin.take().expect("Failed to open stdin");
    let mut enfant_err = BufReader::new(child.stderr.take().unwrap()).lines();

    let fil_enfant = thread::spawn( move || { 
        let mut recepteur_enfant_iterable = recepteur_enfant.iter().peekable(); 
        loop { 
            match recepteur_enfant_iterable.peek() { 
                Some( Canal::Stop ) => {
                    // println!("plus rien à consommer depuis le parent (3)"); 
                    enfant_stdin.write_all( 
                        "OP-STOP\n".as_bytes() 
                    ).expect("Failed to write to stdin (1)"); 
                } 
                _ => () 
            } 
            enfant_stdin.flush().unwrap(); 
            match enfant_err.next() { 
                Some( Ok( etat )  ) => { 
                    match etat.as_str() { 
                        "OP-DEBUT" => println!("[RUST] enfant prêt à recevoir une nouvelle opération via un ordre"), 
                        "OP-FIN" => panic!("[RUST] une fin d'opération demandée par l'enfant alors qu'il devrait être en attente d'un ordre"), 
                        "OP-STOP" => break, 
                        _ => panic!("[RUST] cas non géré") 
                    } 
                    println!("[RUST] (réception de l'enfant 1) {:?}", etat); 
                } 
                Some( Err( erreur ) ) => panic!("[RUST] erreur #1 {:?}", erreur), 
                None => break 
            }
            match recepteur_enfant_iterable.next() { 
                Some( Canal::Stop ) => {
                    println!("plus rien à consommer depuis le parent (2)"); 
                    break; 
                }, 
                Some( Canal::Message( ordre ) ) => { 
                    enfant_stdin.write_all( 
                        ordre.as_bytes() 
                    ).expect("Failed to write to stdin (1)"); 
                    enfant_stdin.flush().unwrap(); 
                    println!("[RUST] (envoi à l'enfant) {:?}", ordre); 
                } 
                _ => () 
            } 
            match enfant_err.next() { 
                Some( Ok( etat )  ) => {
                    match etat.as_str() { 
                        "OP-FIN" => println!("[RUST] fin normale de l'opération par l'enfant"), 
                        _ => () 
                    } 
                    println!("[RUST] (réception de l'enfant 2) {:?}", etat); 
                } 
                Some( Err( erreur ) ) => panic!("[RUST] erreur #2 {:?}", erreur), 
                None => break 
            } 
        } 
    } ); 

    fil_parent_reception.join().unwrap(); 
    fil_enfant.join().unwrap(); 

    let status = child.wait().unwrap();
    println!("[RUST] {}", status);
} 

