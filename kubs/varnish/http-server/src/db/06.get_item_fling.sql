CREATE OR REPLACE TEMP MACRO get_item_fling(lang_id, id) AS TABLE
SELECT
    {
        'id': fling.id,
        'identifier': fling.identifier,
        'effect': prose.effect,
    } AS data
FROM item_fling_effects AS fling
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN item_fling_effect_prose AS prose
    ON prose.item_fling_effect_id = fling.id
    AND prose.local_language_id = lang.id
WHERE
    fling.id = id
;
