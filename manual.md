# MiaSQL

## Preface
MiaSQL was born out of curiosity. Daniel wanted to understand the way SQL databases work.
Therefor he decided to program his own db project. He published his project on github and nobody noticed.

The word 'mia' means 'my' in Esperanto. To name the software in Esperanto was chosen deliberately. Esperanto was created as a world 
language, a language to unite. Mia was created with the same intentions, to provide a tool for all people, a tool that is free 
from third party tinkering, a tool that does not need to earn dividends for some shareholders. 


## Datatypes

These are the datatypes we are currently supporting. 
* BigInt { 64 bit },
* Int { 32 bit },
* SmallInt { 16 bit },
* TinyInt { 8 bit },
* Decimal { 32 bit },
* Float { 64 bit },
* VarChar { String, SizeOf(Pointer) },
* Bool { bool },
* Date { 64 bit } ... nope - not supported yet,
* Time { 64 bit } ... nope - not supported yet,
* DateTime { 64 bit } ... nope - not supported yet


## The basic functionality of mia

		TCP/IP
		 7878
		  || 
	       COMMAND   -----> Parser  
         Sql-Command <-----
		  ||
		  ||
		  ||
		  ||      -----> Ledger (stores commands on disc)
		  ||
		  ||      -----> moi-file Updater (stores data on disc)
		  ||
		  ||      -----> b-tree handler (stores tables in memory)		
		  ||
		  ||
		  ||
		 \  /
		  \/		
	   
	       Response  

## Connecting to the Database
Connecting to the database is simple - just use PuTTY and connect to your db-server on port 7878.


## The Ledger
The ledger is a file on your hard-drive. Every database has a ledger. The ledger stores all
commands which are altering the database and it's data. So the ledger stores not only
CREATE and ALTER commands but also INSERT commands etc... . SELECT commands and user permissions are 
not stored in the ledger. The first entry in the ledger is always a CREATE DATABASE command.
The ledger is the build-in backup system of the database.

There is a plan on building a ledger viewer.