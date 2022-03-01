#![allow(warnings, unused)] 

mod communs; 
use crate::communs::Types; 

mod grammaire; 
use crate::grammaire::constructeur::construire as environnement_construire; 

mod resolution; 
use crate::resolution::contexte::Contexte;
use crate::resolution::contexte::construire as contexte_resolution; 
use crate::resolution::executer as resolution_executer; 

fn main() { 
	match resolution_executer() { 
		Ok( _ ) => (), 
		Err( erreur ) => println!( "# erreur : {}", erreur ) 
	}; 
} 



