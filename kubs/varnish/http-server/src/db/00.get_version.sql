CREATE OR REPLACE TEMP MACRO get_version(lang_id, id) AS TABLE
SELECT
    {
        'id': version.id,
        'name': name.name,
    } AS data
FROM versions AS version
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN version_names AS name
    ON name.version_id = version.id
    AND name.local_language_id = lang.id
WHERE
    version.id = id
;
