CREATE OR REPLACE TEMP MACRO get_move_target(lang_id, id) AS TABLE
SELECT
    {
        'id': target.id,
        'identifier': target.identifier,
        'name': name.name,
        'description': name.description,
    } AS data
FROM move_targets AS target
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN move_target_prose AS name
    ON name.move_target_id = target.id
    AND name.local_language_id = lang.id
WHERE
    target.id = id
;
