```fish
# Load data
pushd http-server
git clone https://github.com/PokeAPI/pokeapi.git --depth=1
ls -p pokeapi/data/v2/csv  \
| grep -v '/$'          \
| sed 's/.csv//g'       \
| xargs -I{} -n 1 sh -c "xsv fixlengths './pokeapi/data/v2/csv/{}.csv' | duckdb pokeapi.db \"CREATE TABLE {} AS FROM read_csv('/dev/stdin');\""

popd

# Build http server image
podman build -t var-http-server ./http-server

# Add netavark network backend if needed
# So that we can use service name as its domain name.
echo '[network]
network_backend = "netavark"' >> $XDG_CONFIG_HOME/containers/containers.conf

# Start services
podman compose up

# Enter varnish container
podman compose exec varnish bash

# Show varnish logs
varnishlog
```
