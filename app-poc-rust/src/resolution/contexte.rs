
use std::mem; 
use core::slice::Iter; 

use crate::grammaire::constructeur::Environnement; 
use crate::communs::Types; 

#[derive(Debug)] 
pub struct Contexte<'env> { 
	environnement: &'env Environnement, 
	clauses: Vec<&'env Types>, 
	regles: Vec<( 
		&'env String, 
		Vec<&'env Types> 
	)>, 
	position: usize 
} 

pub fn construire<'env>( environnement: &'env Environnement ) -> Result<Contexte,&'static str> { 
	let mut clauses = environnement.conditions.values().map( 
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
	clauses.sort_unstable_by( 
		// échouera si NaN pour les f64 
		// https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by 
		|a, b| a.partial_cmp(b).unwrap() 
	); 
	clauses.dedup(); 
	let mut regles: Vec<(f64,(&String,Vec<&'env Types>))> = vec!(); 
	for (regle_cle, regle_valeur) in environnement.regles.iter() { 
		let mut regle_clauses: Vec<&Types> = vec!(); 
		{ 
			let mut positions: Vec<Iter<Types>> = vec!(); 
			let mut actuel =  regle_valeur.1.iter();  
			loop { 
				while let Some( item ) = actuel.next() { 
					match item { 
						Types::Et | Types::Ou => regle_clauses.push( &item ), 
						Types::Appelable( _, _, _ ) => regle_clauses.push( &item ), 
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
		} 
		let regle_clauses = regle_clauses.iter().map( 
			|clause_avant| match clause_avant { 
				Types::Appelable( _, _, _ ) => {
					for clause in clauses.iter() { 
						if clause_avant == clause { 
							return *clause; 
						} 
					} 
					*clause_avant 
				}
				_ => clause_avant 
			}
		).collect::<Vec<&Types>>(); 
		regles.push( 
			( 
				regle_valeur.0,
				(
					&regle_cle, 
					regle_clauses 
				) 
			) 
		); 
	} 
	regles.sort_unstable_by( 
		|a, b| a.0.partial_cmp( &b.0 ).unwrap() 
	); 
	let regles = regles.into_iter().map( 
		|item| item.1 
	).collect::<Vec<(&String,Vec<&'env Types>)>>(); 
	Ok( 
		Contexte { 
			environnement: environnement, 
			clauses: clauses, 
			regles: regles, 
			position: 0 
		} 
	) 
} 

