CREATE OR REPLACE TEMP MACRO get_item_category(lang_id, id) AS TABLE
SELECT
    {
        'id': cat.id,
        'identifier': cat.identifier,
        'name': prose.name,
        'pocket': (SELECT * FROM get_item_pocket(lang_id, cat.pocket_id)),
    } AS data
FROM item_categories AS cat
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN item_category_prose AS prose
    ON prose.item_category_id = cat.id
    AND prose.local_language_id = lang.id
WHERE
    cat.id = id
;
