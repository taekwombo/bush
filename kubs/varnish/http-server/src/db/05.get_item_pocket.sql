CREATE OR REPLACE TEMP MACRO get_item_pocket(lang_id, id) AS TABLE
SELECT
    {
        'id': pocket.id,
        'identifier': pocket.identifier,
        'name': name.name,
    } AS data
FROM item_pockets AS pocket
JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN item_pocket_names AS name
    ON name.item_pocket_id = pocket.id
    AND name.local_language_id = lang.id
WHERE
    pocket.id = id
;

