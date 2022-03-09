//! # '_Règle_'. Organiser son traitement. Sans se désorganiser.
//!
//! Le programme _Règle_ est un moteur de règles, c'est-à-dire qu'il exécute des règles déclarées, suivant leur état individuel (schéma habituel _Si... Alors... Sinon... Finalement..._). Le moteur cherchera dans son fonctionnement, à optimiser fortement la gestion des états, notamment si une règle n'a pas été déclenchée afin d'éviter de refaire des opérations. 
//!
//! Le programme fonctionne à partir des E/S standard, et repose sur des lignes en simili-Bash pour communiquer avec les autres processus. A ce jour, seulement un mode dit "asservi" est supporté : le programme est supervisé par un exécuteur, qui jouera les demandes (appelable de contexte ou d'actions). C'est le programme superviseur qui gère les variables ; le moteur de règle n'a notion que d'états booléens pour savoir si une règle pourrait être déclenchée ou non. 
//! 
//! ## Principales explications et exemples 
//! 
//! D'abord quelques points importants : 
//!   - l'_exécuteur_, qui est ici un processus qui supervise le programme _Règle_ (c-à-d qui l'exécute et garde le contrôle de ses E/S), est le seul qui a accès aux variables et qui a la charge d'exécuter les opérations ; 
//!   - ... 
//! 
//! _nb : Pour des fins de lisibilités, il est préférable de fortement séparer les différents éléments de rédaction de votre jeux de règles : cela facilite la relecture et le parseur n'a pas de restriction._ 
//! 
//! ### Types de données disponibles 
//! 
//! Le programme gère dans ses déclarations de règles seulement trois types : les 'Textes', les 'Nombres' et les 'Variables'. Les 'Appelables' (qui peuvent servir à définir un état ou appeler une action) comme les renvois ou les conditionnels, sont des types dérivés des types fondamentaux (sous le format d'une symbole suivit d'un Texte). 
//! 
//! ```
//! "Ceci est texte" # Texte, toujours entre guillemets 
//! ```
//! 
//! ```
//! 20    # Nombre 
//! 20.5  # Nombre 
//! -20.5 # Nombre 
//! ```
//! 
//! ```
//! ma_variable # Variable  
//! ma_fonction() # Variable + arguments* -> Appelable (clause ou action) 
//! ?"ma condition" # symbole + Texte -> Conditionnel 
//! !"mon renvoi vers une règle" # symbole + Texte => Renvoi 
//! ```
//! 
//! ### Définir une 'Variable' 
//!
//! Une Variable est la déclaration de la liaison entre une Variable et une valeur d'un type des deux types fondamentaux (Texte ou Nombre). 
//!
//! ```
//! Variable message_bonjour: 
//!   "bonjour" 
//! ```
//!
//! Notez que la définition des variables n'est pas dédié directement au moteur : ce dernier doit simplement signifier les valeurs de contexte initial à l'exécuteur (le programme distant). __Si plusieurs définitions sont trouvées, la plus récente sera gardée.__ 
//!
//! ### Définir une 'Condition' 
//! 
//! Une Condition est le regroupement de plusieurs 'Clauses'. Une clause est un état, fruit de l'exécution d'un appelable passé à l'exécuteur : une clause _ne peut pas être une valeur_. Plusieurs clauses peuvent être liées ensembles au travers deux opérateurs logiques : `et` et `ou`. La condition peut aussi n'avoir qu'une seule clause, comme suit : 
//! 
//! ```
//! Condition "est membre": 
//!   client.est_membre( 50, "jours" ) 
//! ```
//! 
//! Si aucune parenthèse ou regroupement n'est admis dans les conditions, vous pouvez appeler d'autres conditions (comme étant des clauses) au sein d'une parente. Cela revient à "regrouper" et le résultat se comportera comme si vous aviez eu recourt à des parenthèses. En plus efficace : pas besoin d'avoir à retaper les mêmes conditions et les mêmes clauses à chaque nouvelle règle... 
//! 
//! ```
//! Condition "offre spéciale": 
//!   ?"est membre"
//!   et 
//!   client.total_historique( ceci_est_une_variable_locale_a_l_executeur, ">", 1000 ) 
//! ```
//! 
//! Une clause définie de la même façon à plusieurs endroits (y compris dans l'ordre des arguments), aura un seul objet en mémoire la représentant au sein d'un contexte de résolution. Ainsi il n'est pas dommageable dans la création des conditions ou dans un jeu de conditions embriquées, d'avoir plusieurs fois le même appels à une même clause : il n'y aura qu'une seule appelée / exécutée à la fin. 
//! 
//! Il est à noter que si l'exécuteur exécute un appelable qui a des effets de bord, le résultat pourrait être inattendu (et catastrophique). Sachant qu'une clause peut être seulement une fois au travers de plusieurs conditions voire de plusieurs règles, la fonction distante doit être la plus "pure" (au sens où elle marginalise le risque d'avoir un retour différent à chaque appel, ou de provoquer un changement d'état de l'exécuteur lui-même). 
//! 
//! ### Définir une 'Règle' 
//! 
//! 
//! 
//! ## Licences et support 
//!
//! Fruit d'un travail de recherche et de réflexion, ce programme est mis à disposition sans aucune garantie de fonctionnement, et pour celle restriction, le respect [de la licence MIT](https://fr.wikipedia.org/wiki/Licence_MIT). Il est pensé prioritaire pour des systèmes UNIX/Linux. 
//!
//! Pour toute garantie ou développement particulier, vous pouvez vous rapprocher de l'auteur Julien Garderon ([Linkedin](https://www.linkedin.com/in/julien-garderon-13a3a920), [Twitter](https://twitter.com/intelligencepol), [Github](https://github.com/JGarderon)). 
//!
#![allow(warnings, unused)] 

mod communs; 
use crate::communs::Types; 

mod grammaire; 
use crate::grammaire::constructeur::construire as environnement_construire; 

mod resolution; 
use crate::resolution::contexte::Contexte;
use crate::resolution::contexte::construire as contexte_resolution; 
use crate::resolution::executer as resolution_executer; 

fn main() { 
	match resolution_executer() { 
		Ok( _ ) => (), 
		Err( erreur ) => eprintln!( "# erreur : {}", erreur ) 
	}; 
} 



