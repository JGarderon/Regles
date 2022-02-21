
use crate::grammaire::parseur::Lemmes; 
use crate::grammaire::parseur::charger; 
use core::slice::Iter;

use std::collections::HashMap; 

use crate::communs::Types; 

#[derive(Debug)]
struct Environnement { 
	variables: HashMap<String,Types> 
} 

fn test( iterable: &mut Vec<Lemmes> ) {
	println!("1 {:?}", iterable.pop()); 
	println!("2 {:?}", iterable.pop()); 
}

pub fn construire() -> Result<(), &'static str> {
	let mut corpus = charger( "regles.txt".to_string() )?; 
	corpus.lemmes.reverse(); 
	while let Some( lemme ) = corpus.lemmes.pop() { 
		println!("=== {:?}", lemme);
		match lemme { 
			Lemmes::Variable_Depart(_) => test( &mut corpus.lemmes ), 
			_ => break 
		} 
	} 
	Ok( () ) 
} 




