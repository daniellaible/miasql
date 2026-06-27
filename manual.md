# MiaSQL

## Preface

MiaSQL was born out of curiosity. Daniel wanted to understand the way SQL databases work.
Therefore, he decided to program his own db project. He published his project on github and nobody noticed.

The word 'mia' means 'my' in Esperanto. To name the software in Esperanto was chosen deliberately. Esperanto was created as a world
language, a language to unite. Mia was created with the same intentions, to provide a tool for all people, a tool that is free
from third party tinkering, a tool that does not need to earn dividends for some shareholders.

## General Information

## Licencing
### Community License
### Enterprise License

## Installing Mia
### Installing Mia on Windows

### Installing Mia on Linux

### Installing Mia on Mac

### Configuring Mia
If you are using the installer, you will be guided through the configuration process.
If you are not using the installer, you will need to configure Mia manually via a yaml file.

### Connecting to Mia
Connecting to the database is simple - just use PuTTY and connect to your db-server on port 7878.

## Language Structure
### Basic Language Settings

##### Bye | Quit
> Bye  | Quit 

This closes your session with the Mia Server

#### Use
> USE <database_name>

This specifies which database you want to use

#### Show tables
> SHOW TABLES

Returns a list of all tables of the selected database. You select a database by using the USE command.

#### Datatypes

These are the datatypes we are currently supporting.
* BigInt { 64 bit },
* Int { 32 bit },
* SmallInt { 16 bit },
* TinyInt { 8 bit },
* Decimal { 32 bit },
* Float { 64 bit },
* VarChar {u16, String },
* Bool { bool },
* Date { 64 bit } ... nope - not supported yet,
* Time { 64 bit } ... nope - not supported yet,
* DateTime { 64 bit } ... nope - not supported yet

#### Data Queries

##### Select

Syntax of a basic select statement:
> SELECT < column1, column2, ... > FROM < table_name > WHERE condition;

Syntax of a basic select statement for all columns:
> SELECT * FROM < table_name > WHERE condition;  

Select all unique entries from a table
> SELECT DISTINCT < table_name > FROM < table_name >;

Select with an order
> SELECT <column1, column2, ... > FROM < table_name> ORDER BY column1, column2, ... ASC|DESC;

Select with grouping
> SELECT < column1, aggregate_function(column2), column3, ... > FROM < table_name > WHERE < condition > GROUP BY < column1, column3 > ORDER BY < column_name >;

Syntax top command
> SELECT TOP number FROM table_name WHERE condition;

Todo: here we need the Joins | aggregate functions | regular functions

#### Data Manipulations

##### Create

###### Create Table

###### Create Database

##### Delete

##### Drop

###### Drop Table

###### Drop Database

##### Truncate
Syntax of the Truncate command
> TRUNCATE TABLE < column1, column2 ... >;

Truncate deletes all data from a table without deleting the table itself. 

##### Alter

##### Insert

##### Update
>

##### Delete
Syntax of a delete statement:
> DELETE FROM < table > WHERE < condition >

Delete needs a table and a where-clause (condition). Without a where-clause other db-systems truncate the whole table. 
Mia does not allow such unintended behavior, therefore, use truncate to delete all data from the table
but use the delete statement to delete only specific data.


### Operators and Functions 

### Security

## The Ledger
The ledger is a file on your hard-drive. Every Mia-DB has a ledger. The ledger stores all
commands which are altering the database and it's data. So the ledger stores not only
CREATE and ALTER commands but also INSERT commands etc... . SELECT commands and user permissions are
not stored in the ledger. The first entry in the ledger is always a CREATE DATABASE command.
The ledger is the build-in backup system of the database.

There is a plan on building a ledger viewer.

## Connectors and APIs
Connecting to the database is simple - just use PuTTY and connect to your db-server on port 7878.
## Clusters

## Shards

## Backup and Recovery

## Additional software to support Mia

## The basic functionality of Mia

+ mos file: list of all tables and where they are stored
+ moi file: table on the drive
+ cmon file: config file for mia

		TCP/IP
		 7878
		  || 
	       COMMAND   -----> Parser  
         Sql-Command <-----
		  ||
		  ||
		  ||
		  ||      -----> Ledger (stores command tokens on disc)
		  ||
		  ||      -----> moi-file Updater (stores data on disc)
              ||      -----> ledger updates field DataOnDisc
		  ||
		  ||      -----> update b-tree (stores tables in memory)		
		  ||      -----> update cluster or shards
		  ||
		  ||
		 \  /
		  \/		
	   
	       Response  

## Syntax
"DROP TABLE IF EXISTS table_name;" is not supported use "DROP TABLE table_name" instead.
This is true for Dropping Databases as well.

What we do support:

SELECT DISTINCT avg(amount), sum(name), lastname
    FROM employee
    WHERE id = 'foo'
    GROUP BY lastname
    ORDER BY lastname;

SELECT lastname
    FROM employee
    WHERE id = 1
    ORDER BY lastname DESC;

SELECT *
FROM employee
WHERE id = 1
ORDER BY lastname DESC;

SELECT firstname, lastname
    FROM employee
    WHERE id = 1
    GROUP BY firstname, lastname;

SELECT Orders.OrderID, Customers.CustomerName, Orders.OrderDate
    FROM Orders
    INNER JOIN Customers ON Orders.CustomerID=Customers.CustomerID;

