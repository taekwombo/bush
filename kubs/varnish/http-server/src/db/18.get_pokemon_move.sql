CREATE OR REPLACE TEMP MACRO get_pokemon_move(lang_id, id) AS TABLE
SELECT
    {
        'id': move.id,
        'identifier': move.identifier,
        'type': (SELECT data FROM get_type_base(lang_id, move.type_id)),
        'power': move.power,
        'pp': move.pp,
        'accuracy': move.accuracy,
        'priority': move.priority,
    } AS data
FROM moves AS move
WHERE move.id = id
;
