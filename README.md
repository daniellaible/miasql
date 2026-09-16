# MiaSQL

## Description

MiaSQL will be an open-source database that will be free to use for non-profit or private entities. Please check the
license agreement.
You can interact with the database via a socket connection and use this shell to enter your commands.

## Documentation

We are working on the documentation – it will be on the website.
On how to use the database, please look at the manual.md file.

There is always only instruction the engine is working on; however, it uses multithreading to access
all the different data structures that are used for this task;

To connect to the database open a PuTTY like shell with port 7878 - there you can enter your sql commands.
In the future this will become a ssh connection, but right now we will be using a regular shell.

## License

There will be two versions of MiaSql: the first one will be a community edition that will be free of cost.
Second, there will be an enterprise edition that offers everything the community edition offers,
additionally, there will be clusters and shards available as well as a multi-tier support.
Both editions community and enterprise use the same Mia-engine.

## Website

We don't have a website yet - we are working on it, but lets get the database running first.

## Roadmap

### Current Version is: 0.1.x

| Version | Goal                                                                                                                           |
|---------|--------------------------------------------------------------------------------------------------------------------------------|
| 0.1.0   | system files of the db can be read, system tables are created in memory                                                        |
| 0.2.0   | basic select, create, insert, alter, truncate and drop statements are working in ram and on disc, also all columns are indexed |
| 0.3.0   | functionality of all statements is guaranteed - including foreign keys,constraints, joins etc                                         |
| 0.4.0   | user management, secure shell, multithreading -> engine functional                                                                             | 
| 0.5.0   | Speed optimization and additional testing                                                                                      | 
| 0.6.0   | drivers for the most popular languages are provided                           |
| 0.7.0   | ledger implementation including restoring the backup, zipping - backup importer from other databases                                                                            |
| 0.8.0   | UI for interacting with Mia is provided                                                                                        |
| 0.9.0   | adding clusters, shards and loadbalancer                                                                                       |
| 1.0.0   | amaze - amaze - amaze - Question                                                                                               |
