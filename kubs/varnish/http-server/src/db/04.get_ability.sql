CREATE OR REPLACE TEMP MACRO get_ability(lang_id, id) AS TABLE
SELECT
    {
        'id': ability.id,
        'identifier': ability.identifier,
        'name': name.name,
        'flavor_text': (
            SELECT flavor_text FROM ability_flavor_text AS flav
            WHERE
                flav.ability_id = ability.id
                AND flav.language_id = lang.id
            ORDER BY flav.version_group_id
            LIMIT 1
        ),
    } AS data
FROM abilities AS ability
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN ability_names AS name
    ON name.ability_id = ability.id
    AND name.local_language_id = lang.id
WHERE
    ability.id = id
;
