
#[derive(Debug)] 
enum Jeton { 
	Etat(bool), 
	LiaisonOu, 
	LiaisonEt,
	GroupeOuvrant, 
	GroupeFermant 
} 

#[derive(Debug)]
struct Contexte {
	pile: Vec<bool>, 
	action: Vec<Jeton> 
} 

impl Contexte {
	fn creer() -> Self {
		Contexte { 
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

	let mut contextes = vec!( Contexte::creer() ); 

	for element in liste { 
		match element {
			Jeton::Etat( e ) => { 
				&contextes[..].last_mut().unwrap().pile.push( e ); 
			},
			Jeton::LiaisonOu | Jeton::LiaisonEt => { 
				&contextes[..].last_mut().unwrap().action.push( element ); 
			}, 
			Jeton::GroupeOuvrant => {
				contextes.push( Contexte::creer() ); 
			}, 
			Jeton::GroupeFermant => { 
				let contexte = contextes.pop().unwrap(); 
				let e = contexte.resoudre(); 
				&contextes[..].last_mut().unwrap().pile.push( e ); 
			} 
		} 
	} 

	let contexte = contextes.pop().unwrap(); 

	let etat = contexte.resoudre(); 

	println!("résultat final = {:?}", etat);

} 




