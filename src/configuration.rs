
use std::env; 
// use std::sync::{Arc, Mutex}; # prochainement, pour la partie 'fil' ? 

use std::collections::HashMap; 
use crate::communs::Erreur; 
use crate::Types;

#[derive(Debug)] 
struct Configuration( HashMap<&'static str,Types> ); 

static mut CONFIGURATION: Option<Configuration> = None; 

pub fn configurer() -> Result<(), Erreur> { 
	unsafe { 
		if CONFIGURATION.is_some() { 
			return Err( 
				Erreur::creer( 
					"La configuration a déjà été faite : possible de relancer" 
				)
			); 
		} else { 
			CONFIGURATION = Some( 
				Configuration { 
					0: HashMap::new() 
				} 
			); 
		} 
		// for argument in env::args() { 
		//     argument.starts_with("--"); 
		// } 
		println!("{:?}", CONFIGURATION); 
	} 
	Ok( () ) 
} 



