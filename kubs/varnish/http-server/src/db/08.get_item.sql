CREATE OR REPLACE TEMP MACRO get_item(lang_id, id) AS TABLE
SELECT
    {
        'id': item.id,
        'identifier': item.identifier,
        'name': name.name,
        'cost': item.cost,
        'category': (SELECT * FROM get_item_category(lang_id, item.category_id)),
        'fling': {
            'power': item.fling_power,
            'effect': (SELECT * FROM get_item_fling(lang_id, item.fling_effect_id)),
        },
    } AS data
FROM items AS item
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN item_names AS name
    ON name.item_id = item.id
    AND name.local_language_id = lang.id
WHERE
    item.id = id
;
