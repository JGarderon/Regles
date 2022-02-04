#!/usr/bin/env python3 

import sys, fileinput, json 

def envoyer_etat( m ): 
    print( 
        m, # .encode("utf-8") 
        file=sys.stderr, 
        flush=True 
    ) 

envoyer_etat( "OP-DEBUT" ) 

for line in fileinput.input(): # encoding="utf-8" 
    # print("-------", line, flush=True)
    try:
        if line.strip() == "OP-STOP": 
            # print( "demande de fin reçue du parent intermédiaire", flush=True ) 
            break 
        o = json.loads( line ) 
        o["!"] = "parfait" 
        print(json.dumps( o ), flush=True) 
        envoyer_etat( "OP-FIN" ) 
    except Exception as e:
        break 

envoyer_etat( "OP-STOP" ) 

