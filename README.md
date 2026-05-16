# MiaSQL

## Description
MiaSQL will be an open-source database that will be free to use for non-profit or private entities. Please check the license agreement.
You can interact with the database via a socket connection and use this shell to enter your commands.

## Documentation
We are working on the documentation - it will be on the website.
Until then please check the manual.md file.

To connect to the database open a PuTTY like shell with port 7878 - there you can enter your sql commands.
In the future this will become a ssh connection, but right now we will be using a regular shell.

## License
Sorry - we have not yet decided which licensing model we choose.
As long as there is nothing else specified, you can look at the code, but you are not allowed
to fork, change, sell, modify, distribute or use this code in any other way or form unless it is with our explicit permission.

## Website
We don't have a website yet - we are working on it, but lets get the database running first.

## Roadmap
### Current Version is: 0.0.x
| Version | Goal                                                                                               |
|---------|----------------------------------------------------------------------------------------------------|
| 0.1.0   | all basic sql commands have been implemented* and are working with the shell                       |
| 0.2.0   | adding datatypes char/binary/varbinary/blob/tinyblob/mediumblob/longblob/text/tinytext/mediumtext/ |
| 0.3.0   | moi files are updated on the fly                                                                   |
| 0.4.0   | the ledger is implemented                                                                          |
| 0.5.0   | updating the b-tree on the fly                                                                     |
| 0.6.0   | nested functions                                                                                   |
| 0.7.0   | implementing order by / group by / having  - constraints                                           |
| 0.8.0   | implementing joins and views                                                                       |
| 0.9.0   | implementing aggregate functions (count/sum/avg etc..) and subqueries                              |
| 0.10.0  | implementing String functions                                                                      |
| 0.11.0  | implementing date and time commands                                                                |
| 0.12.0  | implementing conditional expressions                                                               |
| 0.13.0  | implementing user management                                                                       |
| 0.14.0  | use ssh as shell and not standard socket                                                           |
| 0.15.0  | backup importer for ledger and other databases                                                     |
| 0.16.0  | Interfaces for Python / C / Java / Rust / C# / TypeScript                                          |
| 0.17.0  | sharding                                                                                           |
| 0.18.0  | clusters                                                                                           |






\* no nested commands / no joins / no groups / no orders / no having 
/ no subqueries (IN / ANY / ALL) / no count() / no sum() / no avg() / no min() / no max() / no String functions / no date and time commands / no conditional expressions / no set operations / no transactional control commands 

