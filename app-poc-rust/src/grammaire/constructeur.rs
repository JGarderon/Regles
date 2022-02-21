
use crate::grammaire::parseur::Lemmes; 
use crate::grammaire::parseur::charger; 
use core::slice::Iter;

use std::collections::HashMap; 

use crate::communs::Types; 

#[derive(Debug)]
pub struct Environnement { 
	variables: HashMap<String,Types> 
} 

impl Environnement { 
	fn creer() -> Self { 
		Environnement { 
			variables: HashMap::new() 
		} 
	} 
	fn definir( &mut self, nom: String, valeur: Option<Types> ) -> Option<Types> {
		match valeur { 
			Some( v ) => self.variables.insert( nom, v ), 
			None => self.variables.remove( &nom ) 
		} 
	} 
	fn obtenir( &mut self, nom: &String ) -> Option<&Types> { 
		self.variables.get( nom ) 
	} 
} 

fn definir_variable( iterable: &mut Vec<Lemmes>, environnement: &mut Environnement ) -> Result<(), &'static str> { 
	environnement.definir( 
		match iterable.pop() { 
			Some( Lemmes::Variable( _, texte ) ) => texte, 
			None => return Err( "Variable sans nom" ), 
			_ => return Err( "Nom de variable incorrect" ) 
		}, 
		match iterable.pop() {
			Some( Lemmes::Texte( _, texte ) ) => Some( Types::Texte( texte ) ),
			Some( Lemmes::Nombre( _, nbre_textuel ) ) => match &nbre_textuel.parse::<f64>() { 
				Ok( nbre ) => Some( Types::Nombre( *nbre ) ), 
				Err( _ ) => return Err( "Corps de variable numéraire incorrect" ) 
			}, 			
			None => return Err( "Variable sans corps" ), 
			_ => return Err( "Corps de variable non-supporté" ) 
		} 
	); 
	match iterable.pop() {
		Some( Variable_Fin ) => return Ok( () ),
		None => return Err( "Définition de variable non-explicitement terminée" ) 
	} 
}

pub fn construire() -> Result<Environnement, &'static str> { 
	let mut corpus = charger( "regles.txt".to_string() )?; 
	let mut environnement = Environnement::creer(); 
	corpus.lemmes.reverse(); 
	while let Some( lemme ) = corpus.lemmes.pop() { 
		println!("=== {:?}", lemme);
		match lemme { 
			Lemmes::Variable_Depart( _ ) => definir_variable( 
				&mut corpus.lemmes, 
				&mut environnement 
			)?, 
			_ => () 
		} 
	} 
	Ok( environnement ) 
} 




