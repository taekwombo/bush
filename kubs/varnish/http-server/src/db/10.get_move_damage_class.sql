CREATE OR REPLACE TEMP MACRO get_move_damage_class(lang_id, id) AS TABLE
SELECT
    {
        'id': cl.id,
        'identifier': cl.identifier,
        'name': name.name,
        'description': name.description,
    } AS data
FROM move_damage_classes AS cl
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN move_damage_class_prose AS name
    ON name.move_damage_class_id = cl.id
    AND name.local_language_id = lang.id
WHERE
    cl.id = id
;
