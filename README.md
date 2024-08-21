# folketinget-sqlite3-mirror
This program when run, will create an sqlite3 database (folketinget.sqlite3) in the working directory and continuously update it as changes are made in the public oda.ft.dk API.

Since the data volume is rather large, the initial synchronization will take a while, in order to not overload the API or run into rate-limiting. Be patient.

Last run I did took about 12 hours before the initial synchronization was done.

# Usage
Run using Docker:
```bash
mkdir data
docker run -e RUST_LOG=info \
    -e FTS_SCRAPER_REQUESTS_PER_SECOND=5 \
    -e FTS_SCRAPER_DOMAIN=oda.ft.dk \
    -e FTS_DATABASE_SQLITE_PATH=data/folketinget.sqlite3 \
    -v $(pwd)/data:/data \
    ghcr.io/datavirke/folketinget-sqlite3-mirror:master
```
Collected data will be stored in a `data` directory in your current working directory.
All the environment variables (with the exception of RUST_LOG) contain the default settings. If omitted, the program will default to these same values.

## Notes
Since the different resource types (Dokument, Aktør, DokumentAktør(relation)) are polled in succession and synchronized entirely, before moving onto the new dokument type, the database can never be said to be in a consistent state.

This means that even if a "DokumentAktør" resource exists in the database, you cannot assume that the Dokument or Aktør that the relation refers to exists, when you query it.

If the data is to be used elsewhere, I would suggest implementing a sanitizing step where the data is loaded from folketinget.sqlite3 and validated before being inserted into a known-consistent schema, where foreign keys can be used.
