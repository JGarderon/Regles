
use std::mem; 
use core::slice::Iter; 

use crate::grammaire::constructeur::Environnement; 
use crate::communs::Types; 

#[derive(Debug)] 
pub struct Contexte<'env> { 
	environnement: &'env Environnement, 
	clauses: Vec<&'env Types>, 
	regles: Vec<Vec<( 
		String, 
		Vec<&'env Types>, 
		&'env Vec<Types>, 
		&'env Vec<Types>, 
		&'env Vec<Types> 
	)>>, 
	position_groupe: usize, 
	position_indice: usize 
} 

pub fn construire<'env>( environnement: &'env Environnement ) -> Result<Contexte,&'static str> { 
	let clauses = environnement.conditions.values().map( 
		|condition| { 
			condition.iter().filter_map( 
				|clause| match clause { 
					Types::Appelable( _, _, _ ) => Some( clause ), 
					_ => None 
				} 
			).collect::<Vec<&Types>>()
		} 
	).collect::<Vec<Vec<&Types>>>().into_iter().flat_map( 
		|item| item 
	).collect::<Vec<&Types>>(); 

	if let Some( r ) = environnement.regles.get( "réduction" ) { 
		// println!("r = {:?}", r);

		let mut positions: Vec<Iter<Types>> = vec!(); 
		let mut actuel =  r.1.iter();  
		let mut retour: Vec<&Types> = vec!(); 
		loop { 
			while let Some( item ) = actuel.next() { 
				match item { 
					Types::Et | Types::Ou => retour.push( &item ), 
					Types::Appelable( _, _, _ ) => retour.push( &item ), 
					Types::Conditionnel( nom ) => match environnement.conditions.get( nom ) { 
						Some( clauses ) => { 
							let mut ancien = actuel.clone(); 
							positions.push( ancien ); 
							actuel = clauses.iter(); 
							if positions.len() > 15 { 
								return Err( "Profondeur de récursion maximale autorisée des clauses conditionnelles atteinte" ) 
							} 
						} 
						None => return Err( "Demande de condition inconnue" ) 
					} 
					_ => () 
				} 
			} 
			match positions.pop() { 
				Some( position ) => actuel = position, 
				None => break 
			} 
		} 

		println!("r = {:?}", retour); 

		let mut erreur: bool = false; 
		let retour2 = retour.iter().map( 
			|clause_avant| match clause_avant { 
				Types::Appelable( _, _, _ ) => {
					for clause in clauses.iter() { 
						if clause_avant == clause { 
							return *clause; 
						} 
					} 
					erreur = true; 
					*clause_avant 
				}
				_ => clause_avant 
			}
		).collect::<Vec<&Types>>(); 

		println!("erreur = {:?}", erreur); 

		println!("r2 = {:?}", retour2); 


	} 
	println!("!");


	Ok( 
		Contexte { 
			environnement: environnement, 
			clauses: clauses, 
			regles: vec!(), 
			position_groupe: 0, 
			position_indice: 0 
		} 
	) 
} 




// regles: 
// 	si => vec<&Clause> => concaténation des conditions 
// 	alors => vec<Types> 
// 	sinon => vec<Types> 
// 	finalement => vec<Types> 






