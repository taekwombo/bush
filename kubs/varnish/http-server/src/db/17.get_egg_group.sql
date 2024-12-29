CREATE OR REPLACE TEMP MACRO get_egg_group(lang_id, id) AS TABLE
SELECT
    {
        'id': egg.id,
        'identifier': egg.identifier,
        'name': prose.name,
    } AS data
FROM egg_groups AS egg
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN egg_group_prose AS prose
    ON prose.egg_group_id = egg.id
    AND prose.local_language_id = lang.id
WHERE
    egg.id = id
;
