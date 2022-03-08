
use std::io::{self, Write}; 
use crate::grammaire::constructeur::Environnement; 

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
	Erreur(&'static str) 
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
	pub fn parler( &mut self, message: &str ) -> std::result::Result<String, &'static str> { 
		io::stdout().write_all( format!( "{}\n", message ).as_bytes() ); 
		io::stdout().flush(); 
		match io::stdin().read_line( &mut self.tampon ) {
			Ok( taille ) => match taille { 
				0 => Err( "Une erreur est survenue : le stdin du processus renvoie une valeur nulle" ), 
				_ => { 
					let r = self.tampon.clone(); 
					self.tampon.clear(); 
					Ok( r ) 
				} 
			} 
			Err( _ ) => Err( "Impossible de lire l'entrée du processus pour récupérer le retour" ) 
		} 
	} 
	pub fn soumettre( &mut self, fct: &String, args: &Vec<Types> ) -> Result<bool, &'static str> { 
		io::stdout().write_all( 
			format!( "{}\n", 
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
				0 => Err( "Une erreur est survenue : le stdin du processus renvoie une valeur nulle" ), 
				_ => { 
					let r = match self.tampon.trim_end() { 
						"v" => Ok( true ), 
						"f" => Ok( false ), 
						_ => { 
							println!( 
								"# DUMP, retour de soumission d'un appelable : {:?}", 
								self.tampon.trim_end() 
							); 
							Err( "Le processus a récupéré une ligne indéterminée ; le DUMP a été mis en console de celui-ci" ) 
						} 
					}; 
					self.tampon.clear(); 
					r 
				} 
			} 
			Err( _ ) => Err( "Impossible de lire l'entrée du processus pour récupérer le retour" ) 
		} 
	} 
	pub fn initier( &mut self, environnement: &Environnement ) -> Result<bool,&'static str> { 
		match self.parler( "initier" )?.trim_end() { 
			"o" => (), 
			"a" => return Ok( false ), 
			"n" => return Err( "Le processus distant n'est pas prêt à exécuter les consignes du moteur de règles" ), 
			_ => return Err( "Le processus distant a répondu hors des valeurs autorisées au moment de l'initialisation générale" ) 
		} 
		for (_, variable) in environnement.variables.iter() { 
			let message = match variable { 
				Types::Nombre( n ) => n.to_string(), 
				Types::Texte( t ) => format!( "\"{}\"", t.to_string() ), 
				Types::Variable( v ) => format!( "${}", v.to_string() ), 
				_ => return Err( "Définition de variable invalide lors de l'initialisation du contexte inter-processus" ) 
			}; 
			match self.parler( &format!( "definir {}", message )[..] )?.trim_end() { 
				"o" => (), 
				"n" => return Err( "Une initialisation de variable a été rejetée par le processus distant" ), 
				_ => return Err( "Le processus distant a répondu hors des valeurs autorisées au moment de l'initialisation d'une variable" ) 
			} 
		} 
		Ok( true ) 
	} 
} 

