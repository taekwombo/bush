CREATE OR REPLACE TEMP MACRO get_super_contest_effect(lang_id, id) AS TABLE
SELECT
    {
        'id': effect.id,
        'appeal': effect.appeal,
        'description': name.flavor_text,
    } AS data
FROM super_contest_effects AS effect
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN super_contest_effect_prose AS name
    ON name.super_contest_effect_id = effect.id
    AND name.local_language_id = lang.id
WHERE
    effect.id = id
;
