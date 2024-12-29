CREATE OR REPLACE TEMP MACRO get_pokemon(lang_id, id) AS TABLE
SELECT
    {
        'id': pokemon.id,
        'identifier': pokemon.identifier,
        'weight': pokemon.weight,
        'height': pokemon.height,
        'base_experience': pokemon.base_experience,
        'abilities': (
            SELECT ARRAY_AGG(STRUCT_INSERT(
                (SELECT * FROM get_ability(lang_id, ab.ability_id)),
                slot := ab.slot
            )) AS data
            FROM pokemon_abilities AS ab
            WHERE ab.pokemon_id = pokemon.id
        ),
        'egg_groups': (
            SELECT ARRAY_AGG((
                SELECT data FROM get_egg_group(lang_id, egg.egg_group_id)
            )) AS data
            FROM pokemon_egg_groups AS egg
            WHERE egg.species_id = pokemon.species_id
        ),
        'items': (
            SELECT 
                ARRAY_AGG((
                    SELECT data FROM get_item(lang_id, item.item_id)
                )) AS data
            FROM (
                SELECT DISTINCT ON (item.item_id) item.item_id
                FROM pokemon_items AS item
                WHERE item.pokemon_id = pokemon.id
            ) AS item
        ),
        'stats': MAP_FROM_ENTRIES((
                SELECT ARRAY_AGG((stats.identifier, {
                    'base': stat.base_stat,
                    'effort': stat.effort,
                    'identifier': stats.identifier,
                    'name': name.name,
                }))
                FROM pokemon_stats AS stat
                LEFT JOIN stats
                    ON stat.stat_id = stats.id
                LEFT JOIN languages AS lang
                    ON lang.identifier = lang_id
                LEFT JOIN stat_names AS name
                    ON name.stat_id = stats.id 
                    AND name.local_language_id = lang.id
                WHERE
                    stat.pokemon_id = pokemon.id
        )),
        'types': (
            SELECT ARRAY_AGG(STRUCT_INSERT(
                (SELECT data FROM get_type_base(lang_id, type.type_id)),
                slot := slot
            ))
            FROM pokemon_types AS type
            WHERE
                type.pokemon_id = pokemon.id
        ),
        -- Evolution
        'moves': (
            SELECT ARRAY_AGG(STRUCT_INSERT(
                (SELECT data FROM get_pokemon_move(lang_id, move.move_id)),
                methods := move.meta
            )) FROM (
                SELECT
                    move.move_id AS move_id,
                    move.pokemon_id AS pokemon_id,
                    ARRAY_AGG(DISTINCT method.identifier) AS meta,
                FROM pokemon_moves AS move
                LEFT JOIN pokemon_move_methods AS method
                    ON method.id = move.pokemon_move_method_id
                WHERE move.pokemon_id = pokemon.id
                GROUP BY move.move_id, move.pokemon_id
                ORDER BY move.move_id DESC
            ) AS move
        ),
    } AS data
FROM pokemon
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
WHERE
    pokemon.id = id
;
