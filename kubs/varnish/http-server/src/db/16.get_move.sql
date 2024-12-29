CREATE OR REPLACE TEMP MACRO get_move(lang_id, id) AS TABLE
SELECT
    {
        'id': move.id,
        'identifier': move.identifier,
        'generation': (SELECT ANY_VALUE(data) FROM get_generation(lang_id, move.generation_id)),
        'type': (SELECT data FROM get_type_base(lang_id, move.type_id)),
        'power': move.power,
        'pp': move.pp,
        'accuracy': move.accuracy,
        'priority': move.priority,
        'target': (SELECT data FROM get_move_target(lang_id, move.target_id)),
        'meta': (SELECT data FROM get_move_meta(lang_id, move.id)),
        'damage_class': (
            SELECT data FROM get_move_damage_class(lang_id, move.damage_class_id)
        ),
        'effect': (
            SELECT STRUCT_INSERT(data, chance := move.effect_chance)
            FROM get_move_effect(lang_id, move.effect_id)
        ),
        'contest': {
            'type': (
                SELECT data FROM get_contest_type(lang_id, move.contest_type_id)
            ),
            'effect': (
                SELECT data FROM get_contest_effect(lang_id, move.contest_effect_id)
            ),
        },
        'super_contest_effect': (
            SELECT data
            FROM get_super_contest_effect(lang_id, move.super_contest_effect_id)
        ),
    } AS data
FROM moves AS move
JOIN languages AS lang ON lang.identifier = lang_id
WHERE move.id = id
;
