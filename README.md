# Valhalla Registry WebAdmin

The frontend for the Valhalla Registry

## Sidebar

```
Dashboard
    Overview
    Security
    Performance
Crate
    Overview
    Versions
    Dependencies
    Dependents
    Documentation
    Audits
    Settings
        Owners
Directory
    Accounts
    Groups
    Roles
Settings
    Server
        TSL
    Doc builder
    Audit queue
Management
Maintenance
    Reload configuration
    Update webadmin / backend
    Restart backend
    Rebuild search index
```
## Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080