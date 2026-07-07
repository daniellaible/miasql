# MiaSQL

## Description

MiaSQL will be an open-source database that will be free to use for non-profit or private entities. Please check the
license agreement.
You can interact with the database via a socket connection and use this shell to enter your commands.

## Documentation

We are working on the documentation – it will be on the website.
On how to use the database, please look at the manual.md file.

The B-Tree itself (table) is always single-threaded, so there is always only one worker on one B-Tree,
however, there can be multiple B-Trees in RAM storage - this will be then multithreaded.

With clusters there are multiple instances of the same table (B-Tree) available for the users.
On Shards you have different tables of the same database available (no read and write from the disc),
on Clusters you have the same data available for parallel use.

To connect to the database open a PuTTY like shell with port 7878 - there you can enter your sql commands.
In the future this will become a ssh connection, but right now we will be using a regular shell.

## License

Sorry - we have not yet decided which licensing model we choose.
As long as there is nothing else specified, you can look at the code, but you are not allowed
to fork, change, sell, modify, distribute or use this code in any other way or form unless it is with our explicit
permission.

## Website

We don't have a website yet - we are working on it, but lets get the database running first.

## Roadmap

### Current Version is: 0.1.x

| Version | Goal                                                                                             |
|---------|--------------------------------------------------------------------------------------------------|
| 0.1.0   | system files of the db can be read, system tables are created in memory                          |
| 0.2.0   | basic select, create, insert, alter, truncate and drop statements are working in ram and on disc |
| 0.3.0   | indexing is implemented on the tables                                                            |       
| 0.4.0   | all the functions work on the select statement                                                   |
| 0.5.0   | implementing inner, outer, left, right and full joins                                            |    
| 0.6.0   | user management and secure shell                                                                 |
| 0.7.0   | ledger implementation including restoring the backup and zipping (Alpha Release)                 
| 0.8.0   | backup importer from other databases                                                             |
| 0.9.0   | adding clusters, shards and loadbalancer                                                         |
| 1.0.0   | amaze - amaze - amaze - Question                                                                 


Interfaces for Python / C(++) / Java / Rust / C# / TypeScript / PHP / VB        
UI for Mia