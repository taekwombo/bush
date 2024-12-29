CREATE OR REPLACE TEMP MACRO get_contest_type(lang_id, id) AS TABLE
SELECT
    {
        'id': contest.id,
        'identifier': contest.identifier,
        'name': name.name,
        'flavor': name.flavor,
        'color': name.color,
    } AS data
FROM contest_types AS contest
LEFT JOIN languages AS lang
    ON lang.identifier = lang_id
LEFT JOIN contest_type_names AS name
    ON name.contest_type_id = contest.id
    AND name.local_language_id = lang.id
WHERE
    contest.id = id
;
