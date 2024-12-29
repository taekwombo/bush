CREATE OR REPLACE TEMP MACRO get_contest_effect(lang_id, id) AS TABLE
SELECT
    {
        'id': effect.id,
        'appeal': effect.appeal,
        'jam': effect.jam,
        'flavor': name.flavor_text,
        'effect': name.effect,
    } AS data
FROM contest_effects AS effect
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN contest_effect_prose AS name
    ON name.contest_effect_id = effect.id
    AND name.local_language_id = lang.id
WHERE
    effect.id = id
;
