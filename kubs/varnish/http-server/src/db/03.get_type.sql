CREATE OR REPLACE TEMP MACRO get_type(lang_id, id) AS TABLE
WITH
    atk_strong_against AS (
        SELECT
            struct_insert((SELECT * FROM get_type_base(lang_id, ef.target_type_id)), damage_factor := ef.damage_factor) AS data,
        FROM type_efficacy AS ef
        WHERE
            ef.damage_type_id = id
            AND
            ef.damage_factor > 100
    ),
    atk_weak_against AS (
        SELECT
            struct_insert((SELECT data FROM get_type_base(lang_id, ef.target_type_id)), damage_factor := ef.damage_factor) AS data,
        FROM type_efficacy AS ef,
        WHERE
            ef.damage_type_id = id
            AND
            ef.damage_factor < 100
    ),
    def_strong_against AS (
        SELECT
            struct_insert((SELECT data FROM get_type_base(lang_id, ef.damage_type_id)), damage_factor := ef.damage_factor) AS data,
        FROM type_efficacy AS ef
        WHERE
            ef.target_type_id = id
            AND
            ef.damage_factor < 100
    ),
    def_weak_against AS (
        SELECT
            struct_insert((SELECT data FROM get_type_base(lang_id, ef.damage_type_id)), damage_factor := ef.damage_factor) AS data,
        FROM type_efficacy AS ef
        WHERE
            ef.target_type_id = id
            AND
            ef.damage_factor > 100
    )
SELECT
    STRUCT_INSERT(
        the_type.data,
        defense := {
            'strong': (SELECT COALESCE(ARRAY_AGG(data), []) FROM def_strong_against),
            'weak': (SELECT COALESCE(ARRAY_AGG(data), []) FROM def_weak_against),
        },
        attack := {
            'strong': (SELECT COALESCE(ARRAY_AGG(data), []) FROM atk_strong_against),
            'weak': (SELECT COALESCE(ARRAY_AGG(data), []) FROM atk_weak_against),
        }
    ) AS data
FROM
    get_type_base(lang_id, id) AS the_type
GROUP BY the_type.data
;
