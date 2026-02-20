// Compile with:
// ```sh
//   deno compile --allow-read=$HOME/.pgpass --allow-env=HOME ./pgpass_cmp.ts
// ```
//
// Config:
// ```fish
//   complete --command "psql" -s h -l host     -a "(pgpass_cmp host (commandline -cp))" 
//   complete --command "psql" -s p -l port     -a "(pgpass_cmp port (commandline -cp))"
//   complete --command "psql" -s d -l dbname   -a "(pgpass_cmp db (commandline -cp))"
//   complete --command "psql" -s U -l username -a "(pgpass_cmp user (commandline -cp))"
// ```

import { Flag } from './cli.ts';

enum CompletionType {
    Host = 'host',
    Database = 'db',
    User = 'user',
    Port = 'port',
}

const PsqlFlag = {
    Host: new Flag(['--host', '-h']),
    Port: new Flag(['--port', '-p']),
    User: new Flag(['--username', '-U']),
    Database: new Flag(['--dbname', '-d']),
};

const cmp = Deno.args[0];
const args = Deno.args.slice(1).join(' ').split(/\s/).map((v) => v.trim()).filter((v) => v.length > 0);

let completions: Iterable<string>;

switch (cmp) {
    case CompletionType.Host:
        completions = filterPgPass(readPgPass(), { port: PsqlFlag.Port, user: PsqlFlag.User, database: PsqlFlag.Database })
            .reduce((acc, [host]) => acc.add(host), new Set() as Set<string>)
            .values();

        break;

    case CompletionType.User:
        completions = filterPgPass(readPgPass(), { host: PsqlFlag.Host, port: PsqlFlag.Port, database: PsqlFlag.Database })
            .reduce((acc, [_, __, ___, user]) => acc.add(user), new Set() as Set<string>)
            .values();

        break;

    case CompletionType.Port:
        completions = filterPgPass(readPgPass(), { host: PsqlFlag.Host, user: PsqlFlag.User, database: PsqlFlag.Database })
            .reduce((acc, [_, port]) => acc.add(port), new Set() as Set<string>)
            .values();

        break;

    case CompletionType.Database:
        completions = filterPgPass(readPgPass(), { host: PsqlFlag.Host, port: PsqlFlag.Port, user: PsqlFlag.User })
            .reduce((acc, [_, __, db]) => acc.add(db), new Set() as Set<string>)
            .values();

        break;

    default:
        throw new Error(`Invalid completion type "${cmp}"`);
}

for (const cmp of completions) {
    console.log(cmp);
}

type PgPassLines = [host: string, port: string, db: string, user: string, pass: string][];

function readPgPass(): PgPassLines {
    try {
        const lines = Deno.readTextFileSync(`${Deno.env.get('HOME')}/.pgpass`).split('\n').filter((l) => l.length);
        
        return lines.map((line) => line.split(':')) as PgPassLines;
    } catch (_) {
        return [];
    }
}

type FilterFlags = {
    host?: Flag;
    port?: Flag;
    user?: Flag;
    database?: Flag;
};

function filterPgPass(pgpass: PgPassLines, flagIn: FilterFlags): PgPassLines {
    const flags: Record<string, null | undefined | string> = {};

    for (const [key, flag] of Object.entries(flagIn)) {
        const parsed = flag.parse(args);

        if (parsed && parsed[1] && parsed[1].length > 0) {
            flags[key] = parsed[1];
        }
    }

    return pgpass.filter(([host, port, db, user]) => {
        switch (true) {
            case flags.host && flags.host !== host:
            case flags.port && flags.port !== port:
            case flags.user && flags.user !== user:
            case flags.database && flags.database !== db:
                return false;

            default:
                return true;
        }
    });
}
