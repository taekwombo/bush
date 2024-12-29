CREATE OR REPLACE TEMP MACRO get_generation(lang_id, id) AS TABLE
SELECT
    {
        'id': gen.id,
        'identifier': gen.identifier,
        'name': gen_name.name,
    } AS data
FROM generations AS gen
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN generation_names AS gen_name
    ON gen_name.generation_id = gen.id
    AND gen_name.local_language_id = lang.id
WHERE
    gen.id = id
;
