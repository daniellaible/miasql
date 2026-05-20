
# To Do's

## Commands

### Base Commands
+ CREATE_TABLE
  + add Tokens in the enum that represent the constraints
  + CREATE TABLE Persons (PersonID int PRIMARY KEY, LastName varchar(255) NOT NULL, FirstName varchar(255), Address varchar(255), City varchar(255));
  + CREATE TABLE GermanCustomers AS SELECT * FROM Customers WHERE Country = 'Germany'; 
+ CREATE_DATABASE
+ ALTER 
  + ALTER TABLE Customers ADD Email varchar(255); 
  + ALTER TABLE Employees ALTER COLUMN BirthDate BIGINT;
  + ALTER TABLE Customers DROP COLUMN ContactName
  + ALTER TABLE Persons DROP CONSTRAINT UC_Person;
  + ALTER TABLE Persons DROP INDEX UC_Person;
  + ALTER TABLE Persons DROP CONSTRAINT PK_Person;
  + ALTER TABLE Persons DROP PRIMARY KEY;
  + ALTER TABLE Orders DROP CONSTRAINT FK_PersonOrder;
  + ALTER TABLE Orders DROP FOREIGN KEY FK_PersonOrder;
+ TRUNCATE
  +  TRUNCATE TABLE Categories;

### Joins
+ Inner Join
+ Outer Join
+ Left Join
+ Right Join
+ Self Join
+ Full Join

### KeyWords


### Functions and Operators

#### Operators
 + And
 + Or
 + Not
 + UNION ALL
 + In
 + Between
 + Group By
 + Having
 + Exists ?
 + Any
 + All
 + 

#### Math:
  + Trim
  + ABS
  + ASIN
  + ATAN
  + AVG
  + CEIL
  + COS
  + COT
  + COUNT
  + FLOOR
  + LOG
  + MAX
  + MIN
  + PI
  + POWER
  + RAND
  + SIN
  + SQRT
  + SUM 
  + TAB
  + TRUNCATE

#### String:
  + ASCII
  + CONCAT
  + FORMAT
  + INSERT
  + REPLACE
  + SUBSTRING
  + TRIM
  + UPPER
  + LIKE

## MISC
+ WhereClause can not handle anything else than BigInt - should be able to handle VarChars too

## Datatypes
  + CHAR
  + BINARY
  + VARBINARY
  + BLOB
  + TINYBLOB
  + MEDIUMBLOB
  + LONGBLOB
  + TEXT
  + MEDIUMTEXT
  + LONGTEXT


## Database
+ column definitions need to have constraints, not only just names 

