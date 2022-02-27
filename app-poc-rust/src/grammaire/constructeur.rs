
// à ajouter : .is_nan() 
// https://doc.rust-lang.org/std/primitive.f64.html#method.is_nan 


use crate::grammaire::parseur::Lemmes; 
use crate::grammaire::parseur::charger; 
use core::slice::Iter;

use std::collections::HashMap; 

use crate::communs::Types; 

#[derive(Debug)]
pub struct Environnement { 
	pub variables: HashMap<String,Types>, 
	pub conditions: HashMap<String,Vec<Types>>, 
	pub regles: HashMap<String,( 
		f64, 
		Vec<Types>, 
		Vec<Types>, 
		Vec<Types>, 
		Vec<Types> 
	)> 
} 

impl Environnement { 
	fn creer() -> Self { 
		Environnement { 
			variables: HashMap::new(), 
			conditions: HashMap::new(), 
			regles: HashMap::new()  
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
	fn conditionner( &mut self, nom: String, condition: Vec<Types> ) { 
		self.conditions.insert( nom, condition ); 
	} 
	fn regler( &mut self, nom: String, poids: f64, si: Vec<Types>, alors: Vec<Types>, sinon: Vec<Types>, finalement: Vec<Types> ) { 
		self.regles.insert( 
			nom, 
			(
				poids, 
				si, 
				alors, 
				sinon, 
				finalement 
			) 
		); 
	} 
} 

fn retrouver_appelable( iterable: &mut Vec<Lemmes> ) -> Result<Vec<Types>,&'static str> { 
	let mut appelable: Vec<Types> = vec!(); 
	match iterable.pop() { 
		Some( Lemmes::Appelable_Depart( _ ) ) => (), 
		_ => return Err( "Appelable mal formé au sein d'une clause" ), 
	} 
	loop { 
		match iterable.pop() { 
			Some( Lemmes::Appelable_Fin( _ ) ) => break, 
			Some( Lemmes::Texte( _, texte ) ) => appelable.push( Types::Texte( String::from( &texte[1..texte.len()-1] ) ) ), 
			Some( Lemmes::Variable( _, texte ) ) => appelable.push( Types::Variable( String::from( &texte[0..texte.len()] ) ) ), 
			Some( Lemmes::Nombre( _, nbre_textuel ) ) => match &nbre_textuel.parse::<f64>() { 
				Ok( nbre ) => appelable.push( Types::Nombre( *nbre ) ), 
				Err( _ ) => return Err( "Corps de variable numéraire incorrect" ) 
			}, 
			_ => return Err( "Appelable mal formé au sein d'une clause" ), 
		} 
	} 
	Ok( appelable ) 
}

fn definir_variable( iterable: &mut Vec<Lemmes>, environnement: &mut Environnement ) -> Result<(), &'static str> { 
	environnement.definir( 
		match iterable.pop() { 
			Some( Lemmes::Variable( _, texte ) ) => texte, 
			None => return Err( "Variable sans nom" ), 
			_ => return Err( "Nom de variable incorrect" ) 
		}, 
		match iterable.pop() {
			Some( Lemmes::Texte( _, texte ) ) => Some( Types::Texte( String::from( &texte[1..texte.len()-1] ) ) ),
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

fn definir_condition( iterable: &mut Vec<Lemmes>, environnement: &mut Environnement ) -> Result<(), &'static str> { 
	let condition_nom: String = match iterable.pop() { 
		Some( Lemmes::Texte( _, texte ) ) => String::from( &texte[1..texte.len()-1] ), 
		None => return Err( "Condition sans nom" ), 
		_ => return Err( "Nom de condition incorrect" ) 
	}; 
	match iterable.pop() { 
		Some( Lemmes::Clause_Depart( _ ) ) => (), 
		_ => return Err( "Un départ de clause est attendu" ) 
	}; 
	let mut clauses: Vec<Types> = vec!(); 
	loop { 
		match iterable.pop() { 
			Some( Lemmes::Clause_Fin( _ ) ) => break, 
			Some( Lemmes::Conditionnel( _, nom ) ) => clauses.push( Types::Conditionnel( String::from( &nom[1..nom.len()-1] ) ) ), 
			Some( Lemmes::Et( _ ) ) => clauses.push( Types::Et ), 
			Some( Lemmes::Ou( _ ) ) => clauses.push( Types::Ou ), 
			Some( Lemmes::Variable( _, nom ) ) => clauses.push( Types::Appelable( 
				nom, 
				None,
				retrouver_appelable( iterable )? 
			) ), 
			None => return Err( "Une clause n'est pas terminée" ), 
			_ => return Err( "Element incompatible avec un ensemble de clauses" ) 
		}; 
	} 
	match iterable.pop() { 
		Some( Lemmes::Condition_Fin( _ ) ) => (), 
		_ => return Err( "Un fin de condition est attendue" ) 
	}; 
	environnement.conditionner( 
		condition_nom, 
		clauses 
	); 
	Ok( () ) 
} 

fn definir_regle( iterable: &mut Vec<Lemmes>, environnement: &mut Environnement ) -> Result<(), &'static str> { 
	let regle_nom: String = match iterable.pop() { 
		Some( Lemmes::Texte( _, texte ) ) => String::from( &texte[1..texte.len()-1] ), 
		None => return Err( "Règle sans nom" ), 
		_ => return Err( "Nom de règle incorrect" ) 
	}; 
	let regle_poids: f64 = match iterable.pop() { 
		Some( Lemmes::Regle_Poids( _, nbre_textuel ) ) => match &nbre_textuel.parse::<f64>() { 
			Ok( nbre ) => *nbre, 
			Err( _ ) => return Err( "Corps de variable numéraire incorrect pour le poids d'une règle" ) 
		}, 
		None => return Err( "Règle sans poids" ), 
		_ => return Err( "Poids de règle incorrect" ) 
	}; 
	match iterable.pop() { 
		Some( Lemmes::Si_Depart( _ ) ) => (), 
		_ => return Err( "Un début de clé 'Si' est attendu" ) 
	}; 
	let mut si: Vec<Types> = vec!(); 
	loop { 
		match iterable.pop() { 
			Some( Lemmes::Si_Fin( _ ) ) => break, 
			Some( Lemmes::Conditionnel( _, nom ) ) => si.push( Types::Conditionnel( String::from( &nom[1..nom.len()-1] ) ) ), 
			Some( Lemmes::Et( _ ) ) => si.push( Types::Et ), 
			Some( Lemmes::Ou( _ ) ) => si.push( Types::Ou ), 
			None => return Err( "Une fin de clé 'Si' est attendue" ), 
			_ => return Err( "La clé 'Si' ne peut comporter que des conditionnalités" ) 
		}; 
	} 
	let mut alors: Vec<Types> = vec!(); 
	let mut sinon: Vec<Types> = vec!(); 
	let mut finalement: Vec<Types> = vec!(); 
	loop{ 
		match iterable.pop() { 
			Some( Lemmes::Alors_Depart( _ ) ) => loop { 
				match iterable.pop() { 
					Some( Lemmes::Alors_Fin( _ ) ) => break, 
					Some( Lemmes::Texte( _, texte ) ) => alors.push( Types::Texte( String::from( &texte[1..texte.len()-1] ) ) ),
					Some( Lemmes::Nombre( _, nbre_textuel ) ) => match &nbre_textuel.parse::<f64>() { 
						Ok( nbre ) => alors.push( Types::Nombre( *nbre ) ), 
						Err( _ ) => return Err( "Corps de variable numéraire incorrect" ) 
					}, 
					Some( Lemmes::Variable( _, nom ) ) => alors.push( Types::Appelable( 
						nom, 
						None, 
						retrouver_appelable( iterable )? 
					) ), 
					None => return Err( "Clé 'Alors' non-terminée" ), 
					_ => return Err( "Clé 'Alors' dotée d'une valeur invalide" ), 
				} 
			}, 
			Some( Lemmes::Sinon_Depart( _ ) ) => loop { 
				match iterable.pop() { 
					Some( Lemmes::Sinon_Fin( _ ) ) => break, 
					Some( Lemmes::Texte( _, texte ) ) => sinon.push( Types::Texte( String::from( &texte[1..texte.len()-1] ) ) ),
					Some( Lemmes::Nombre( _, nbre_textuel ) ) => match &nbre_textuel.parse::<f64>() { 
						Ok( nbre ) => sinon.push( Types::Nombre( *nbre ) ), 
						Err( _ ) => return Err( "Corps de variable numéraire incorrect" ) 
					}, 
					Some( Lemmes::Variable( _, nom ) ) => sinon.push( Types::Appelable( 
						nom, 
						None, 
						retrouver_appelable( iterable )? 
					) ), 
					None => return Err( "Clé 'Sinon' non-terminée" ), 
					_ => return Err( "Clé 'Sinon' dotée d'une valeur invalide" ), 
				} 
			}, 
			Some( Lemmes::Finalement_Depart( _ ) ) => loop { 
				match iterable.pop() { 
					Some( Lemmes::Finalement_Fin( _ ) ) => break, 
					Some( Lemmes::Texte( _, texte ) ) => finalement.push( Types::Texte( String::from( &texte[1..texte.len()-1] ) ) ),
					Some( Lemmes::Nombre( _, nbre_textuel ) ) => match &nbre_textuel.parse::<f64>() { 
						Ok( nbre ) => finalement.push( Types::Nombre( *nbre ) ), 
						Err( _ ) => return Err( "Corps de variable numéraire incorrect" ) 
					}, 
					Some( Lemmes::Variable( _, nom ) ) => finalement.push( Types::Appelable( 
						nom, 
						None, 
						retrouver_appelable( iterable )? 
					) ), 
					None => return Err( "Clé 'Finalement' non-terminée" ), 
					_ => return Err( "Clé 'Finalement' dotée d'une valeur invalide" ), 
				} 
			}, 
			Some( Lemmes::Regle_Fin( _ ) ) => break, 
			e @ _ => { 
				println!( "{:?}", e ); 
				return Err( "Un début de clé incorrect a été trouvé" ); 
			} 
		} 
	}
	environnement.regler( 
		regle_nom, 
		regle_poids, 
		si, 
		alors, 
		sinon, 
		finalement
	); 
	Ok( () ) 
} 

pub fn construire( chemin: String ) -> Result<Environnement, &'static str> { 
	let mut corpus = charger( chemin )?; 
	let mut environnement = Environnement::creer();  
	corpus.lemmes.reverse(); 
	while let Some( lemme ) = corpus.lemmes.pop() { 
		match lemme { 
			Lemmes::Variable_Depart( _ ) => definir_variable( 
				&mut corpus.lemmes, 
				&mut environnement 
			)?, 
			Lemmes::Condition_Depart( _ ) => definir_condition( 
				&mut corpus.lemmes, 
				&mut environnement 
			)?, 
			Lemmes::Regle_Depart( _ ) => definir_regle( 
				&mut corpus.lemmes, 
				&mut environnement 
			)?, 
			_ => () 
		} 
	} 
	Ok( environnement ) 
} 




