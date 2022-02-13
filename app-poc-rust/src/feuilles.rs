
use crate::Jeton;
use crate::Clause;
use crate::Condition;

#[derive(Debug)]
pub struct Feuille<T> {
	pub pile: Vec<bool>, 
	pub action: Vec<Jeton<T>> 
} 

impl Feuille<Clause> { 
	pub fn creer() -> Self {
		Feuille { 
			pile: vec!(), 
			action: vec!() 
		} 
	} 
	pub fn resoudre( self )-> bool where Self: Sized { 
		*&self.pile[1..] 
			.iter()
			.zip( &self.action[..] )
			.fold( 
				self.pile[0], 
				|acc,(etat,liaison)| match liaison {
					Jeton::LiaisonOu => acc | *etat,
					Jeton::LiaisonEt => acc & *etat, 
					_ => panic!( "l'arbre résolutoire des clauses est incorrect" ) 
				} 
			) 
	} 
} 

impl Feuille<Condition> { 
	pub fn creer() -> Self {
		Feuille { 
			pile: vec!(), 
			action: vec!() 
		} 
	} 
	pub fn resoudre( self )-> bool where Self: Sized { 
		*&self.pile[1..] 
			.iter()
			.zip( &self.action[..] )
			.fold( 
				self.pile[0], 
				|acc,(etat,liaison)| match liaison {
					Jeton::LiaisonOu => acc | *etat,
					Jeton::LiaisonEt => acc & *etat, 
					_ => panic!( "l'arbre résolutoire des conditions est incorrect" ) 
				} 
			) 
	} 
} 
