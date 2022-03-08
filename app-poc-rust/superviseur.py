#!/usr/bin/env python3 

import subprocess 
import random 
import asyncio 
import shlex 
import re 

### ----------------------------------------------

NBRE_PROCESSUS_MAX = 1 
NBRE_ESSAIS_MAX = 1 
TEMPS_SIMULATION_MAX = 0 

### ----------------------------------------------

class Moteur: 

	processus = None 
	regex_definir = re.compile( "definir\s+[^\n]+" ) 
	
	def __init__( self, executeur ): 
		self.processus = subprocess.Popen( 
			"target/release/app-poc-rust", 
			stdin=subprocess.PIPE, 
			stdout=subprocess.PIPE, 
			bufsize=1,
		    universal_newlines=True
		) 
		self.executeur = executeur

	async def lancer( self ): 
		for ligne in self.processus.stdout: 
			ligne = ligne.strip() 
			if await self.poursuire( ligne ) is False: 
				break 

	async def poursuire( self, ligne ): 
		if ligne == "initier": 
			suite = await self.executeur.initier() 
			self.processus.stdin.write( 
				"o\n" if suite else "a\n" 
			) 
			self.processus.stdin.flush() 
			return suite 
		elif self.regex_definir.match( ligne ) is not None: 
			self.processus.stdin.write( 
				await self.executeur.definir( ligne ) 
			) 
		else: 
			self.processus.stdin.write( 
				await self.executeur.faire( ligne ) 
			) 
		return True 


### ----------------------------------------------

class ExecuteurSimulation: 

	nbre = 0 
	maxi = None 
	etat = True 

	def __init__( self ): 
		global NBRE_ESSAIS_MAX 
		self.maxi = random.randint( 1, NBRE_ESSAIS_MAX ) 

	async def initier( self ): 
		global TEMPS_SIMULATION_MAX 
		print( id(self), "exécuteur - initier", "!", self.nbre, "sur", self.maxi, "essai(s)" ) 
		etat = False if self.maxi <= self.nbre else True 
		self.nbre += 1 
		await asyncio.sleep( random.randint( 0, TEMPS_SIMULATION_MAX ) ) 
		return etat 

	async def definir( self, ligne ): 
		global TEMPS_SIMULATION_MAX 
		print( id(self), "exécuteur - définir", ligne ) # shlex.split( ligne ) ) 
		await asyncio.sleep( random.randint( 0, TEMPS_SIMULATION_MAX ) ) 
		return "o\n" 

	async def faire( self, ligne ): 
		global TEMPS_SIMULATION_MAX 
		print( id(self), "exécuteur - faire", ligne ) # shlex.split( ligne ) ) 
		await asyncio.sleep( random.randint( 0, TEMPS_SIMULATION_MAX) ) 
		return "f\n" if random.randint( 1, 5 ) == 5 else "v\n" 

### ----------------------------------------------

async def main( *moteurs ): 
	await asyncio.gather( *moteurs )

if __name__ == "__main__": 
	asyncio.run( 
		main( 
			*list( ( 
				Moteur( ExecuteurSimulation() ).lancer() for i in range( 0, NBRE_PROCESSUS_MAX ) 
			) ) 
		) 
	)  


