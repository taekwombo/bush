CREATE OR REPLACE TEMP MACRO get_pokemon_move_meta(lang_id, pokemon_id, move_id) AS TABLE
SELECT
    {
        'method': {
            'id': method.id,
            'identifier': method.identifier,
            'name': name.name,
        },
        'level': move.level,
        'version_groups': ARRAY_AGG(move.version_group_id),
    } AS data
FROM pokemon_moves AS move
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN pokemon_move_methods AS method
    ON method.id = move.pokemon_move_method_id
LEFT JOIN pokemon_move_method_prose AS name
    ON name.pokemon_move_method_id = method.id
    ANd name.local_language_id = lang.id
WHERE
    move.pokemon_id = pokemon_id
    AND
    move.move_id = move_id
GROUP BY
    move.pokemon_move_method_id,
    move.level,
    method.id,
    method.identifier,
    name.name
;
