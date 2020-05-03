![Rust Reader](https://github.com/PetitRoBERT/back-roBERT/workflows/rust_reader/badge.svg)

# Petit RoBERT

## Develop

To launch the whole project in development, run:

```bash
docker-compose -f docker-compose.dev.yml build

docker-compose -f docker-compose.dev.yml up
```

To use the debugger use this `launch.json` file in VsCode:
```json
    {
        "version": "0.2.0",
        "configurations": [
            {
                "type": "node",
                "request": "attach",
                "name": "Debug: api-gateway-rest",
                "remoteRoot": "services/api-gateway",
                "localRoot": "${workspaceFolder}",
                "protocol": "inspector",
                "port": 9229,
                "restart": true,
                "address": "0.0.0.0",
                "skipFiles": ["<node_internals>/**"]
            },
            {
                "type": "node",
                "request": "attach",
                "name": "Debug: api-gateway-database",
                "remoteRoot": "services/api-gateway",
                "localRoot": "${workspaceFolder}",
                "protocol": "inspector",
                "port": 9230,
                "restart": true,
                "address": "0.0.0.0",
                "skipFiles": ["<node_internals>/**"]
            }
        ]
    }
```

Now you're ready to go 🚀!
