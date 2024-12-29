CREATE OR REPLACE TEMP MACRO get_move_effect(lang_id, id) AS TABLE
SELECT
    {
        'id': effect.id,
        'short_effect': name.short_effect,
        'effect': name.effect,
    } AS data
FROM move_effects AS effect
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN move_effect_prose AS name
    ON name.move_effect_id = effect.id
    AND name.local_language_id = lang.id
WHERE
    effect.id = id
;
