.mode ascii
.bail on

.read ./src/db/00.get_version.sql
.read ./src/db/01.get_generation.sql
.read ./src/db/02.get_type_base.sql
.read ./src/db/03.get_type.sql
.read ./src/db/04.get_ability.sql
.read ./src/db/05.get_item_pocket.sql
.read ./src/db/06.get_item_fling.sql
.read ./src/db/07.get_item_category.sql
.read ./src/db/08.get_item.sql
.read ./src/db/09.get_move_target.sql
.read ./src/db/10.get_move_damage_class.sql
.read ./src/db/11.get_move_effect.sql
.read ./src/db/12.get_contest_type.sql
.read ./src/db/13.get_contest_effect.sql
.read ./src/db/14.get_super_contest_effect.sql
.read ./src/db/15.get_move_meta.sql
.read ./src/db/16.get_move.sql
.read ./src/db/17.get_egg_group.sql
.read ./src/db/18.get_pokemon_move.sql
.read ./src/db/19.get_pokemon_move_meta.sql
.read ./src/db/20.get_pokemon.sql

SELECT json_pretty((data.items)::JSON) FROM get_pokemon('en', 337)
LIMIT 1
;

