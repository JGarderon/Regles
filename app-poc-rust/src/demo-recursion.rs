#![allow(warnings, unused)] 


#[derive(Clone,Debug)] 
struct Clause {
	etat: Option<bool> 
} 

#[derive(Clone,Debug)] 
enum Jeton { 
	Clause(Clause), 
	Etat(bool), 
	LiaisonOu, 
	LiaisonEt,
	GroupeOuvrant, 
	GroupeFermant 
} 

#[derive(Debug)] 
struct Condition { 
	etat: Option<bool>, 
	clauses: Vec<Jeton> 
} 

impl Condition { 
	fn creer( clauses: Vec<Jeton> ) -> Self { 
		Condition { 
			etat: None,  
			clauses: clauses 
		} 
	} 
	// fn preparer( &mut self ) -> Vec<String> { 

	// } 
	fn resoudre( &mut self ) { 
		let mut condition = vec!( Feuille::creer() ); 
		for element in &self.clauses { 
			match element {
				Jeton::Clause( c ) => {
					&condition[..].last_mut().unwrap().pile.push( c.etat.unwrap() ); 
				}
				Jeton::Etat( e ) => { 
					&condition[..].last_mut().unwrap().pile.push( *e ); 
				},
				Jeton::LiaisonOu | Jeton::LiaisonEt => { 
					&condition[..].last_mut().unwrap().action.push( element.clone() ); 
				}, 
				Jeton::GroupeOuvrant => {
					condition.push( Feuille::creer() ); 
				}, 
				Jeton::GroupeFermant => { 
					let e = condition.pop().unwrap().resoudre(); 
					&condition[..].last_mut().unwrap().pile.push( e ); 
				} 
			} 
		} 
		let feuille = condition.pop().unwrap(); 
		self.etat = Some( feuille.resoudre() ); 
	}
}

#[derive(Debug)]
struct Feuille {
	pile: Vec<bool>, 
	action: Vec<Jeton> 
} 

impl Feuille {
	fn creer() -> Self {
		Feuille { 
			pile: vec!(), 
			action: vec!() 
		} 
	} 
	fn resoudre( self ) -> bool { 
		*&self.pile[1..] 
			.iter()
			.zip( &self.action[..] )
			.fold( 
				self.pile[0], 
				|acc,(etat,liaison)| match liaison {
					Jeton::LiaisonOu => acc | *etat,
					Jeton::LiaisonEt => acc & *etat, 
					_ => panic!( "l'arbre résolutoire est incorrect" ) 
				} 
			) 
	} 
}

fn main() {

	let liste = vec!( 
		Jeton::GroupeOuvrant, 
			Jeton::Etat(true), 
			Jeton::LiaisonOu, 
			Jeton::Etat(false), 
			Jeton::LiaisonEt, 
			Jeton::Etat(true), 
		Jeton::GroupeFermant, 
		Jeton::LiaisonEt, 
		Jeton::GroupeOuvrant, 
			Jeton::Etat(true), 
			Jeton::LiaisonEt, 
			Jeton::GroupeOuvrant, 
				Jeton::Etat(true), 
				Jeton::LiaisonOu, 
				Jeton::Etat(false), 
			Jeton::GroupeFermant, 
		Jeton::GroupeFermant, 
		Jeton::LiaisonEt, 
		Jeton::Etat(true) 
	); 

	let mut condition = Condition::creer( liste ); 
	
	println!( 
		"résultat final = {:?}",  
		condition.resoudre() 
	); 

} 




