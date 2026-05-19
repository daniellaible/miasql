
# To Do's

## Commands

+ CREATE_TABLE
  + add Tokens in the enum that represent the constraints
  + CREATE TABLE Persons (PersonID int PRIMARY KEY, LastName varchar(255) NOT NULL, FirstName varchar(255), Address varchar(255), City varchar(255));
  + CREATE TABLE GermanCustomers AS SELECT * FROM Customers WHERE Country = 'Germany'; 
+ CREATE_DATABASE
  + DROP DATABASE testDB;
+ DROP_TABLE 
  + DROP TABLE Shippers;
+ DROP_DATABASE
  + DROP DATABASE testDB;
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

## MISC
+ WhereClause can not handle anything else than BigInt - should be able to handle VarChars too


## Datatypes

## Database
+ column definitions need to have constraints, not only just names 

