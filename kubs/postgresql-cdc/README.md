```fish
# Clone postgres repository
git clone --branch REL_17_4 --depth=1 https://github.com/postgres/postgres.git

# Configure
pushd ./postgres && ./configure                         \
    # Build libpgcommon
    && pushd ./src/common && make && popd               \
    # Build libpgport
    && pushd ./src/port   && make && popd               \
    # Build libpq
    && pushd ./src/interfaces/libpq && make && popd     \
    && popd
```

```fish
# Print default postgresql configuration.
podman run -i --rm docker.io/postgres:17.4 cat /usr/share/postgresql/postgresql.conf.sample > postgres.conf.sample
```

```sql
-- Show configuration value
SHOW config_file;
SHOW hba_file;
```

https://github.com/jackc/pgx
https://github.com/jackc/pglogrepl.git
https://github.com/ConduitIO/conduit-connector-postgres.git
