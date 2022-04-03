
use std::io::{self, Write}; 
use crate::grammaire::constructeur::Environnement; 

#[derive(Debug)]
pub struct Erreur( Vec<String> ); 

impl Erreur {
	pub fn creer( m_initial: &str ) -> Self {
		Erreur { 0: vec!( format!( "{}", m_initial ) ) } 
	} 
	pub fn creer_chaine( m_initial: String ) -> Self {
		Erreur { 0: vec!( m_initial ) } 
	} 
	pub fn empiler( mut self, m_suivant: &str ) -> Self { 
		self.0.push( format!( "{}", m_suivant ) ); 
		self 
	} 
	pub fn afficher( &self, m_suivant: &str ) { 
		eprintln!( "# ERREUR '{}' - dépilement :", m_suivant ); 
		self.0.iter().all(
			|m| { 
				eprintln!( "# ... {}", m );
				false 
			}
		); 
	} 
}

#[derive(Debug,PartialEq,PartialOrd,Clone)] 
pub enum Types {
	Nombre(f64), 
	Texte(String), 
	Variable(String), 
	Conditionnel(String), 
	Appelable(String, Option<bool>, Vec<Types>), 
	Renvoi(String), 
	Ouverture, 
	Fermeture, 
	Et, 
	Ou, 
} 

pub enum ActionResolution { 
	Continuer, 
	Arreter, 
	Erreur(Erreur) 
} 

pub struct Dialogue { 
	tampon: String 
} 

impl Dialogue { 
	pub fn creer() -> Self { 
		Dialogue { 
			tampon: String::new() 
		} 
	} 
	pub fn parler( &mut self, message: &str ) -> std::result::Result<String, Erreur> { 
		io::stdout().write_all( format!( "{}\n", message ).as_bytes() ); 
		io::stdout().flush(); 
		match io::stdin().read_line( &mut self.tampon ) {
			Ok( taille ) => match taille { 
				0 => Err( 
					Erreur::creer( "Une erreur est survenue : le stdin du processus renvoie une valeur nulle" ) 
				), 
				_ => { 
					let r = self.tampon.clone(); 
					self.tampon.clear(); 
					Ok( r ) 
				} 
			} 
			Err( _ ) => Err( 
				Erreur::creer( "Impossible de lire l'entrée du processus pour récupérer le retour" ) 
			) 
		} 
	} 
	pub fn soumettre( &mut self, fct: &String, args: &Vec<Types> ) -> Result<bool, Erreur> { 
		io::stdout().write_all( 
			format!( "executer {}\n", 
				args.iter().fold( 
					fct.clone(), 
					|acc, item| { 
						format!( 
							"{} {}", 
							acc, 
							match item { 
								Types::Nombre( n ) => n.to_string(), 
								Types::Texte( t ) => format!( "\"{}\"", t.to_string() ), 
								Types::Variable( v ) => format!( "${}", v.to_string() ), 
								_ => format!( 
									"$(erreur: l'item '{:?}' n'est pas au bon format)", 
									item 
								) 
							} 
						) 
					} 
				) 
			).as_bytes() 
		); 
		io::stdout().flush(); 
		match io::stdin().read_line( &mut self.tampon ) {
			Ok( taille ) => match taille { 
				0 => Err( 
					Erreur::creer( "Une erreur est survenue : le stdin du processus renvoie une valeur nulle" ) 
				), 
				_ => { 
					let r = match self.tampon.trim_end() { 
						"v" => Ok( true ), 
						"f" => Ok( false ), 
						_ => {  
							Err( 
								Erreur::creer_chaine( 
									format!( 
										"Le processus a récupéré une ligne indéterminée : '{}'", 
										self.tampon.trim_end() 
									) 
								) 
							)  
						} 
					}; 
					self.tampon.clear(); 
					r 
				} 
			} 
			Err( _ ) => Err( 
				Erreur::creer( "Impossible de lire l'entrée du processus pour récupérer le retour" ) 
			) 
		} 
	} 
	pub fn initier( &mut self, environnement: &Environnement ) -> Result<bool, Erreur> { 
		match self.parler( "initier" )?.trim_end() { 
			"o" => (), 
			"a" => return Ok( false ), 
			"n" => return Err( 
				Erreur::creer( "Le processus distant n'est pas prêt à exécuter les consignes du moteur de règles" ) 
			), 
			_ => return Err( 
				Erreur::creer( "Le processus distant a répondu hors des valeurs autorisées au moment de l'initialisation générale" ) 
			) 
		} 
		for (variable_nom, variable_valeur) in environnement.variables.iter() { 
			let message = match variable_valeur { 
				Types::Nombre( n ) => format!( "\"{}\" {}", variable_nom, n.to_string() ), 
				Types::Texte( t ) => format!( "\"{}\" \"{}\"", variable_nom, t.to_string() ), 
				Types::Variable( v ) => format!( "\"{}\" ${}", variable_nom, v.to_string() ), 
				_ => return Err( 
					Erreur::creer( "Définition de variable invalide lors de l'initialisation du contexte inter-processus" ) 
				) 
			}; 
			match self.parler( &format!( "definir {}", message )[..] )?.trim_end() { 
				"o" => (), 
				"n" => return Err( 
					Erreur::creer( "Une initialisation de variable a été rejetée par le processus distant" ) 
				), 
				_ => return Err( 
					Erreur::creer( "Le processus distant a répondu hors des valeurs autorisées au moment de l'initialisation d'une variable" ) 
				) 
			} 
		} 
		Ok( true )  
	} 
} 

