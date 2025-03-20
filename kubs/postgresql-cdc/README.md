
```fish
# Print default postgresql configuration.
podman run -i --rm docker.io/postgres:17.4 cat /usr/share/postgresql/postgresql.conf.sample > postgres.conf.sample
```

```sql
-- Show configuration file location;
SHOW config_file;
```
