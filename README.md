# hireufrj

1. `sudo apt-get install postgresql-12`
2. `sudo apt-get install libpq-dev`
3. `sudo service postgresql start`
4. `sudo su postgres`
5. `psql postgres`
6. `cargo install diesel_cli --no-default-features --features postgres`
7. `createdb -h localhost -p 5432 -U postgres hiringufrj`
8. `\l`
9. `diesel setup`
10. `\c hiringufrj`
11. `diesel migration generate hireufrj`
12. `diesel migration run`