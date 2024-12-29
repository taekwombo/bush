CREATE OR REPLACE TEMP MACRO get_type_base(lang_id, id) AS TABLE
SELECT
    {
        'id': type.id,
        'identifier': type.identifier,
        'name': name.name,
    } AS data
FROM types AS type
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN type_names AS name
    ON name.type_id = type.id
    AND name.local_language_id = lang.id
WHERE
    type.id = id
;

