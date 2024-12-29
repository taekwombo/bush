CREATE OR REPLACE TEMP MACRO get_move_meta(lang_id, move_id) AS TABLE
SELECT
    {
        'hits': {
            'min': meta.min_hits,
            'max': meta.max_hits,
        },
        'turns': {
            'min': meta.min_turns,
            'max': meta.max_turns,
        },
        'drain': meta.drain,
        'healing': meta.healing,
        'crit_rate': meta.crit_rate,
        'flinch_chance': meta.flinch_chance,
        'aliment': {
           'id': ailment.id, 
            'identifier': ailment.identifier,
            'name': ailname.name,
        },
        'stats': (
            SELECT
                ARRAY_AGG({
                    'id': stat.id,
                    'identifier': stat.identifier,
                    'change': stat_change.change,
                }) AS data
            FROM move_meta_stat_changes AS stat_change
            LEFT JOIN languages AS lang
                ON lang.identifier = lang_id
            LEFT JOIN stats AS stat
                ON stat.id = stat_change.stat_id
            LEFT JOIN stat_names AS name
                ON name.stat_id = stat.id
                AND name.local_language_id = lang.id
            WHERE
                stat_change.move_id = meta.move_id
        ),
    } AS data
FROM move_meta AS meta
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN move_meta_ailments AS ailment
    ON ailment.id = meta.meta_ailment_id
LEFT JOIN move_meta_ailment_names AS ailname
    ON ailname.move_meta_ailment_id = ailment.id
    AND ailname.local_language_id = lang.id
WHERE
    meta.move_id = move_id
;
