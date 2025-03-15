## Generating otel data using `telemetrygen`

```fish
# Generate traces
telemetrygen traces --otlp-insecure --otlp-endpoint=localhost:4317 --traces 3

# Generate metrics
telemetrygen metrics --otlp-insecure --otlp-endpoint=localhost:4317 --metrics 3
```

## Grafana

```fish
# Add netavark network backend if needed
# So that we can use service name as its domain name.
echo '[network]
network_backend = "netavark"' >> $XDG_CONFIG_HOME/containers/containers.conf

# Start servicess
podman compose up
```

## Grafana Alloy

```fish
# Clone Alloy
git clone https://github.com/grafana/alloy.git --depth=1

# Start services - needs docker due to:
# > can't set healthcheck.start_interval as feature require Docker Engine v25 or later
# > Error: executing /usr/libexec/docker/cli-plugins/docker-compose -f alloy.yaml up: exit status 1
# PR: https://github.com/containers/podman-compose/pull/780
docker compose -f ./alloy.yaml up
```

## SigNoz

> This bad boy takes forever to start. Mutations heh.

```fish
# Clone signoz
git clone https://github.com/SigNoz/signoz.git --depth=1

# Start services
podman compose -f ./signoz.yaml up
```
